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
        ├── clock (hardware::pounder::CLockConfig)
        │   ├── multiplier: u8
        │   ├── reference_clock: u32
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
    biquad <0/1> ba ba <T[6]>
    biquad <0/1> ba u <T>
    biquad <0/1> ba min <T>
    biquad <0/1> ba max <T>
    biquad <0/1> raw ba <T[5]>
    biquad <0/1> raw u <T>
    biquad <0/1> raw min <T>
    biquad <0/1> raw max <T>
    biquad <0/1> pid order <P/I/I2>
    biquad <0/1> pid gain <T[5]>
    biquad <0/1> pid limit <T[5]>
    biquad <0/1> pid setpoint <T>
    biquad <0/1> pid min <T>
    biquad <0/1> pid max <T>
    biquad <0/1> filter typ <Lowpass/Highpass/Bandpass/Allpass/Notch/Peaking/Lowshelf/Highshelf/IHo>
    biquad <0,1> filter frequency <T>
    biquad <0,1> filter gain <T>
    biquad <0,1> filter shelf <T>
    biquad <0,1> filter shape <Q/Bandwidth/Slope> <T>
    biquad <0,1> filter offset <T>
    biquad <0,1> filter min <T>
    biquad <0,1> filter max <T>
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
    pounder clock reference_clock <u32>
    pounder clock external_clock <0/1>
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
    BiquadBa {
        channel: usize,
        field: BiquadBaField,
        ba: [f32; 6],
        u: f32,
        min: f32,
        max: f32
    },
    BiquadRaw {
        channel: usize,
        field: BiquadRawField,
        ba: [f32; 5],
        u: f32,
        min: f32,
        max: f32
    },
    BiquadPid {
        channel: usize,
        field: BiquadPidField,
        order: Order,
        gain: PidParam,
        limit: PidParam,
        setpoint: f32,
        min: f32,
        max: f32
    },
    BiquadFilter {
        channel: usize,
        field: BiquadFilterField,
        typ: FilterTyp,
        frequency: f32,
        gain: f32,
        shelf: f32,
        shape: FilterShape,
        offset: f32,
        min: f32,
        max:f32
    },
    Run {
        channel: usize,
        run: Run
    },
    Source {
        channel: usize,
        field: SourceField,
        signal: Signal,
        frequency: f32,
        symmetry: f32,
        amplitude: f32,
        offset: f32,
        phase: f32,
        length: u32,
        state: i64,
        rate: i32
    },
    Trigger,
    TelemetryPeriod(f32),
    Stream(core::net::SocketAddr),
    PounderClock {
        multiplier: u8,
        reference_clock: u32,
        external_clock: bool
    },
    PounderChannel {
        in_out: InOut,
        channel: usize,
        field: PounderChannelField,
        dds_frequency: f32,
        phase_offset: f32,
        amplitude: f32,
        attenuation: f32
    }
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

#[derive(Debug, Clone, PartialEq)]
pub struct PidParam {
    pub i2: f32,
    pub i: f32,
    pub p: f32,
    pub d: f32,
    pub d2: f32
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
    Q(f32),
    BandWidth(f32),
    Slope(f32)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Run {
    Run,
    Hold,
    External
}

//source <0/1> signal <Cosine/Square/Triangle/WhiteNoise/SweptSine>
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

#[derive(Clone, Debug, PartialEq)]
pub enum BiquadBaField {
    None,
    Ba,
    U,
    Min,
    Max
}

#[derive(Clone, Debug, PartialEq)]
pub enum BiquadRawField {
    None,
    Ba,
    U,
    Min,
    Max
}

#[derive(Clone, Debug, PartialEq)]
pub enum BiquadPidField {
    None,
    Order,
    Gain,
    Limit,
    Setpoint,
    Min,
    Max
}

#[derive(Clone, Debug, PartialEq)]
pub enum BiquadFilterField {
    None,
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
    None,
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
pub enum PounderChannelField {
    None,
    Dds_frequency,
    Phase_offset,
    Amplitude,
    Attenuation
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

#[derive(Debug, Clone, PartialEq)]
pub enum ShowCommand {
    Input,
    Output,
    Pid,
    BParameter,
    PostFilter,
    Ipv4,
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
        // biquad_raw,
        // biquad_pid,
        // biquad_filter,
        // run,
        // source,
        // trigger,
        // telemetry_period,
        // stream,
        // pounder_clock,
        // pounder_channel
    ))(input)
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
                    [b0 as f32, b1 as f32, b2 as f32, a0 as f32, a1 as f32, a2 as f32],
                _ => return Ok((input, Err(Error::ParseFloat))),
            };
            let cmd = Command::BiquadBa {
                channel: parsed_channel,
                field: BiquadBaField::Ba,
                ba: ba_array,
                u: 0.0,
                min: 0.0,
                max: 0.0,
            };

            Ok((input, Ok(cmd)))
        },

        // |input| {
        //     let (input, parsed_field) = alt((
        //        value(BiquadBaField::U, tag("u")),
        //        value(BiquadBaField::Min, tag("min")),
        //        value(BiquadBaField::Max, tag("max"))
        //     ))(input)?;
        //     let (input, _) = whitespace (input)?;
        //     let (input, parsed_u) = float(input)?;
        //     match parsed_u {
        //         Ok(value) => {
        //             match parsed_field {
        //                 BiquadBaField::U => {
        //                     let cmd = Command::BiquadBa {
        //                         channel: parsed_channel,
        //                         field: BiquadBaField::U,
        //                         ba: [0.0; 6],
        //                         u: value as f32,
        //                         min: 0.0,
        //                         max: 0.0
        //                     }
        //                     Ok((input, Ok(cmd)))
        //                 },
        //                 BiquadBaField::Min => {
        //                     let cmd = Command::BiquadBa {
        //                         channel: parsed_channel,
        //                         field: BiquadBaField::Min,
        //                         ba: [0.0; 6];
        //                         u: 0.0,
        //                         min: value as f32,
        //                         max: 0.0
        //                     }
        //                     Ok((input, Ok(cmd)))
        //                 },
        //                 BiquadBaField::Max => {
        //                     let cmd = Command::BiquadBa {
        //                         channel: parsed_channel,
        //                         field: BiquadBaField::Max,
        //                         ba: [0.0; 6],
        //                         u: 0.0,
        //                         min: 0.0,
        //                         max: value as f32
        //                     }
        //                     Ok((input, Ok(cmd)))
        //                 },
        //                 _ => Ok((input, Err(Error::)))
        //             }
        //         }
        //         Err(e) => Ok((input, Err(Error::Incomplete)))
        //     }
        // },

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
            let result = match parsed_value {
                Ok(value) => {
                    
                }
            }
        }
    ))(input)?;

    let (input, _) = end(input)?;
    Ok((input, result))
}

// fn biquad_raw(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {}

// fn biquad_pid(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {}

// fn biquad_filter(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {}

// fn run(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {}

// fn source(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {}

// fn trigger(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {}

// fn telemetry_period(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {}

// fn stream(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {}

// fn pounder_clock(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {}

// fn pounder_channel(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {}

/*
    gain <0/1> <G1/G2/G5/G10>
    biquad <0/1> ba ba <T[6]>
    biquad <0/1> ba u <T>
    biquad <0/1> ba min <T>
    biquad <0/1> ba max <T>
    biquad <0/1> raw ba <T[5]>
    biquad <0/1> raw u <T>
    biquad <0/1> raw min <T>
    biquad <0/1> raw max <T>
    biquad <0/1> pid order <P/I/I2>
    biquad <0/1> pid ain <T[5]>
    biquad <0/1> pid limit <T[5]>
    biquad <0/1> pid setpoint <T>
    biquad <0/1> pid min <T>
    biquad <0/1> pid max <T>
    biquad <0/1> filter typ <Lowpass/Highpass/Bandpass/Allpass/Notch/Peaking/Lowshelf/Highshelf/IHo>
    biquad <0,1> filter frequency <T>
    biquad <0,1> filter gain <T>
    biquad <0,1> filter shelf <T>
    biquad <0,1> filter shape <Q/Bandwidth/Slope> <T>
    biquad <0,1> filter offset <T>
    biquad <0,1> filter min <T>
    biquad <0,1> filter max <T>
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
    pounder clock reference_clock <u32>
    pounder clock external_clock <0/1>
    pounder <in_channel/out_channel> <0/1> dds_frequency <f32>
    pounder <in_channel/out_channel> <0/1> dds_phase_offset <f32>
    pounder <in_channel/out_channel> <0/1> dds_amplitude <f32>
    pounder <in_channel/out_channel> <0/1> attenuation <f32>
*/

