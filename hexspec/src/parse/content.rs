use nom::bytes::complete::is_a;
use nom::multi::many0;
use nom::{branch::alt, character::complete::space1};

use super::{interp::parse_interp, Input, ParseError, ParseResult};

pub fn parse_content(input: Input) -> ParseResult<Vec<u8>> {
    let mut buf = Vec::new();

    let (input, buffers) = many0(alt((parse_blanks, parse_interp, parse_hexcodes)))(input)?;

    for buffer in buffers {
        buf.extend(buffer);
    }

    Ok((input, buf))
}

pub fn parse_hexcodes(input: Input) -> ParseResult<Vec<u8>> {
    let mut buf = Vec::new();
    let mut nibble = None;
    let (input, content) = is_a("0123456789ABCDEFabcdef")(input)?;

    for ch in content.chars() {
        let val = match ch {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => ch as u8 - b'0',
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' => ch as u8 - b'a' + 10,
            'A' | 'B' | 'C' | 'D' | 'E' | 'F' => ch as u8 - b'A' + 10,

            _ => unreachable!(),
        };

        match nibble {
            Some(upper) => {
                buf.push((upper << 4) | val);

                nibble = None;
            }

            None => {
                nibble = Some(val);
            }
        }
    }

    if nibble.is_some() {
        return Err(nom::Err::Failure((input, ParseError::MissingNibble).into()));
    }

    Ok((input, buf))
}

pub fn parse_blanks(input: Input) -> ParseResult<Vec<u8>> {
    let (input, _) = space1(input)?;

    Ok((input, Default::default()))
}

#[test]
fn test_hexcodes() {
    let base = "ABCD";
    let input = Input::new(base);

    let (_, bytes) = parse_hexcodes(input).unwrap();

    assert_eq!(&bytes, &[0xAB, 0xCD]);
}

#[test]
fn test_blanks() {
    let base = "    ";
    let input = Input::new(base);

    let (rest, bytes) = parse_blanks(input).unwrap();

    assert_eq!(&bytes, &[]);
    assert!(rest.is_empty());
}

#[test]
fn test_compose() {
    let base = "@qb(34) EB50";
    let input = Input::new(base);

    let (rest, bytes) = parse_content(input).unwrap();
    assert_eq!(
        &bytes,
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0xEB, 0x50]
    );
    assert!(rest.is_empty());
}

/// At some point (before this commit) when an invalid interp was used, parse_content would return
/// an Ok and not eat bytes meant for content.
///
/// This introduced an infinite loop in parsing HexSpecs, (NO GOOD)
#[test]
fn test_invalid_interp() {
    let base = "@xyz(34) EB50";
    let input = Input::new(base);

    parse_content(input).unwrap_err();
}
