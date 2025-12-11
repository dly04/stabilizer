use miniconf::Tree;

use idsp::iir;

use core::sync::atomic::{AtomicU32, Ordering};

use platform::{AppSettings, NetSettings};
use serde::{Deserialize, Serialize};
use signal_generator::{self, Source};
use crate::{
    convert::{AdcCode, DacCode, Gain},
    hardware:: {
        pounder::{ClockConfig, PounderConfig},
        setup::{Mezzanine, Pounder},
    }
};

use core::fmt;

// The number of cascaded IIR biquads per channel. Select 1 or 2!
pub const IIR_CASCADE_LENGTH: usize = 1;

// The number of samples in each batch process
pub const BATCH_SIZE: usize = 8;

// The logarithm of the number of 100MHz timer ticks between each sample. With a value of 2^7 =
// 128, there is 1.28uS per sample, corresponding to a sampling frequency of 781.25 KHz.
pub const SAMPLE_TICKS_LOG2: u8 = 7;
pub const SAMPLE_TICKS: u32 = 1 << SAMPLE_TICKS_LOG2;
pub const SAMPLE_PERIOD: f32 =
    SAMPLE_TICKS as f32 * crate::design_parameters::TIMER_PERIOD;


#[derive(Clone, Debug, Tree, Default)]
#[tree(meta(doc, typename))]
pub struct Settings {
    pub dual_iir: DualIir,
    pub net: NetSettings,
}

impl AppSettings for Settings {
    fn new(net: NetSettings) -> Self {
        Self {
            net,
            dual_iir: DualIir::default(),
        }
    }

    fn net(&self) -> &NetSettings {
        &self.net
    }
}

impl serial_settings::Settings for Settings {
    fn reset(&mut self) {
        *self = Self {
            dual_iir: DualIir::default(),
            net: NetSettings::new(self.net.mac),
        }
    }
}

#[derive(Clone, Debug, Tree)]
#[tree(meta(doc, typename = "BiquadReprTree"))]
pub struct BiquadRepr {
    /// Biquad parameters
    #[tree(rename="typ", typ="&str", with=miniconf::str_leaf, defer=self.repr)]
    pub _typ: (),
    pub repr: iir::BiquadRepr<f32, f32>,
}

impl Default for BiquadRepr {
    fn default() -> Self {
        let mut i = iir::Biquad::IDENTITY;
        i.set_min(-i16::MAX as _);
        i.set_max(i16::MAX as _);
        Self {
            _typ: (),
            repr: iir::BiquadRepr::Raw(i),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub enum Run {
    #[default]
    /// Run
    Run,
    /// Hold
    Hold,
    /// Hold controlled by corresponding digital input
    External,
}

impl Run {
    pub fn run(&self, di: bool) -> bool {
        match self {
            Self::Run => true,
            Self::Hold => false,
            Self::External => di,
        }
    }
}

/// A ADC-DAC channel
#[derive(Clone, Debug, Tree, Default)]
#[tree(meta(doc, typename))]
pub struct Channel {
    /// Analog Front End (AFE) gain.
    #[tree(with=miniconf::leaf)]
    pub gain: Gain,
    /// Biquad
    pub biquad: [BiquadRepr; IIR_CASCADE_LENGTH],
    /// Run/Hold behavior
    #[tree(with=miniconf::leaf)]
    pub run: Run,
    /// Signal generator configuration to add to the DAC0/DAC1 outputs
    pub source: signal_generator::Config,
}

impl Channel {
    pub fn build(&self) -> Result<Active, signal_generator::Error> {
        Ok(Active {
            source: self
                .source
                .build(SAMPLE_PERIOD, DacCode::FULL_SCALE.recip())
                .unwrap(),
            state: Default::default(),
            run: self.run,
            biquad: self.biquad.each_ref().map(|biquad| {
                biquad.repr.build::<f32>(
                    SAMPLE_PERIOD,
                    1.0,
                    DacCode::LSB_PER_VOLT,
                )
            }),
        })
    }
}

#[derive(Clone, Tree)]
#[tree(meta(doc, typename))]
pub struct DualIir {
    /// Channel configuration
    pub ch: [Channel; 2],
    /// Trigger both signal sources
    #[tree(with=miniconf::leaf)]
    pub trigger: bool,
    /// Telemetry output period in seconds.
    #[tree(with=miniconf::leaf)]
    pub telemetry_period: f32,
    /// Target IP and port for UDP streaming.
    ///
    /// Can be multicast.
    #[tree(with=miniconf::leaf)]
    pub stream: stream::Target,
    /// Specifies the config for pounder DDS clock configuration, DDS channels & attenuations
    ///
    /// # Path
    /// `pounder`
    ///
    /// # Value
    /// See [PounderConfig#miniconf]
    #[tree]
    pub pounder: Option<PounderConfig>,
}

impl fmt::Debug for DualIir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DualIir")
            // .field("ch", &self.ch)
            // .field("trigger", &self.trigger)
            // .field("telemetry_period", &self.telemetry_period)
            .field("stream", &self.stream)
            .field("pounder", &self.pounder)
            .finish()
    }
}

impl Default for DualIir {
    fn default() -> Self {
        Self {
            telemetry_period: 10.0,
            trigger: false,
            stream: Default::default(),
            ch: Default::default(),
            pounder: None.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Active {
    pub run: Run,
    pub biquad: [iir::Biquad<f32>; IIR_CASCADE_LENGTH],
    pub state: [[f32; 4]; IIR_CASCADE_LENGTH],
    pub source: Source,
}

