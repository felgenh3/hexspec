use nom::{branch::alt, bytes::complete::tag, character::complete::one_of};

use crate::parse::{
    utils::{parse_group, parse_ty},
    Input, ParseResult,
};

pub fn parse_number(input: Input) -> ParseResult<Vec<u8>> {
    alt((parse_sized_oriented, parse_arch_oriented))(input)
}

fn parse_sized_oriented(input: Input) -> ParseResult<Vec<u8>> {
    let (input, endian) = one_of("bl")(input)?;
    let (input, size) = alt((tag("16"), tag("32"), tag("64")))(input)?;
    let (input, content) = parse_group(input)?;

    let endian = match endian {
        'b' => Endian::Big,
        'l' => Endian::Little,
        _ => unreachable!(),
    };

    let size = match size.as_ref() {
        "16" => Size::Word,
        "32" => Size::DWord,
        "64" => Size::QWord,
        _ => unreachable!(),
    };

    let neg = content.starts_with('-');
    imprint(input, content, size, neg, Some(endian))
}

fn parse_arch_oriented(input: Input) -> ParseResult<Vec<u8>> {
    let (input, size) = one_of("bwdq")(input)?;

    let size = match size {
        'b' => Size::Byte,
        'w' => Size::Word,
        'd' => Size::DWord,
        'q' => Size::QWord,

        _ => unreachable!(),
    };

    let (input, endian) = if size != Size::Byte {
        let (i, e) = one_of("lb")(input)?;
        (i, Some(e))
    } else {
        (input, None)
    };

    let endian = match endian {
        Some('l') => Some(Endian::Little),
        Some('b') => Some(Endian::Big),
        None => None,
        _ => unreachable!(),
    };

    let (input, content) = parse_group(input)?;

    let neg = content.starts_with('-');

    imprint(input, content, size, neg, endian)
}

#[derive(PartialEq)]
enum Size {
    Byte,
    Word,
    DWord,
    QWord,
}

enum Endian {
    Little,
    Big,
}

fn imprint<'a>(
    input: Input<'a>,
    content: Input<'a>,
    size: Size,
    neg: bool,
    endian: Option<Endian>,
) -> ParseResult<'a, Vec<u8>> {
    let mut buf = Vec::new();
    match (size, neg, endian) {
        (Size::Byte, false, None) => {
            let b = parse_ty(content)?;
            buf.push(b);
        }
        (Size::Byte, true, None) => {
            let ib = parse_ty::<i8>(content)?;
            buf.push(ib as u8);
        }

        (Size::Word, false, Some(endian)) => {
            let i = parse_ty::<u16>(content)?;

            match endian {
                Endian::Big => buf.extend(i.to_be_bytes()),
                Endian::Little => buf.extend(i.to_le_bytes()),
            }
        }
        (Size::Word, true, Some(endian)) => {
            let i = parse_ty::<i16>(content)?;

            match endian {
                Endian::Big => buf.extend(i.to_be_bytes()),
                Endian::Little => buf.extend(i.to_le_bytes()),
            }
        }
        (Size::DWord, false, Some(endian)) => {
            let i = parse_ty::<u32>(content)?;

            match endian {
                Endian::Big => buf.extend(i.to_be_bytes()),
                Endian::Little => buf.extend(i.to_le_bytes()),
            }
        }
        (Size::DWord, true, Some(endian)) => {
            let i = parse_ty::<i32>(content)?;

            match endian {
                Endian::Big => buf.extend(i.to_be_bytes()),
                Endian::Little => buf.extend(i.to_le_bytes()),
            }
        }
        (Size::QWord, false, Some(endian)) => {
            let i = parse_ty::<u64>(content)?;

            match endian {
                Endian::Big => buf.extend(i.to_be_bytes()),
                Endian::Little => buf.extend(i.to_le_bytes()),
            }
        }
        (Size::QWord, true, Some(endian)) => {
            let i = parse_ty::<i64>(content)?;

            match endian {
                Endian::Big => buf.extend(i.to_be_bytes()),
                Endian::Little => buf.extend(i.to_le_bytes()),
            }
        }

        _ => unreachable!(),
    }

    Ok((input, buf))
}

#[test]
fn test_number() {
    let base = "b(8)";
    let input = Input::new(base);

    let (rest, bytes) = parse_number(input).unwrap();

    assert_eq!(&bytes, &[0x08]);
    assert!(rest.is_empty());

    let base = "dl(4)";
    let input = Input::new(base);

    let (rest, bytes) = parse_number(input).unwrap();

    assert_eq!(&bytes, &[0x04, 0x00, 0x00, 0x00]);
    assert!(rest.is_empty());

    let base = "db(4)";
    let input = Input::new(base);

    let (rest, bytes) = parse_number(input).unwrap();

    assert_eq!(&bytes, &[0x00, 0x00, 0x00, 0x04]);
    assert!(rest.is_empty());
}

#[test]
fn test_size_number() {
    let base = "l32(4)";
    let input = Input::new(base);

    let (rest, bytes) = parse_number(input).unwrap();

    assert_eq!(&bytes, &[0x04, 0x00, 0x00, 0x00]);
    assert!(rest.is_empty());

    let base = "b32(4)";
    let input = Input::new(base);

    let (rest, bytes) = parse_number(input).unwrap();

    assert_eq!(&bytes, &[0x00, 0x00, 0x00, 0x04]);
    assert!(rest.is_empty());

    let base = "b64(4)";
    let input = Input::new(base);

    let (rest, bytes) = parse_number(input).unwrap();

    assert_eq!(&bytes, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04]);
    assert!(rest.is_empty());

    let base = "l16(4)";
    let input = Input::new(base);

    let (rest, bytes) = parse_number(input).unwrap();

    assert_eq!(&bytes, &[0x04, 0x00]);
    assert!(rest.is_empty());
}
