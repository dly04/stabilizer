/*
    DualIir (dual-iir::DualIir)
    ├── ch[0] (dual-iir::Channel) %
    │   ├── gain: convert::Gain (enum) %
    │   ├── biquad[0] %
    │   │   ├── typ: idsp::iir::BiquadRepr (enum) %
    │   │   └── repr (idsp::iir::BiquadRepr) (enum)%
    |   |       ├── Ba (idsp::iir::repr::Ba) %
    │   │       │   ├── ba: [T[3]]2 %
    │   │       │   ├── u: T %
    │   │       │   ├── min: T %
    │   │       │   └── max: T %
    │   │       ├── Raw (idsp::iir::biquad::Biquad) %
    │   │       │   ├── ba: T[5] %
    │   │       │   ├── u: T %
    │   │       │   ├── min: T %
    │   │       │   └── max: T %
    │   │       ├── Pid (idsp::iir::Pid) %
    │   │       │   ├── order: idsp::iir::pid::Order (enum) %
    │   │       │   ├── gain %
    |   |       |   |   ├── i2: T %
    │   │       │   |   ├── i: T %
    │   │       │   |   ├── p: T %
    │   │       │   |   ├── d: T %
    │   │       │   |   └── d2: T %
    │   │       │   ├── limit %
    |   |       |   |   ├── i2: T %
    │   │       │   |   ├── i: T %
    │   │       │   │   ├── p: T %
    │   │       │   |   ├── d: T %
    │   │       │   |   └── d2: T %
    │   │       │   ├── setpoint: T %
    │   │       │   ├── min: T %
    │   │       │   └── max: T %
    │   │       └── Filter (idsp::iir::repr::FilterRepr) %
    │   │           ├── typ: idsp::iir::repr::Typ (enum) %
    │   │           ├── frequency: T %
    │   │           ├── gain: T %
    │   │           ├── shelf: T %
    │   │           ├── shape (idsp::iir::coefficients::Shape) (enum) %
    │   │           │   ├── Q: T %
    │   │           │   ├── Bandwidth: T %
    │   │           │   └── Slope: T %
    │   │           ├── offset: T %
    │   │           ├── min: T %
    │   │           └── max:T %
    │   ├── run: dual-iir::Run (enum) %
    │   └── source (signal_generator::Config) %
    │       ├── signal: signal_generator::Signal (enum) %
    │       ├── frequency: f32 %
    │       ├── symmetry: f32 %
    │       ├── amplitude: f32 %
    │       ├── offset: f32 %
    │       ├── phase: f32 %
    │       ├── length: u32 %
    │       ├── state: i64 %
    │       └── rate: i32 %
    ├── ch[1] (dual-iir::Channel) %
    │   ├── gain: convert::Gain (enum) %
    │   ├── biquad[0] %
    │   │   ├── typ: idsp::iir::BiquadRepr (enum) %
    │   │   └── repr (idsp::iir::BiquadRepr) (enum) %
    |   |       ├── Ba (idsp::iir::repr::Ba) %
    │   │       │   ├── ba: [T[3]]2 %
    │   │       │   ├── u: T %
    │   │       │   ├── min: T %
    │   │       │   └── max: T %
    │   │       ├── Raw (idsp::iir::biquad::Biquad) %
    │   │       │   ├── ba: T[5] %
    │   │       │   ├── u: T %
    │   │       │   ├── min: T %
    │   │       │   └── max: T %
    │   │       ├── Pid (idsp::iir::Pid) %
    │   │       │   ├── order: idsp::iir::pid::Order (enum) %
    │   │       │   ├── gain %
    |   |       |   |   ├── i2: T %
    │   │       │   |   ├── i: T %
    │   │       │   |   ├── p: T %
    │   │       │   |   ├── d: T %
    │   │       │   |   └── d2: T %
    │   │       │   ├── limit %
    |   |       |   |   ├── i2: T %
    │   │       │   |   ├── i: T %
    │   │       │   │   ├── p: T %
    │   │       │   |   ├── d: T %
    │   │       │   |   └── d2: T %
    │   │       │   ├── setpoint: T %
    │   │       │   ├── min: T %
    │   │       │   └── max: T %
    │   │       └── Filter (idsp::iir::repr::FilterRepr) %
    │   │           ├── typ: idsp::iir::repr::Typ (enum) %
    │   │           ├── frequency: T %
    │   │           ├── gain: T %
    │   │           ├── shelf: T %
    │   │           ├── shape (idsp::iir::coefficients::Shape) (enum) %
    │   │           │   ├── Q: T %
    │   │           │   ├── Bandwidth: T %
    │   │           │   └── Slope: T %
    │   │           ├── offset: T %
    │   │           ├── min: T %
    │   │           └── max:T %
    │   ├── run: dual-iir::Run (enum) %
    │   └── source (signal_generator::Config) %
    │       ├── signal: signal_generator::Signal (enum) %
    │       ├── frequency: f32 %
    │       ├── symmetry: f32 %
    │       ├── amplitude: f32 %
    │       ├── offset: f32 %
    │       ├── phase: f32 %
    │       ├── length: u32 %
    │       ├── state: i64 %
    │       └── rate: i32 %
    ├── trigger: bool %
    ├── telemetry_period: f32 %
    ├── stream (stream::Target) %
    │   └── : core::net::SocketAddr %
    └── pounder (Option<hardware::pounder::PounderConfig>)
        ├── clock (hardware::pounder::CLockConfig) %
        │   ├── multiplier: u8 %
        │   ├── reference_clock: u32 %
        │   └── external_clock: bool %
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
    biquad <0/1> <0> typ <Ba/Raw/Pid/Filter>
    for typ = Ba:
        biquad <0/1> <0> ba <T[6]>
        biquad <0/1> <0> u <T>
        biquad <0/1> <0> min <T>
        biquad <0/1> <0> max <T>
    for typ = Raw:
        biquad <0/1> <0> ba <T[5]>
        biquad <0/1> <0> u <T>
        biquad <0/1> <0> min <T>
        biquad <0/1> <0> max <T>
    for typ = Pid:
        biquad <0/1> <0> order <P/I/I2>
        biquad <0/1> <0> gain <T[5]>
        biquad <0/1> <0> limit <T[5]>
        biquad <0/1> <0> setpoint <T>
        biquad <0/1> <0> min <T>
        biquad <0/1> <0> max <T>
    for typ = Filter:
        biquad <0/1> <0> typ <Lowpass/Highpass/Bandpass/Allpass/Notch/Peaking/Lowshelf/Highshelf/IHo>
        biquad <0,1> <0> frequency <T>
        biquad <0,1> <0> gain <T>
        biquad <0,1> <0> shelf <T>
        biquad <0,1> <0> shape <Q/Bandwidth/Slope> <T>
        biquad <0,1> <0> offset <T>
        biquad <0,1> <0> min <T>
        biquad <0,1> <0> max <T>
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
    sequence::preceded,
    IResult, Needed,
};
use num_traits::{Num, ParseFloatError};
use serde::{Deserialize, Serialize};

use log::info;

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ipv4Config {
    pub address: [u8; 4],
    pub mask_len: u8,
    pub gateway: Option<[u8; 4]>,
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

#[derive(Debug, Clone, PartialEq)]
pub enum PidParameter {
    Target,
    KP,
    KI,
    KD,
    OutputMin,
    OutputMax,
}

/// B-Parameter equation parameter
#[derive(Debug, Clone, PartialEq)]
pub enum BpParameter {
    T0,
    B,
    R0,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PwmPin {
    ISet,
    MaxIPos,
    MaxINeg,
    MaxV,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CenterPoint {
    VRef,
    Override(f32),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Polarity {
    Normal,
    Reversed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    PounderFrequency {
        frequency: u32,
    },
    Show(ShowCommand),
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

fn report(input: &[u8]) -> IResult<&[u8], Command> {
    info!("here, report!");
    let (input, _) = tag("report")(input)?;
    let (input, _) = end(input)?;
    Ok((input, Command::Show(ShowCommand::Input)))
}

fn ipv4_addr(input: &[u8]) -> IResult<&[u8], Result<[u8; 4], Error>> {
    let (input, a) = unsigned(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, b) = unsigned(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, c) = unsigned(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, d) = unsigned(input)?;
    let address = move || Ok([a? as u8, b? as u8, c? as u8, d? as u8]);
    Ok((input, address()))
}

fn ipv4(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    info!("here, ipv4!");
    let command = Ok(Command::Show(ShowCommand::Ipv4));
    Ok((input, command))
}

fn command(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    alt((
        pounder_frequency,
        map(report, Ok),
    ))(input)
}

fn pounder_frequency(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    let (input, temp) = tag("pounder_frequency")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, value) = unsigned(input)?;
    let result = value.map(|freq| {
        Command::PounderFrequency { frequency: freq }
    });

    Ok((input, result))
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

