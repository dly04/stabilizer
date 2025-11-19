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
    // Ipv4(Ipv4Config),
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
    preceded(
        tag("report"),
        // `report` - Report once
        value(Command::Show(ShowCommand::Input), end),
    )(input)
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
    // Dummy 实现：总是返回显示 IPv4 命令
    let command = Ok(Command::Show(ShowCommand::Ipv4));
    Ok((input, command))
}

fn command(input: &[u8]) -> IResult<&[u8], Result<Command, Error>> {
    alt((
        map(report, Ok),
        ipv4,
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

