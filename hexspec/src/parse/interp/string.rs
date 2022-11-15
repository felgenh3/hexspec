use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag},
    character::complete::char,
    combinator::value,
};

use crate::parse::{Input, ParseResult};

pub fn parse_string(input: Input) -> ParseResult<Vec<u8>> {
    let (input, _) = char('"')(input)?;
    //let (input, content) = take_until("\"")(input)?;
    let (input, content) = escaped_transform(
        is_not("\"\\"),
        //take_till(|ch| ch == '"' || ch == '\\'),
        '\\',
        alt((
            value("\0", tag("0")),
            value("\"", tag("\"")),
            value("\n", tag("n")),
        )),
    )(input)?;
    let (input, _) = char('"')(input)?;

    Ok((input, content.as_bytes().to_vec()))
}

#[test]
fn test_string() {
    let base = "\"GEM1\"";
    let input = Input::new(base);

    let (rest, bytes) = parse_string(input).unwrap();
    assert_eq!(&bytes, &[0x47, 0x45, 0x4D, 0x31]);
    assert!(rest.is_empty());
}

#[test]
fn test_string_escapes() {
    let base = "\"GEM1\\0\\0\\0\\0\"";
    let input = Input::new(base);

    let (rest, bytes) = parse_string(input).unwrap();
    assert_eq!(&bytes, &[0x47, 0x45, 0x4D, 0x31, 0x00, 0x00, 0x00, 0x00]);
    assert!(rest.is_empty());
}
