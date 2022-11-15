use super::{Input, ParseResult};
use nom::{branch::alt, character::complete::char, combinator::cut};

mod number;
mod string;
mod zeros;

pub fn parse_interp(input: Input) -> ParseResult<Vec<u8>> {
    let (input, _) = char('@')(input)?;
    let (input, bytes) = cut(alt((
        string::parse_string,
        number::parse_number,
        zeros::parse_zero_pad,
    )))(input)?;

    Ok((input, bytes))
}

#[test]
fn test_interp() {
    let base = "@qb(34)";
    let input = Input::new(base);

    let (rest, bytes) = parse_interp(input).unwrap();
    assert_eq!(&bytes, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22]);
    assert!(rest.is_empty());

    let base = "@\"Hello\"";
    let input = Input::new(base);

    let (rest, bytes) = parse_interp(input).unwrap();
    assert_eq!(&bytes, b"Hello");
    assert!(rest.is_empty());
}
