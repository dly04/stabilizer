use super::hal;
use crate::hardware::{setup, I2c1Proxy, shared_adc::AdcChannel};
use crate::telemetry::PounderTelemetry;
use ad9959::{Address, frequency_to_ftw, phase_to_pow, amplitude_to_acr, validate_clocking};
use embedded_hal_02::blocking::spi::Transfer;
use miniconf::{Leaf, Tree};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use stm32h7xx_hal::time::MegaHertz;

pub mod dds_output;
pub mod hrtimer;

#[cfg(not(feature = "pounder_v1_0"))]
pub mod timestamp;

#[derive(Debug, Copy, Clone, strum::EnumIter)]
pub enum GpioPin {
    Led4Green,
    Led5Red,
    Led6Green,
    Led7Red,
    Led8Green,
    Led9Red,
    DetPwrdown0,
    DetPwrdown1,
    AttLe0,
    AttLe1,
    AttLe2,
    AttLe3,
    DdsReset,
    AttRstN,
    OscEnN,
    ExtClkSel,
}

impl From<GpioPin> for mcp230xx::Mcp23017 {
    fn from(x: GpioPin) -> Self {
        match x {
            GpioPin::Led4Green => Self::A0,
            GpioPin::Led5Red => Self::A1,
            GpioPin::Led6Green => Self::A2,
            GpioPin::Led7Red => Self::A3,
            GpioPin::Led8Green => Self::A4,
            GpioPin::Led9Red => Self::A5,
            GpioPin::DetPwrdown0 => Self::A6,
            GpioPin::DetPwrdown1 => Self::A7,
            GpioPin::AttLe0 => Self::B0,
            GpioPin::AttLe1 => Self::B1,
            GpioPin::AttLe2 => Self::B2,
            GpioPin::AttLe3 => Self::B3,
            GpioPin::DdsReset => Self::B4,
            GpioPin::AttRstN => Self::B5,
            GpioPin::OscEnN => Self::B6,
            GpioPin::ExtClkSel => Self::B7,
        }
    }
}

impl From<GpioPin> for tca9539::Pin {
    fn from(x: GpioPin) -> Self {
        match x {
            GpioPin::Led4Green => Self::P00,
            GpioPin::Led5Red => Self::P01,
            GpioPin::Led6Green => Self::P02,
            GpioPin::Led7Red => Self::P03,
            GpioPin::Led8Green => Self::P04,
            GpioPin::Led9Red => Self::P05,
            GpioPin::DetPwrdown0 => Self::P06,
            GpioPin::DetPwrdown1 => Self::P07,
            GpioPin::AttLe0 => Self::P10,
            GpioPin::AttLe1 => Self::P11,
            GpioPin::AttLe2 => Self::P12,
            GpioPin::AttLe3 => Self::P13,
            GpioPin::DdsReset => Self::P14,
            GpioPin::AttRstN => Self::P15,
            GpioPin::OscEnN => Self::P16,
            GpioPin::ExtClkSel => Self::P17,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
    Spi,
    I2c,
    Qspi(hal::xspi::QspiError),
    Bounds,
    InvalidAddress,
    InvalidChannel,
    Adc,
    InvalidState,
}

impl From<hal::xspi::QspiError> for Error {
    fn from(e: hal::xspi::QspiError) -> Error {
        Error::Qspi(e)
    }
}

/// The numerical value (discriminant) of the Channel enum is the index in the attenuator shift
/// register as well as the attenuator latch enable signal index on the GPIO extender.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum Channel {
    In0 = 0,
    Out0 = 1,
    In1 = 2,
    Out1 = 3,
}

impl From<Channel> for GpioPin {
    fn from(x: Channel) -> Self {
        match x {
            Channel::In0 => GpioPin::AttLe0,
            Channel::Out0 => GpioPin::AttLe1,
            Channel::In1 => GpioPin::AttLe2,
            Channel::Out1 => GpioPin::AttLe3,
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Tree)]
pub struct DdsChannelConfig {
    pub frequency: Leaf<f32>,
    pub phase_offset: Leaf<f32>,
    pub amplitude: Leaf<f32>,
}

impl Default for DdsChannelConfig {
    fn default() -> Self {
        Self {
            frequency: Leaf(0.0),
            phase_offset: Leaf(0.0),
            amplitude: Leaf(0.0),
        }
    }
}

/// Represents a fully defined DDS profile, with parameters expressed in machine units
pub struct Profile {
    /// A 32-bits representation of DDS frequency in relation to the system clock frequency.
    /// This value corresponds to the AD9959 CFTW0 register, which specifies the frequency
    /// of DDS channels.
    pub frequency_tuning_word: u32,
    /// The DDS phase offset. It corresponds to the AD9959 CPOW0 register, which specifies
    /// the phase offset of DDS channels.
    pub phase_offset: u16,
    /// Control amplitudes of DDS channels. It corresponds to the AD9959 ACR register, which
    /// controls the amplitude scaling factor of DDS channels.
    pub amplitude_control: u32,
}

impl TryFrom<(ClockConfig, ChannelConfig)> for Profile {
    type Error = ad9959::Error;

    fn try_from(
        (clocking, channel): (ClockConfig, ChannelConfig),
    ) -> Result<Self, Self::Error> {
        let system_clock_frequency =
            *clocking.reference_clock * *clocking.multiplier as f32;
        Ok(Profile {
            frequency_tuning_word: frequency_to_ftw(
                *channel.dds.frequency,
                system_clock_frequency,
            )?,
            phase_offset: phase_to_pow(*channel.dds.phase_offset)?,
            amplitude_control: amplitude_to_acr(*channel.dds.amplitude)?,
        })
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Tree)]
pub struct ChannelConfig {
    #[tree]
    pub dds: DdsChannelConfig,
    pub attenuation: Leaf<f32>,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        ChannelConfig {
            dds: DdsChannelConfig::default(),
            attenuation: Leaf(31.5),
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Tree)]
pub struct ClockConfig {
    pub multiplier: Leaf<u8>,
    pub reference_clock: Leaf<f32>,
    pub external_clock: Leaf<bool>,
}

impl Default for ClockConfig {
    fn default() -> Self {
        Self {
            multiplier: Leaf(5),
            reference_clock: Leaf(MegaHertz::MHz(100).to_Hz() as f32),
            external_clock: Leaf(false),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, Tree)]
pub struct PounderConfig {
    #[tree]
    pub clock: ClockConfig,
    #[tree]
    pub in_channel: [ChannelConfig; 2],
    #[tree]
    pub out_channel: [ChannelConfig; 2],
}

impl From<Channel> for ad9959::Channel {
    /// Translate pounder channels to DDS output channels.
    fn from(other: Channel) -> Self {
        Self::new(
            1 << match other {
                Channel::In0 => 1,
                Channel::In1 => 3,
                Channel::Out0 => 0,
                Channel::Out1 => 2,
            },
        )
    }
}

/// A structure for the QSPI interface for the DDS.
pub struct QspiInterface {
    pub qspi: hal::xspi::Qspi<hal::stm32::QUADSPI>,
    mode: ad9959::Mode,
    streaming: bool,
}

impl QspiInterface {
    /// Initialize the QSPI interface.
    ///
    /// Args:
    /// * `qspi` - The QSPI peripheral driver.
    pub fn new(
        mut qspi: hal::xspi::Qspi<hal::stm32::QUADSPI>,
    ) -> Result<Self, Error> {
        // This driver only supports operation in 4-bit mode due to bus inconsistencies between the
        // QSPI peripheral and the DDS. Instead, we will bit-bang communications in
        // single-bit-two-wire to the DDS to configure it to 4-bit operation.
        qspi.configure_mode(hal::xspi::QspiMode::FourBit)?;
        Ok(Self {
            qspi,
            mode: ad9959::Mode::SingleBitTwoWire,
            streaming: false,
        })
    }

    pub fn start_stream(&mut self) -> Result<(), Error> {
        self.qspi.is_busy()?;

        // Configure QSPI for infinite transaction mode using only a data phase (no instruction or
        // address).
        let qspi_regs = unsafe { &*hal::stm32::QUADSPI::ptr() };
        qspi_regs.fcr.modify(|_, w| w.ctcf().set_bit());

        unsafe {
            qspi_regs.dlr.write(|w| w.dl().bits(0xFFFF_FFFF));
            qspi_regs.ccr.modify(|_, w| {
                w.imode().bits(0).fmode().bits(0).admode().bits(0)
            });
        }

        self.streaming = true;

        Ok(())
    }
}

impl ad9959::Interface for QspiInterface {
    type Error = Error;

    /// Configure the operations mode of the interface.
    ///
    /// Args:
    /// * `mode` - The newly desired operational mode.
    fn configure_mode(&mut self, mode: ad9959::Mode) -> Result<(), Error> {
        self.mode = mode;

        Ok(())
    }

    /// Write data over QSPI to the DDS.
    ///
    /// Args:
    /// * `addr` - The address to write over QSPI to the DDS.
    /// * `data` - The data to write.
    fn write(&mut self, addr: Address, data: &[u8]) -> Result<(), Error> {
        let addr = addr.raw_value().value();

        // The QSPI interface implementation always operates in 4-bit mode because the AD9959 uses
        // IO3 as SYNC_IO in some output modes. In order for writes to be successful, SYNC_IO must
        // be driven low. However, the QSPI peripheral forces IO3 high when operating in 1 or 2 bit
        // modes. As a result, any writes while in single- or dual-bit modes has to instead write
        // the data encoded into 4-bit QSPI data so that IO3 can be driven low.
        match self.mode {
            ad9959::Mode::SingleBitTwoWire => {
                // Encode the data into a 4-bit QSPI pattern.

                // In 4-bit mode, we can send 2 bits of address and data per byte transfer. As
                // such, we need at least 4x more bytes than the length of data. To avoid dynamic
                // allocation, we assume the maximum transaction length for single-bit-two-wire is
                // 2 bytes.
                let mut encoded_data: [u8; 12] = [0; 12];

                if (data.len() * 4) > (encoded_data.len() - 4) {
                    return Err(Error::Bounds);
                }

                // Encode the address into the first 4 bytes.
                for address_bit in 0..8 {
                    let offset: u8 =
                        { if address_bit % 2 != 0 { 4 } else { 0 } };

                    // Encode MSB first. Least significant bits are placed at the most significant
                    // byte.
                    let byte_position = 3 - (address_bit >> 1) as usize;

                    if addr & (1 << address_bit) != 0 {
                        encoded_data[byte_position] |= 1 << offset;
                    }
                }

                // Encode the data into the remaining bytes.
                for byte_index in 0..data.len() {
                    let byte = data[byte_index];
                    for bit in 0..8 {
                        let offset: u8 = { if bit % 2 != 0 { 4 } else { 0 } };

                        // Encode MSB first. Least significant bits are placed at the most
                        // significant byte.
                        let byte_position = 3 - (bit >> 1) as usize;

                        if byte & (1 << bit) != 0 {
                            encoded_data
                                [(byte_index + 1) * 4 + byte_position] |=
                                1 << offset;
                        }
                    }
                }

                let (encoded_address, encoded_payload) = {
                    let end_index = (1 + data.len()) * 4;
                    (encoded_data[0], &encoded_data[1..end_index])
                };

                self.qspi.write(encoded_address, encoded_payload)?;

                Ok(())
            }
            ad9959::Mode::FourBitSerial => {
                if self.streaming {
                    Err(Error::InvalidState)
                } else {
                    self.qspi.write(addr, data)?;
                    Ok(())
                }
            }
            _ => Err(Error::InvalidState),
        }
    }

    fn read(&mut self, addr: Address, dest: &mut [u8]) -> Result<(), Error> {
        let addr = addr.raw_value().value();

        // This implementation only supports operation (read) in four-bit-serial mode.
        if self.mode != ad9959::Mode::FourBitSerial {
            return Err(Error::InvalidState);
        }

        self.qspi.read(0x80 | addr, dest)?;

        Ok(())
    }
}

enum IoExpander {
    Mcp(mcp230xx::Mcp230xx<I2c1Proxy, mcp230xx::Mcp23017>),
    Pca(tca9539::Pca9539<I2c1Proxy>),
}

impl IoExpander {
    fn new(i2c: I2c1Proxy) -> Self {
        // Population option on Pounder v1.2 and later.
        let mut mcp23017 =
            mcp230xx::Mcp230xx::new_default(i2c.clone()).unwrap();
        if mcp23017.read(0).is_ok() {
            Self::Mcp(mcp23017)
        } else {
            let pca9359 = tca9539::Pca9539::new_default(i2c).unwrap();
            Self::Pca(pca9359)
        }
    }

    /// Set the state (its electrical level) of the given GPIO pin on Pounder.
    fn set_gpio_dir(
        &mut self,
        pin: GpioPin,
        dir: mcp230xx::Direction,
    ) -> Result<(), Error> {
        match self {
            Self::Mcp(dev) => {
                dev.set_direction(pin.into(), dir).map_err(|_| Error::I2c)
            }
            Self::Pca(dev) => {
                let dir = match dir {
                    mcp230xx::Direction::Output => tca9539::Direction::Output,
                    _ => tca9539::Direction::Input,
                };
                dev.set_direction(pin.into(), dir).map_err(|_| Error::I2c)
            }
        }
    }

    /// Set the state (its electrical level) of the given GPIO pin on Pounder.
    fn set_gpio_level(
        &mut self,
        pin: GpioPin,
        level: mcp230xx::Level,
    ) -> Result<(), Error> {
        match self {
            Self::Mcp(dev) => {
                dev.set_gpio(pin.into(), level).map_err(|_| Error::I2c)
            }
            Self::Pca(dev) => {
                let level = match level {
                    mcp230xx::Level::Low => tca9539::Level::Low,
                    _ => tca9539::Level::High,
                };
                dev.set_level(pin.into(), level).map_err(|_| Error::I2c)
            }
        }
    }
}

/// A structure containing implementation for Pounder hardware.
pub struct PounderDevices {
    io: IoExpander,
    lm75: lm75::Lm75<I2c1Proxy, lm75::ic::Lm75>,
    attenuator_spi: hal::spi::Spi<hal::stm32::SPI1, hal::spi::Enabled, u8>,
    pwr: (
        AdcChannel<
            'static,
            hal::stm32::ADC1,
            hal::gpio::gpiof::PF11<hal::gpio::Analog>,
        >,
        AdcChannel<
            'static,
            hal::stm32::ADC2,
            hal::gpio::gpiof::PF14<hal::gpio::Analog>,
        >,
    ),
    aux_adc: (
        AdcChannel<
            'static,
            hal::stm32::ADC3,
            hal::gpio::gpiof::PF3<hal::gpio::Analog>,
        >,
        AdcChannel<
            'static,
            hal::stm32::ADC3,
            hal::gpio::gpiof::PF4<hal::gpio::Analog>,
        >,
    ),
}

impl PounderDevices {
    /// Construct and initialize pounder-specific hardware.
    ///
    /// Args:
    /// * `i2c` - A Proxy to I2C1.
    /// * `attenuator_spi` - A SPI interface to control digital attenuators.
    /// * `pwr` - The ADC channels to measure the IN0/1 input power.
    /// * `aux_adc` - The ADC channels to measure the ADC0/1 auxiliary input.
    pub fn new(
        i2c: I2c1Proxy,
        attenuator_spi: hal::spi::Spi<hal::stm32::SPI1, hal::spi::Enabled, u8>,
        pwr: (
            AdcChannel<
                'static,
                hal::stm32::ADC1,
                hal::gpio::gpiof::PF11<hal::gpio::Analog>,
            >,
            AdcChannel<
                'static,
                hal::stm32::ADC2,
                hal::gpio::gpiof::PF14<hal::gpio::Analog>,
            >,
        ),
        aux_adc: (
            AdcChannel<
                'static,
                hal::stm32::ADC3,
                hal::gpio::gpiof::PF3<hal::gpio::Analog>,
            >,
            AdcChannel<
                'static,
                hal::stm32::ADC3,
                hal::gpio::gpiof::PF4<hal::gpio::Analog>,
            >,
        ),
    ) -> Result<Self, Error> {
        let mut devices = Self {
            lm75: lm75::Lm75::new(i2c.clone(), lm75::Address::default()),
            io: IoExpander::new(i2c.clone()),
            attenuator_spi,
            pwr,
            aux_adc,
        };

        // Configure power-on-default state for pounder. All LEDs are off, on-board oscillator
        // selected and enabled, attenuators out of reset. Note that testing indicates the
        // output state needs to be set first to properly update the output registers.
        for pin in GpioPin::iter() {
            devices.io.set_gpio_level(pin, mcp230xx::Level::Low)?;
            devices.io.set_gpio_dir(pin, mcp230xx::Direction::Output)?;
        }

        devices.reset_attenuators().unwrap();

        devices.reset_dds().unwrap();

        Ok(devices)
    }

    /// Sample one of the two auxiliary ADC channels associated with the respective RF input channel.
    pub fn sample_aux_adc(&mut self, channel: Channel) -> Result<f32, Error> {
        let adc_scale = match channel {
            Channel::In0 => self.aux_adc.0.read_normalized().unwrap(),
            Channel::In1 => self.aux_adc.1.read_normalized().unwrap(),
            _ => return Err(Error::InvalidChannel),
        };

        // Convert analog percentage to voltage. Note that the ADC uses an external 2.048V analog
        // reference.
        Ok(adc_scale * 2.048)
    }

    /// Select external reference clock input.
    pub fn set_ext_clk(&mut self, enabled: bool) -> Result<(), Error> {
        let level = if enabled {
            mcp230xx::Level::High
        } else {
            mcp230xx::Level::Low
        };
        // Active low
        self.io.set_gpio_level(GpioPin::OscEnN, level)?;
        self.io.set_gpio_level(GpioPin::ExtClkSel, level)
    }

    /// Reset the DDS via the GPIO extender (Pounder v1.2 and later)
    pub fn reset_dds(&mut self) -> Result<(), Error> {
        // DDS reset (Pounder v1.2 or later)
        self.io
            .set_gpio_level(GpioPin::DdsReset, mcp230xx::Level::High)?;
        // I2C duration of this transaction is long enough (> 5 µs) to ensure valid reset.
        self.io
            .set_gpio_level(GpioPin::DdsReset, mcp230xx::Level::Low)
    }

    /// Read the temperature reported by the LM75 temperature sensor on Pounder in deg C.
    pub fn temperature(&mut self) -> Result<f32, Error> {
        self.lm75.read_temperature().map_err(|_| Error::I2c)
    }
}

impl PounderDevices {
    /// Reset all of the attenuators to a power-on default state.
    fn reset_attenuators(&mut self) -> Result<(), Error> {
        // Active low
        self.io
            .set_gpio_level(GpioPin::AttRstN, mcp230xx::Level::Low)?;
        self.io
            .set_gpio_level(GpioPin::AttRstN, mcp230xx::Level::High)
    }

    /// Latch a configuration into a digital attenuator.
    ///
    /// Args:
    /// * `channel` - The attenuator channel to latch.
    fn latch_attenuator(&mut self, channel: Channel) -> Result<(), Error> {
        // Rising edge sensitive
        // Be robust against initial state: drive low, then high (contrary to the datasheet figure).
        self.io
            .set_gpio_level(channel.into(), mcp230xx::Level::Low)?;
        self.io
            .set_gpio_level(channel.into(), mcp230xx::Level::High)
    }

    /// Read the raw attenuation codes stored in the attenuator shift registers.
    ///
    /// Args:
    /// * `channels` - A 4 byte slice to be shifted into the
    ///   attenuators and to contain the data shifted out.
    fn transfer_attenuators(
        &mut self,
        channels: &mut [u8; 4],
    ) -> Result<(), Error> {
        self.attenuator_spi
            .transfer(channels)
            .map_err(|_| Error::Spi)?;

        Ok(())
    }

    /// Set the attenuation of a single channel.
    ///
    /// Args:
    /// * `channel` - The pounder channel to configure the attenuation of.
    /// * `attenuation` - The desired attenuation of the channel in dB. This has a resolution of
    ///   0.5dB.
    pub fn set_attenuation(
        &mut self,
        channel: Channel,
        attenuation: f32,
    ) -> Result<f32, Error> {
        if !crate::convert::att_is_valid(attenuation) {
            return Err(Error::Bounds);
        }

        // Calculate the attenuation code to program into the attenuator. The attenuator uses a
        // code where the LSB is 0.5 dB.
        let attenuation_code = (attenuation * 2.0) as u8;

        // Read all the channels, modify the channel of interest, and write all the channels back.
        // This ensures the staging register and the output register are always in sync.
        let mut channels = [0_u8; 4];
        self.transfer_attenuators(&mut channels)?;

        // The lowest 2 bits of the 8-bit shift register on the attenuator are ignored. Shift the
        // attenuator code into the upper 6 bits of the register value. Note that the attenuator
        // treats inputs as active-low, so the code is inverted before writing.
        channels[channel as usize] = !(attenuation_code << 2);
        self.transfer_attenuators(&mut channels)?;

        // Finally, latch the output of the updated channel to force it into an active state.
        self.latch_attenuator(channel)?;

        Ok(attenuation_code as f32 / 2.0)
    }

    /// Get the attenuation of a channel.
    ///
    /// Args:
    /// * `channel` - The channel to get the attenuation of.
    ///
    /// Returns:
    /// The programmed attenuation of the channel in dB.
    pub fn get_attenuation(&mut self, channel: Channel) -> Result<f32, Error> {
        let mut channels = [0_u8; 4];

        // Reading the data always shifts data out of the staging registers, so we perform a
        // duplicate write-back to ensure the staging register is always equal to the output
        // register.
        self.transfer_attenuators(&mut channels)?;
        self.transfer_attenuators(&mut channels)?;

        // The attenuation code is stored in the upper 6 bits of the register, where each LSB
        // represents 0.5 dB. The attenuator stores the code as active-low, so inverting the result
        // (before the shift) has the affect of transforming the bits of interest (and the
        // dont-care bits) into an active-high state and then masking off the don't care bits. If
        // the shift occurs before the inversion, the upper 2 bits (which would then be don't
        // care) would contain erroneous data.
        let attenuation_code = (!channels[channel as usize]) >> 2;

        // Convert the desired channel code into dB of attenuation.
        Ok(attenuation_code as f32 / 2.0)
    }
}

impl PounderDevices {
    /// Sample an ADC channel.
    ///
    /// Args:
    /// * `channel` - The channel to sample.
    ///
    /// Returns:
    /// The sampled voltage of the specified channel.
    fn sample_converter(&mut self, channel: Channel) -> Result<f32, Error> {
        let adc_scale = match channel {
            Channel::In0 => self.pwr.0.read_normalized().unwrap(),
            Channel::In1 => self.pwr.1.read_normalized().unwrap(),
            _ => return Err(Error::InvalidChannel),
        };

        // Convert analog percentage to voltage. Note that the ADC uses an external 2.048V analog
        // reference.
        Ok(adc_scale * 2.048)
    }

    /// Measure the power of an input channel in dBm.
    ///
    /// Args:
    /// * `channel` - The pounder input channel to measure the power of.
    ///
    /// Returns:
    /// Power in dBm after the digitally controlled attenuator before the amplifier.
    pub fn measure_power(&mut self, channel: Channel) -> Result<f32, Error> {
        let analog_measurement = self.sample_converter(channel)?;

        // The AD8363 with VSET connected to VOUT provides an output voltage of 51.7 mV/dB at
        // 100MHz with an intercept of -58 dBm.
        // It is placed behind a 20 dB tap.
        Ok(analog_measurement * (1. / 0.0517) + (-58. + 20.))
    }
}

impl setup::Pounder {
    pub fn update_dds(
        &mut self,
        settings: PounderConfig,
        clocking: &mut ClockConfig,
    ) {
        if *clocking != settings.clock {
            match validate_clocking(
                *settings.clock.reference_clock,
                *settings.clock.multiplier,
            ) {
                Ok(_frequency) => {
                    self.pounder
                        .set_ext_clk(*settings.clock.external_clock)
                        .unwrap();

                    // skip since ad9959 should already set up the clock
                    // self.dds_output
                    //     .builder()
                    //     .set_system_clock(
                    //         *settings.clock.reference_clock,
                    //         *settings.clock.multiplier,
                    //     )
                    //     .unwrap()
                    //     .write();

                    *clocking = settings.clock;
                }
                Err(err) => {
                    log::error!("Invalid AD9959 clocking parameters: {:?}", err)
                }
            }
        }

        for (channel_config, pounder_channel) in settings
            .in_channel
            .iter()
            .chain(settings.out_channel.iter())
            .zip([Channel::In0, Channel::In1, Channel::Out0, Channel::Out1])
        {
            match Profile::try_from((*clocking, *channel_config)) {
                Ok(dds_profile) => {
                    self.dds_output
                        .builder()
                        .update_channels_with_profile(
                            pounder_channel.into(),
                            dds_profile,
                        )
                        .write();

                    if let Err(err) = self.pounder.set_attenuation(
                        pounder_channel,
                        *channel_config.attenuation,
                    ) {
                        log::error!("Invalid attenuation settings: {:?}", err)
                    }
                }
                Err(err) => {
                    log::error!("Invalid AD9959 profile settings: {:?}", err)
                }
            }
        }
    }

    pub fn get_telemetry(&mut self, config: PounderConfig) -> PounderTelemetry {
        PounderTelemetry {
            temperature: self.pounder.lm75.read_temperature().unwrap(),
            input_power: [
                self.pounder.measure_power(Channel::In0).unwrap(),
                self.pounder.measure_power(Channel::In1).unwrap(),
            ],
            config,
        }
    }
}