use nom::bytes::complete::tag;

use crate::parse::{
    utils::{parse_group, parse_ty},
    Input, ParseResult,
};

pub fn parse_zero_pad(input: Input) -> ParseResult<Vec<u8>> {
    let (input, _) = tag("zp")(input)?;

    let (input, content) = parse_group(input)?;

    let amount = parse_ty::<usize>(content)?;
    let buf = vec![0; amount];

    Ok((input, buf))
}
