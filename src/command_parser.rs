/*
    DualIir (dual-iir::DualIir)
    ├── ch[0] (dual-iir::Channel)
    │   ├── gain: convert::Gain (enum)
    │   ├── biquad[0]
    │   │   ├── typ: idsp::iir::BiquadRepr (enum)
    │   │   └── repr (idsp::iir::BiquadRepr) (enum)
    |   |       ├── Ba (idsp::iir::repr::Ba)
    │   │       │   ├── ba: [T[3]]2
    │   │       │   ├── u: T
    │   │       │   ├── min: T
    │   │       │   └── max: T
    │   │       ├── Raw (idsp::iir::biquad::Biquad)
    │   │       │   ├── ba: T[5]
    │   │       │   ├── u: T
    │   │       │   ├── min: T
    │   │       │   └── max: T
    │   │       ├── Pid (idsp::iir::Pid)
    │   │       │   ├── order: idsp::iir::pid::Order (enum)
    │   │       │   ├── gain
    |   |       |   |   ├── i2: T
    │   │       │   |   ├── i: T
    │   │       │   |   ├── p: T
    │   │       │   |   ├── d: T
    │   │       │   |   └── d2: T
    │   │       │   ├── limit
    |   |       |   |   ├── i2: T
    │   │       │   |   ├── i: T
    │   │       │   │   ├── p: T
    │   │       │   |   ├── d: T
    │   │       │   |   └── d2: T
    │   │       │   ├── setpoint: T
    │   │       │   ├── min: T
    │   │       │   └── max: T
    │   │       └── Filter (idsp::iir::repr::FilterRepr)
    │   │           ├── typ: idsp::iir::repr::Typ (enum)
    │   │           ├── frequency: T
    │   │           ├── gain: T
    │   │           ├── shelf: T
    │   │           ├── shape (idsp::iir::coefficients::Shape) (enum)
    │   │           │   ├── Q: T
    │   │           │   ├── Bandwidth: T
    │   │           │   └── Slope: T
    │   │           ├── offset: T
    │   │           ├── min: T
    │   │           └── max:T
    │   ├── run: dual-iir::Run (enum)
    │   └── source (signal_generator::Config)
    │       ├── signal: signal_generator::Signal (enum)
    │       ├── frequency: f32
    │       ├── symmetry: f32
    │       ├── amplitude: f32
    │       ├── offset: f32
    │       ├── phase: f32
    │       ├── length: u32
    │       ├── state: i64
    │       └── rate: i32
    ├── ch[1] (dual-iir::Channel)
    │   ├── gain: convert::Gain (enum)
    │   ├── biquad[0]
    │   │   ├── typ: idsp::iir::BiquadRepr (enum)
    │   │   └── repr (idsp::iir::BiquadRepr) (enum)
    |   |       ├── Ba (idsp::iir::repr::Ba)
    │   │       │   ├── ba: [T[3]]2
    │   │       │   ├── u: T
    │   │       │   ├── min: T
    │   │       │   └── max: T
    │   │       ├── Raw (idsp::iir::biquad::Biquad)
    │   │       │   ├── ba: T[5]
    │   │       │   ├── u: T
    │   │       │   ├── min: T
    │   │       │   └── max: T
    │   │       ├── Pid (idsp::iir::Pid)
    │   │       │   ├── order: idsp::iir::pid::Order (enum)
    │   │       │   ├── gain
    |   |       |   |   ├── i2: T
    │   │       │   |   ├── i: T
    │   │       │   |   ├── p: T
    │   │       │   |   ├── d: T
    │   │       │   |   └── d2: T
    │   │       │   ├── limit
    |   |       |   |   ├── i2: T
    │   │       │   |   ├── i: T
    │   │       │   │   ├── p: T
    │   │       │   |   ├── d: T
    │   │       │   |   └── d2: T
    │   │       │   ├── setpoint: T
    │   │       │   ├── min: T
    │   │       │   └── max: T
    │   │       └── Filter (idsp::iir::repr::FilterRepr)
    │   │           ├── typ: idsp::iir::repr::Typ (enum)
    │   │           ├── frequency: T
    │   │           ├── gain: T
    │   │           ├── shelf: T
    │   │           ├── shape (idsp::iir::coefficients::Shape) (enum)
    │   │           │   ├── Q: T
    │   │           │   ├── Bandwidth: T
    │   │           │   └── Slope: T
    │   │           ├── offset: T
    │   │           ├── min: T
    │   │           └── max:T
    │   ├── run: dual-iir::Run (enum)
    │   └── source (signal_generator::Config)
    │       ├── signal: signal_generator::Signal (enum)
    │       ├── frequency: f32
    │       ├── symmetry: f32
    │       ├── amplitude: f32
    │       ├── offset: f32
    │       ├── phase: f32
    │       ├── length: u32
    │       ├── state: i64
    │       └── rate: i32
    ├── trigger: bool
    ├── telemetry_period: f32
    ├── stream (stream::Target)
    │   └── : core::net::SocketAddr
    └── pounder (Option<hardware::pounder::PounderConfig>)
        ├── clock (hardware::pounder::ClockConfig)
        │   ├── multiplier: u8
        │   ├── reference_clock: f32
        │   └── external_clock: bool
        ├── in_channel[0]
        │   ├── dds (hardware::pounder::DdsChannelConfig)
        │   |   ├── frequency: f32
        │   |   ├── phase_offset: f32
        │   |   └── amplitude: f32
        │   └── attenuation: f32
        ├── in_channel[1]
        │   ├── dds (hardware::pounder::DdsChannelConfig)
        │   |   ├── frequency: f32
        │   |   ├── phase_offset: f32
        │   |   └── amplitude: f32
        │   └── attenuation: f32
        ├── out_channel[0]
        │   ├── dds (hardware::pounder::DdsChannelConfig)
        │   |   ├── frequency: f32
        │   |   ├── phase_offset: f32
        │   |   └── amplitude: f32
        │   └── attenuation: f32
        └── out_channel[1]
            ├── dds (hardware::pounder::DdsChannelConfig)
            |   ├── frequency: f32
            |   ├── phase_offset: f32
            |   └── amplitude: f32
            └── attenuation: f32
*/

/*
    gain <0/1> <G1/G2/G5/G10>
    biquad <0/1> ba ba <[f64; 6]>
    biquad <0/1> ba u <f64>
    biquad <0/1> ba min <f64>
    biquad <0/1> ba max <f64>
    biquad <0/1> raw ba <[f64, 6]>
    biquad <0/1> raw u <f64>
    biquad <0/1> raw min <f64>
    biquad <0/1> raw max <f64>
    biquad <0/1> pid order <P/I/I2>
    biquad <0/1> pid gain <[f64, 5]>
    biquad <0/1> pid limit <[f64, 5]>
    biquad <0/1> pid setpoint <f64>
    biquad <0/1> pid min <f64>
    biquad <0/1> pid max <f64>
    biquad <0/1> filter typ <Lowpass/Highpass/Bandpass/Allpass/Notch/Peaking/Lowshelf/Highshelf/IHo>
    biquad <0/1> filter frequency <f64>
    biquad <0/1> filter gain <f64>
    biquad <0/1> filter shelf <f64>
    biquad <0/1> filter shape <Q/Bandwidth/Slope> <f64>
    biquad <0/1> filter offset <f64>
    biquad <0/1> filter min <f64>
    biquad <0/1> filter max <f64>
    run <0/1> <Run/Hold/External>
    source <0/1> signal <Cosine/Square/Triangle/WhiteNoise/SweptSine>
    source <0/1> frequency <f32>
    source <0/1> symmetry <f32>
    source <0/1> amplitude <f32>
    source <0/1> offset <f32>
    source <0/1> phase <f32>
    source <0/1> length <u32>
    source <0/1> state <i64>
    source <0/1> rate <i32>
    trigger <0/1>
    telemetry_period <f32>
    stream <addr>:<port>    //192.168.0.1:1234
    pounder clock multiplier <u8>
    pounder clock reference_clock <f32>
    pounder clock external_clock <True/False>
    pounder <in_channel/out_channel> <0/1> dds_frequency <f32>
    pounder <in_channel/out_channel> <0/1> dds_phase_offset <f32>
    pounder <in_channel/out_channel> <0/1> dds_amplitude <f32>
    pounder <in_channel/out_channel> <0/1> attenuation <f32>
*/

use core::fmt;
use core::num::ParseIntError;
use core::str::{from_utf8, Utf8Error};
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_while1},
    character::{
        complete::{char, one_of},
        is_digit,
    },
    combinator::{complete, map, opt, value},
    error::ErrorKind,
    multi::{fold_many0, fold_many1},
    sequence::{preceded, tuple},
    IResult, Needed,
};
use num_traits::{Num, ParseFloatError};
use serde::{Deserialize, Serialize};

use log::info;
use crate::convert;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    PounderFrequency {
        frequency: u32,
    },
    Show(ShowCommand),

    Gain {
        channel: usize,
        gain: Gain
    },
    BiquadBa(BiquadBaData),
    BiquadRaw(BiquadRawData),
    BiquadPid(BiquadPidData),
    BiquadFilter(BiquadFilterData),
    Run {
        channel: usize,
        run: Run
    },
    Source(SourceData),
    Trigger,
    TelemetryPeriod(f64),
    Stream{
        ip: [u8; 4],
        port: u16
    },
    PounderClock(PounderClockData),
    PounderChannel(PounderChannelData)
}

impl Command {
    pub fn parse(input: &[u8]) -> Result<Self, Error> {
        info!("Command::parse()");
        match command(input) {
            Ok((input_remain, result)) if input_remain.is_empty() => result,
            Ok((input_remain, _)) => Err(Error::UnexpectedInput(input_remain[0])),
            Err(e) => Err(e.into()),
        }
    }

    pub fn default_biquad_ba(channel: usize, field: BiquadBaField) -> Self {
        Command::BiquadBa(BiquadBaData {
            channel,
            field,
            ba: [0.0; 6],
            u: 0.0,
            min: 0.0,
            max: 0.0
        })
    }

    pub fn default_biquad_raw(channel: usize, field: BiquadRawField) -> Self {
        Command::BiquadRaw(BiquadRawData {
            channel,
            field,
            ba: [0.0; 5],
            u: 0.0,
            min: 0.0,
            max: 0.0
        })
    }

    pub fn default_biquad_pid(channel: usize, field: BiquadPidField) -> Self {
        Command::BiquadPid(BiquadPidData {
            channel,
            field,
            order: Order::P,
            gain: PidParam,
            limit: PidParam,
            setpoint: 0.0,
            min: 0.0,
            max: 0.0
        })
    }

    pub fn default_biquad_filter(channel: usize, field: BiquadFilterField) -> Self {
        Command::BiquadFilter(BiquadFilterData {
            channel,
            field,
            typ: FilterTyp::Lowpass,
            frequency: 0.0,
            gain: 0.0,
            shelf: 0.0,
            shape: FilterShape::Q(0.0),
            offset: 0.0,
            min: 0.0,
            max: 0.0
        })
    }

    pub fn default_pounder_clock(field: PounderClockField) -> Self {
        Command::PounderClock(PounderClockData {
            field,
            multiplier: 0,
            reference_clock: 0.0,
            external_clock: false
        })
    }

    pub fn default_pounder_channel(in_out: InOut, channel: usize, field: PounderChannelField) -> Self {
        Command::PounderChannel(PounderChannelData {
            in_out,
            channel,
            field,
            dds_frequency: 0.0,
            phase_offset: 0.0,
            amplitude: 0.0,
            attenuation: 0.0
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BiquadBaData {
    channel: usize,
    field: BiquadBaField,
    ba: [f64; 6],
    u: f64,
    min: f64,
    max: f64
}

#[derive(Clone, Debug, PartialEq)]
pub struct BiquadRawData {
    channel: usize,
    field: BiquadRawField,
    ba: [f64; 5],
    u: f64,
    min: f64,
    max: f64
}

#[derive(Clone, Debug, PartialEq)]
pub struct BiquadPidData {
    channel: usize,
    field: BiquadPidField,
    order: Order,
    gain: PidParam,
    limit: PidParam,
    setpoint: f64,
    min: f64,
    max: f64
}

#[derive(Clone, Debug, PartialEq)]
pub struct SourceData {
    channel: usize,
    field: SourceField,
    signal: Signal,
    frequency: f64,
    symmetry: f64,
    amplitude: f64,
    offset: f64,
    phase: f64,
    length: u32,
    state: i64,
    rate: i32
}

#[derive(Clone, Debug, PartialEq)]
pub struct BiquadFilterData {
    channel: usize,
    field: BiquadFilterField,
    typ: FilterTyp,
    frequency: f64,
    gain: f64,
    shelf: f64,
    shape: FilterShape,
    offset: f64,
    min: f64,
    max:f64
}

#[derive(Clone, Debug, PartialEq)]
pub struct PounderClockData {
    field: PounderClockField,
    multiplier: u8,
    reference_clock: f32,
    external_clock: bool
}

#[derive(Clone, Debug, PartialEq)]
pub struct PounderChannelData {
    in_out: InOut,
    channel: usize,
    field: PounderChannelField,
    dds_frequency: f32,
    phase_offset: f32,
    amplitude: f32,
    attenuation: f32
}

#[derive(Debug, Clone, PartialEq)]
pub enum Gain {
    G1,
    G2,
    G5,
    G10
}

#[derive(Debug, Clone, PartialEq)]
pub enum BiquadTyp {
    Ba,
    Raw,
    Pid,
    Filter
}

#[derive(Debug, Clone, PartialEq)]
pub enum Order {
    P,
    I,
    I2
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PidParam {
    pub i2: f64,
    pub i: f64,
    pub p: f64,
    pub d: f64,
    pub d2: f64
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterTyp {
    Lowpass,
    Highpass,
    Bandpass,
    Allpass,
    Notch,
    Peaking,
    Lowshelf,
    Highshelf,
    IHo
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterShape {
    Q(f64),
    Bandwidth(f64),
    Slope(f64)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Run {
    Run,
    Hold,
    External
}

#[derive(Debug, Clone, PartialEq)]
pub enum Signal {
    Cosine,
    Square,
    Triangle,
    WhiteNoise,
    SweptSine
}

#[derive(Debug, Clone, PartialEq)]
pub enum InOut {
    InputChannel,
    OutputChannel
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Parser(ErrorKind),
    Incomplete,
    UnexpectedInput(u8),
    Utf8(Utf8Error),
    ParseInt(ParseIntError),
    // `num_traits::ParseFloatError` does not impl Clone
    ParseFloat,
}

impl<'t> From<nom::Err<(&'t [u8], ErrorKind)>> for Error {
    fn from(e: nom::Err<(&'t [u8], ErrorKind)>) -> Self {
        match e {
            nom::Err::Incomplete(_) => Error::Incomplete,
            nom::Err::Error((_, e)) => Error::Parser(e),
            nom::Err::Failure((_, e)) => Error::Parser(e),
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::Utf8(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Error::ParseFloat
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::Incomplete => "incomplete input".fmt(fmt),
            Error::UnexpectedInput(c) => {
                "unexpected input: ".fmt(fmt)?;
                c.fmt(fmt)
            }
            Error::Parser(e) => {
                "parser: ".fmt(fmt)?;
                (e as &dyn core::fmt::Debug).fmt(fmt)
            }
            Error::Utf8(e) => {
                "utf8: ".fmt(fmt)?;
                (e as &dyn core::fmt::Debug).fmt(fmt)
            }
            Error::ParseInt(e) => {
                "parsing int: ".fmt(fmt)?;
                (e as &dyn core::fmt::Debug).fmt(fmt)
            }
            Error::ParseFloat => "parsing float".fmt(fmt),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BiquadBaField {
    Ba,
    U,
    Min,
    Max
}

#[derive(Clone, Debug, PartialEq)]
pub enum BiquadRawField {
    Ba,
    U,
    Min,
    Max
}

#[derive(Clone, Debug, PartialEq)]
pub enum BiquadPidField {
    Order,
    Gain,
    Limit,
    Setpoint,
    Min,
    Max
}

#[derive(Clone, Debug, PartialEq)]
pub enum BiquadFilterField {
    Typ,
    Frequency,
    Gain,
    Shelf,
    Shape,
    Offset,
    Min,
    Max
}

#[derive(Clone, Debug, PartialEq)]
pub enum SourceField {
    Signal,
    Frequency,
    Symmetry,
    Amplitude,
    Offset,
    Phase,
    Length,
    State,
    Rate
}

#[derive(Clone, Debug, PartialEq)]
pub enum PounderClockField {
    Multiplier,
    Reference_clock,
    External_clock
}

#[derive(Clone, Debug, PartialEq)]
pub enum PounderChannelField {
    Dds_frequency,
    Phase_offset,
    Amplitude,
    Attenuation
}

fn end(input: &[u8]) -> IResult<&[u8], ()> {
    complete(fold_many0(one_of("\r\n\t "), (), |(), _| ()))(input)
}

fn whitespace(input: &[u8]) -> IResult<&[u8], ()> {
    fold_many1(char(' '), (), |(), _| ())(input)
}

fn unsigned(input: &[u8]) -> IResult<&[u8], Result<u32, Error>> {
    take_while1(is_digit)(input).map(|(input, digits)| {
        let result = from_utf8(digits)
            .map_err(|e| e.into())
            .and_then(|digits| digits.parse::<u32>().map_err(|e| e.into()));
        (input, result)
    })
}

fn signed(input: &[u8]) -> IResult<&[u8], Result<i64, Error>> {
    let (input, sign) = opt(is_a("-"))(input)?;
    let negative = sign.is_some();
    take_while1(is_digit)(input).map(|(input, digits)| {
        let result = from_utf8(digits)
        .map_err(|e| e.into())
        .and_then(|digits| digits.parse::<u64>().map_err(|e| e.into()))
        .map(|num| num as i64)
        .map(|num| if negative {-num} else {num});
        (input, result)
    })
}

fn float(input: &[u8]) -> IResult<&[u8], Result<f64, Error>> {
    let (input, sign) = opt(is_a("-"))(input)?;
    let negative = sign.is_some();
    let (input, digits) = take_while1(|c| is_digit(c) || c == b'.')(input)?;
    let result = from_utf8(digits)
        .map_err(|e| e.into())
        .and_then(|digits| f64::from_str_radix(digits, 10).map_err(|e| e.into()))
        .map(|result: f64| if negative { -result } else { result });
    Ok((input, result))
}

fn channel(input: &[u8]) -> IResult<&[u8], usize> {
    map(one_of("01"), |c| (c as usize) - ('0' as usize))(input)
}

fn parse_gain(input: &[u8]) -> IResult<&[u8], Gain> {
    alt((
        value(Gain::G10, tag("G10")),
        value(Gain::G1, tag("G1")),
        value(Gain::G2, tag("G2")),
        value(Gain::G5, tag("G5")), 
    ))(input)
}

fn command(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    alt((
        pounder_frequency,
        map(report, Ok),

        gain,
        biquad_ba,
        biquad_raw,
        biquad_pid,
        biquad_filter,
        run,
        source,
        trigger,
        telemetry_period,
        stream,
        pounder_clock,
        pounder_channel
    ))(input)
}

fn report(input: &[u8]) -> IResult<&[u8], Command> {
    let (input, _) = tag("report")(input)?;
    let (input, _) = end(input)?;
    Ok((input, Command::Show(ShowCommand::Input)))
}

fn pounder_frequency(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    let (input, _) = tag("pounder_frequency")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, value) = unsigned(input)?;
    let result = value.map(|freq| {
        Command::PounderFrequency { frequency: freq }
    });
    Ok((input, result))
}

fn gain(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    let (input, _) = tag("gain")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, channel) = channel(input)?;
    let (input, _) = whitespace(input)?;
    let (input, gain) = parse_gain(input)?;
    end(input)?;

    Ok((
        input, 
        Ok(Command::Gain {channel, gain}),
    ))
}

fn biquad_ba(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    let (input, _) = tag("biquad")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, parsed_channel) = channel(input)?;
    let (input, _) = whitespace (input)?;
    let (input, _) = tag("ba")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, result) = alt((
        |input| {
            let (input, _) = tag("ba")(input)?;
            let (input, _) = whitespace (input)?;
            let parse_6_f32 = tuple ((
                float, whitespace, float, whitespace, float, whitespace, float, whitespace, float, whitespace, float
            ));
            let (input, (b0, _, b1, _, b2, _, a0, _, a1, _, a2)) = parse_6_f32(input)?;
            let ba_array = match (b0, b1, b2, a0, a1, a2) {
                (Ok(b0), Ok(b1), Ok(b2), Ok(a0), Ok(a1), Ok(a2)) =>
                    [b0, b1, b2, a0, a1, a2],
                _ => return Ok((input, Err(Error::ParseFloat))),
            };
            let cmd = match Command::default_biquad_ba(parsed_channel, BiquadBaField::Ba) {
                Command::BiquadBa(mut data) => {
                    data.ba = ba_array;
                    Command::BiquadBa(data)
                }
                _ => unreachable!()
            };
            Ok((input, Ok(cmd)))
        },
        |input| {
            let (input, (parsed_field, _, parsed_value)) = tuple((
                alt((
                    value(BiquadBaField::U, tag("u")),
                    value(BiquadBaField::Min, tag("min")),
                    value(BiquadBaField::Max, tag("max")),
                )),
                whitespace,
                float,
            ))(input)?;
            let param_result = match parsed_value {
                Ok(value) => {
                    let (u, min, max) = match parsed_field {
                        BiquadBaField::U => (value, 0.0, 0.0),
                        BiquadBaField::Min => (0.0, value, 0.0),
                        BiquadBaField::Max => (0.0, 0.0, value),
                        _ => unreachable!()
                    };
                    let cmd = match Command::default_biquad_ba(parsed_channel, parsed_field) {
                        Command::BiquadBa(mut data) => {
                            (data.u, data.min, data.max) = (u, min, max);
                            Command::BiquadBa(data)
                        }
                        _ => unreachable!()
                    };
                    Ok((input, Ok(cmd)))
                }
                Err(_e) => return Ok((input, Err(Error::ParseFloat)))
            };
            param_result
        }
    ))(input)?;

    let (input, _) = end(input)?;
    Ok((input, result))
}

fn biquad_raw(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    let (input, _) = tag("biquad")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, parsed_channel) = channel(input)?;
    let (input, _) = whitespace(input)?;
    let (input, _) = tag("raw")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, result) = alt((
        |input| {
            let (input, _) = tag("ba")(input)?;
            let (input, _) = whitespace(input)?;
            let parse_5_f32 = tuple((
                float, whitespace, float, whitespace, float, whitespace, float, whitespace, float
            ));
            let (input, (b0, _, b1, _, b2, _, a1, _, a2)) = parse_5_f32(input)?;
            let ba_array = match (b0, b1, b2, a1, a2) {
                (Ok(b0), Ok(b1), Ok(b2), Ok(a1), Ok(a2)) =>
                    [b0, b1, b2, a1, a2],
                _ => return Ok((input, Err(Error::ParseFloat)))
            };
            let cmd = match Command::default_biquad_raw(channel, BiquadRawField::Ba) {
                Command::BiquadRaw(mut data) => {
                    data.ba = ba_array;
                    Command::BiquadRaw(data)
                }
                _ => unreachable!()
            };
            Ok((input, Ok(cmd)))
        },
        |input| {
            let (input, (parsed_field, _, parsed_value)) = tuple((
                alt((
                    value(BiquadRawField::U, tag("u")),
                    value(BiquadRawField::Min, tag("min")),
                    value(BiquadRawField::Max, tag("max"))
                )),
                whitespace,
                float
            ))(input)?;
            let param_result = match parsed_value {
                Ok(value) => {
                    let (u, min, max) = match parsed_field {
                        BiquadRawField::U => (value, 0.0, 0.0),
                        BiquadRawField::Min => (0.0, value, 0.0),
                        BiquadRawField::Max => (0.0, 0.0, value),
                        _ => unreachable!()
                    };
                    let cmd = match Command::default_biquad_raw(channel, parsed_field) {
                        Command::BiquadRaw(data) => {
                            (data.u, data.min, data.max) = (u, min, max);
                            Command::BiquadRaw(data)
                        }
                        _ => unreachable!()
                    };
                    Ok((input, Ok(cmd)))
                }
                Err(_e) => return Ok((input, Err(Error::ParseFloat)))
            };
            param_result
        }
    ))(input)?;

    let (input, _) = end(input)?;
    Ok((input, result))
}

fn biquad_pid(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    let (input, _) = tag("biquad")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, parsed_channel) = channel(input)?;
    let (input, _) = tag("pid")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, result) = alt ((
        |input| {
            let (input, _) = tag("order")(input)?;
            let (input, _) = whitespace(input)?;
            let (input, order) = alt((
                value(Order::P, tag("P")),
                value(Order::I, tag("I")),
                value(Order::I2, tag("I2"))
            ))(input)?;
            let cmd = match Command::default_biquad_pid(parsed_channel, BiquadPidField::Order) {
                Command::BiquadPid(data) {
                    data.order = order;
                    Command::BiquadPid(data)
                }
                _ => unreachable!()
            };
            Ok((input, Ok(cmd)))
        },
        |input| {
            let (input, (parsed_field, _, (i2, _, i, _, p, _, d, _, d2))) = tuple((
                alt((
                    value(BiquadPidField::Gain, tag("gain")),
                    value(BiquadPidField::Limit, tag("limit"))
                )),
                whitespace,
                tuple((
                    float, whitespace, float, whitespace, float, whitespace, float, whitespace, float
                ))
            ))(input)?;
            let (gain, limit) = match (i2, i, p, d, d2) {
                (Ok(i2), Ok(i), Ok(p), Ok(d), Ok(d2)) => {
                    match parsed_field {
                        BiquadPidField::Gain => (PidParam {
                                i2, i, p, d, d2
                            }, PidParam {
                                i2: 0.0, i: 0.0, p: 0.0, d: 0.0, d2: 0.0
                            }),
                        BiquadPidField::Limit => (PidParam {
                                i2: 0.0, i: 0.0, p: 0.0, d: 0.0, d2: 0.0
                            }, PidParam {
                                i2, i, p, d, d2
                            }),
                        _ => unreachable!()
                    }
                }
                _ => return Ok((input, Err(Error::ParseFloat)))
            };
            let cmd = match Command::default_biquad_pid(parsed_channel, parsed_field) {
                Command::BiquadPid(data) => {
                    (data.gain, data.limit) = (gain, limit);
                    Command::BiquadPid(data)
                };
                _ => unreachable!()
            }
            Ok((input, Ok(cmd)))
        },
        |input| {
            let (input, (parsed_field, _, parsed_value)) = tuple((
                alt((
                    value(BiquadPidField::Setpoint, tag("setpoint")),
                    value(BiquadPidField::Min, tag("min")),
                    value(BiquadPidField::Max, tag("max"))
                )),
                whitespace,
                float
            ))(input)?;
            match parsed_value {
                Ok(value) => {
                    let (setpoint, min, max) = match parsed_field {
                        BiquadPidField::Setpoint => (value, 0.0, 0.0),
                        BiquadPidField::Min => (0.0, value, 0.0),
                        BiquadPidField::Max => (0.0, 0.0, value),
                        _ => unreachable!()
                    };
                    let cmd = match Command::default_biquad_pid(parsed_channel, parsed_field) {
                        Command::BiquadPid(data) => {
                            (data.setpoint, data.min, data.max) = (setpoint, min, max)
                            Command::BiquadPid(data)
                        }
                        _ => unreachable!()
                    };
                    Ok((input, Ok(cmd)))
                }
                Err(_e) => return Ok((input, Err(Error::ParseFloat)))
            }
        }
    ))(input)?;

    let (input, _) = end(input)?;
    Ok((input, result))
}

fn biquad_filter(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    let (input, _) = tag("biquad")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, channel) = channel(input)?;
    let (inpit, _) = whitespace(input)?;
    let (input, _) = tag("filter")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, result) = alt((
        |input| {
            let (input, _) = tag("typ")(input)?;
            let (input, _) = whitespace(input)?;
            let (input, typ) = alt((
                value(FilterTyp::Lowpass, tag("Lowpass")),
                value(FilterTyp::Highpass, tag("Highpass")),
                value(FilterTyp::Bandpass, tag("Bandpass")),
                value(FilterTyp::Allpass, tag("Allpass")),
                value(FilterTyp::Notch, tag("Notch")),
                value(FilterTyp::Peaking, tag("Peaking")),
                value(FilterTyp::Lowshelf, tag("Lowshelf")),
                value(FilterTyp::Highshelf, tag("Highshelf")),
                value(FilterTyp::IHo, tag("IHo"))
            ))(input)?;
            let cmd = match Command::default_biquad_filter(parsed_channel, BiquadFilterField::Typ) {
                Command::BiquadFilter(data) => {
                    data.typ = typ;
                    Command::BiquadFilter(data)
                }
                _ => unreachable!()
            };
            Ok((input, Ok(cmd)))
        },
        |input| {
            let (input, (parsed_field, _, parsed_value)) = tuple((
                alt((
                    value(BiquadFilterField::Frequency, tag("frequency")),
                    value(BiquadFilterField::Gain, tag("gain")),
                    value(BiquadFilterField::Shelf, tag("shelf")),
                    value(BiquadFilterField::Offset, tag("offset")),
                    value(BiquadFilterField::Min, tag("min")),
                    value(BiquadFilterField::Max, tag("max"))
                )),
                whitespace,
                float
            ))(input)?;
            match parsed_value {
                Ok(value) => {
                    let (frequency, gain, shelf, offset, min, max) = match parsed_field {
                        BiquadFilterField::Frequency => (value, 0.0, 0.0, 0.0, 0.0, 0.0),
                        BiquadFilterField::Gain => (0.0, value, 0.0, 0.0, 0.0, 0.0),
                        BiquadFilterField::Shelf => (0.0, 0.0, value, 0.0, 0.0, 0.0),
                        BiquadFilterField::Offset => (0.0, 0.0, 0.0, value, 0.0, 0.0),
                        BiquadFilterField::Min => (0.0, 0.0, 0.0, 0.0, value, 0.0),
                        BiquadFilterField::Max => (0.0, 0.0, 0.0, 0.0, 0.0, value),
                        _ => unreachable!()
                    };
                    let cmd = match Command::default_biquad_pid(parsed_channel, parsed_field) {
                        Command::BiquadPid(data) => {
                            (data.frequency, data.gain, data.shelf, data.offset, data.min, data.max) = (frequency, gain, shelf, offset, min, max);
                            Command:BiquadPid(data)
                        }
                        _ => unreachable!()
                    };
                    Ok((input, Ok(cmd)))
                }
                Err(_e) => return Ok((input, Err(Error::ParseFloat)))
            }
        },
        |input| {
            let (input, _) = tag("shape")(input)?;
            let (input, (parsed_shape, _, parsed_value)) = tuple((
                alt((
                    value(FilterShape::Q(0.0), tag("Q")),
                    value(FilterShape::Bandwidth(0.0), tag("Bandwidth")),
                    value(FilterShape::Slope(0.0), tag("Slope"))
                )),
                whitespace,
                float
            ))(input)?;
            let shape = match parsed_value {
                Ok(value) => {
                    match parsed_shape {
                        FilterShape::Q(_) => FilterShape::Q(value),
                        FilterShape::Bandwidth(_) => FilterShape::Bandwidth(value),
                        FilterShape::Slope(_) => FilterShape::Slope(value),
                    }
                }
                Err(_e) => return Ok((input, Err(Error::ParseFloat)))
            };
            let cmd = match Command::default_biquad_filter (parsed_channel, BiquadFilterField::Shape) {
                Command::BiquadFilter(data) => {
                    data.shape = shape;
                    Command:BiquadFilter(data)
                }
                _ => unreachable!()
            };
            Ok((input, Ok(cmd)))
        }
    ))(input)?;

    let (input, _) = end(input)?;
    Ok((input, result))
}

fn run(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    let (input, _) = tag("run")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, channel) = channel(input)?;
    let (input, _) = whitespace(input)?;
    let (input, run) = alt((
        value(Run::Run, tag("Run")),
        value(Run::Hold, tag("Hold")),
        value(Run::External, tag("External"))
    ))(input)?;
    let cmd = Command::Run {
        channel,
        run
    };
    Ok((input, Ok(cmd)))
}

fn source(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    let (input, _) = tag("source")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, channel) = channel(input)?;
    let (input, _) = whitespace(input)?;
    let (input, result) = alt((
        |input| {
            let (input, _) = tag("signal")(input)?;
            let (input, _) = whitespace(input)?;
            let (input, signal) = alt((
                value(Signal::Cosine, tag("Cosine")),
                value(Signal::Square, tag("Square")),
                value(Signal::Triangle, tag("Triangle")),
                value(Signal::WhiteNoise, tag("WhiteNoise")),
                value(Signal::SweptSine, tag("SweptSine"))
            ))(input)?;
            let cmd = Command::Source {
                channel,
                field: SourceField::Signal,
                signal,
                frequency: 0.0,
                symmetry: 0.0,
                amplitude: 0.0,
                offset: 0.0,
                phase: 0.0,
                length: 0,
                state: 0,
                rate:0
            };
            Ok((input, Ok(cmd)))
        },
        |input| {
            let (input, (field, _, parsed_value)) = tuple((
                alt((
                    value(SourceField::Frequency, tag("frequency")),
                    value(SourceField::Symmetry, tag("symmetry")),
                    value(SourceField::Amplitude, tag("amplitude")),
                    value(SourceField::Offset, tag("offset")),
                    value(SourceField::Phase, tag("phase"))
                )),
                whitespace,
                float
            ))(input)?;
            match parsed_value {
                Ok(value) => {
                    let (frequency, symmetry, amplitude, offset, phase) = match field {
                        SourceField::Frequency => (value, 0.0, 0.0, 0.0, 0.0),
                        SourceField::Symmetry => (0.0, value, 0.0, 0.0, 0.0),
                        SourceField::Amplitude => (0.0, 0.0, value, 0.0, 0.0),
                        SourceField::Offset => (0.0, 0.0, 0.0, value, 0.0),
                        SourceField::Phase => (0.0, 0.0, 0.0, 0.0, value),
                        _ => unreachable!()
                    };
                    let cmd = Command::Source {
                        channel,
                        field,
                        signal: Signal::Cosine,
                        frequency,
                        symmetry,
                        amplitude,
                        offset,
                        phase,
                        length: 0,
                        state: 0,
                        rate: 0
                    };
                    Ok((input, Ok(cmd)))
                }
                Err(_e) => return Ok((input, Err(Error::ParseFloat)))
            }
        },
        |input| {
            let (input, _) = tag("length")(input)?;
            let (input, _) = whitespace(input)?;
            let (input, parsed_length) = unsigned(input)?;
            let cmd = parsed_length.map(|length| {
                Command::Source {
                    channel,
                    field: SourceField::Length,
                    signal: Signal::Cosine,
                    frequency: 0.0,
                    symmetry: 0.0,
                    amplitude: 0.0,
                    offset: 0.0,
                    phase: 0.0,
                    length,
                    state: 0,
                    rate: 0
                }
            });
            Ok((input, cmd))
        },
        |input| {
            let (input, _) = tag("state")(input)?;
            let (input, _) = whitespace(input)?;
            let (input, parsed_state) = signed(input)?;
            let cmd = parsed_state.map(|state| {
                Command::Source {
                    channel,
                    field: SourceField::Rate,
                    signal: Signal::Cosine,
                    frequency: 0.0,
                    symmetry: 0.0,
                    amplitude: 0.0,
                    offset: 0.0,
                    phase: 0.0,
                    length: 0,
                    state,
                    rate: 0
                }
            });
            Ok((input, cmd))
        },
        |input| {
            let (input, _) = tag("rate")(input)?;
            let (input, _) = whitespace(input)?;
            let (input, value) = signed(input)?;
            let cmd = value.map(|parsed_rate| {
                Command::Source {
                    channel,
                    field: SourceField::Rate,
                    signal: Signal::Cosine,
                    frequency: 0.0,
                    symmetry: 0.0,
                    amplitude: 0.0,
                    offset: 0.0,
                    phase: 0.0,
                    length: 0,
                    state: 0,
                    rate: parsed_rate as i32
                }
            });
            Ok((input, cmd))
        }
    ))(input)?;

    let(input, _) = end(input)?;
    Ok((input, result))
}

fn trigger(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    let (input, _) = tag("trigger")(input)?;
    let (input, _) = end(input)?;
    Ok((input, Ok(Command::Trigger)))
}

fn telemetry_period(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    let (input, _) = tag("telemetry_period")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, value) = float(input)?;
    let cmd = value.map(|telemetry_period| {
        Command::TelemetryPeriod (telemetry_period)
    });
    Ok((input, cmd))
}

fn stream(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    let (input, _) = tag("stream")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, a_result) = unsigned(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, b_result) = unsigned(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, c_result) = unsigned(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, d_result) = unsigned(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, port_result) = unsigned(input)?;
    let (input, _) = end(input)?;
    let cmd = a_result.and_then(|a| {
        b_result.and_then(|b| {
            c_result.and_then(|c| {
                d_result.and_then(|d| {
                    port_result.map(|port| {
                        Command::Stream {
                            ip: [a as u8, b as u8, c as u8, d as u8],
                            port: port as u16
                        }
                    })
                })
            })
        })
    });

    Ok((input, cmd))
}

fn pounder_clock(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    let (input, _) = tag("pounder")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, result) = alt((
        |input| {
            let (input, _) = tag("multiplier")(input)?;
            let (input, _) = whitespace(input)?;
            let (input, parsed_multiplier) = unsigned(input)?;
            let cmd = parsed_multiplier.map(|value| {
                Command::PounderClock {
                    field: PounderClockField::Multiplier,
                    multiplier: value as u8,
                    reference_clock: 0.0,
                    external_clock: false
                }
            });
            Ok((input, cmd))
        },
        |input| {
            let (input, _) = tag("reference_clock")(input)?;
            let (input, _) = whitespace(input)?;
            let (input, parsed_reference_clock) = float(input)?;
            let cmd = parsed_reference_clock.map(|value| {
                Command::PounderClock {
                    field: PounderClockField::Reference_clock,
                    multiplier: 0,
                    reference_clock: value as f32,
                    external_clock: false
                }
            });
            Ok((input, cmd))
        },
        |input| {
            let (input, _) = tag("external_clock")(input)?;
            let (input, _) = whitespace(input)?;
            let (input, external_clock) = alt((
                value(true, tag("True")),
                value(false, tag("False"))
            ))(input)?;
            let cmd = Command::PounderClock {
                field: PounderClockField::External_clock,
                multiplier: 0,
                reference_clock: 0.0,
                external_clock
            };
            Ok((input, Ok(cmd)))
        }
    ))(input)?;

    let (input, _) = end(input)?;
    Ok((input, result))
}

fn pounder_channel(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    let (input, _) = tag("pounder")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, in_out) = alt((
        value(InOut::InputChannel, tag("in_channel")),
        value(InOut::OutputChannel, tag("out_channel"))
    ))(input)?;
    let (input, _) = whitespace(input)?;
    let (input, channel) = channel(input)?;
    let (input, (field, _, parsed_value)) = tuple((
        alt((
            value(PounderChannelField::Dds_frequency, tag("dds_frequency")),
            value(PounderChannelField::Phase_offset, tag("dds_phase_offset")),
            value(PounderChannelField::Amplitude, tag("dds_amplitude")),
            value(PounderChannelField::Attenuation, tag("attenuation"))
        )),
        whitespace,
        float
    ))(input)?;
    match parsed_value {
        Ok(value) => {
            let (dds_frequency, phase_offset, amplitude, attenuation) = match field {
                PounderChannelField::Dds_frequency => (value as f32, 0.0, 0.0, 0.0),
                PounderChannelField::Phase_offset => (0.0, value as f32, 0.0, 0.0),
                PounderChannelField::Amplitude => (0.0, 0.0, value as f32, 0.0),
                PounderChannelField::Attenuation => (0.0, 0.0, 0.0, value as f32),
                _ => unreachable!()
            };
            let cmd = Command::PounderChannel {
                in_out,
                channel,
                field,
                dds_frequency,
                phase_offset,
                amplitude,
                attenuation
            };
            let (input, _) = end(input)?;
            return Ok((input, Ok(cmd)))
        }
        Err(_e) => return Ok((input, Err(Error::ParseFloat)))
    }
}

