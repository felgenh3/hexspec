//!
//! A utility library for parsers in hexspec
//!

use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::one_of,
};

use super::{error::Contextualized, Input, ParseError, ParseResult};

/// Parse () or [] or {} and return the characters within (must have characters within)
pub fn parse_group(input: Input) -> ParseResult<Input> {
    let (input, start_bound) = one_of("([{")(input)?;
    let end_bound = match start_bound {
        '(' => ")",
        '[' => "]",
        '{' => "}",
        _ => unreachable!(),
    };

    let (input, content) = take_until1(end_bound)(input)?;
    let (input, _) = tag(end_bound)(input)?;

    Ok((input, content))
}

pub fn parse_ty<T>(content: Input) -> Result<T, nom::Err<Contextualized<crate::ParseError>>>
where
    T: FromStr<Err = std::num::ParseIntError>,
{
    match T::from_str(&content) {
        Ok(t) => Ok(t),
        Err(e) => Err(nom::Err::Error((content, ParseError::ParseInt(e)).into())),
    }
}
