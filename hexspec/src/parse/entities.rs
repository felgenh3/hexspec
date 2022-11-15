use nom::{
    bytes::complete::{take_till, take_while1},
    character::complete::{char, newline, one_of, space0},
    combinator::opt,
};

use super::{content::parse_content, parsed::Parsed, Input, Label, LineIR, ParseResult};

fn parse_label(input: Input) -> ParseResult<Label> {
    let (input, _) = space0(input)?;
    let (input, is_rel) = opt(char('.'))(input)?;
    let (input, name) = take_while1(|ch: char| ch.is_alphanumeric() || ch == '_')(input)?;
    let (input, _) = char(':')(input)?;

    let label = if is_rel.is_some() {
        Label::Rel(name.as_ref())
    } else {
        Label::Abs(name.as_ref())
    };

    Ok((input, label))
}

fn parse_bytes(input: Input) -> ParseResult<Vec<u8>> {
    parse_content(input)
}

fn parse_comment(input: Input) -> ParseResult<()> {
    let (input, _) = one_of(";#")(input)?;
    let (input, _) = take_till(|ch| ch == '\n' || ch == '\r')(input)?;

    Ok((input, ()))
}

pub fn parse_line(input: Input) -> ParseResult<LineIR> {
    let (input, label) = opt(Parsed::spanned(parse_label))(input)?;
    let (input, bytes) = Parsed::spanned(parse_bytes)(input)?;
    let (input, comment) = opt(Parsed::spanned(parse_comment))(input)?;

    let (input, _) = opt(newline)(input)?;

    Ok((
        input,
        LineIR {
            label,
            bytes,
            comment,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_just_label() {
        let input = "ip:";
        let input = Input::new(input);

        let (_, label) = Parsed::spanned(parse_label)(input).unwrap();
        let label = label.extra;
        assert_eq!(label.as_ref(), &Label::Abs("ip"));
    }

    #[test]
    fn parse_label_attribution() {
        let input = "  ip:";
        let input = Input::new(input);

        let (_, label) = Parsed::spanned(parse_label)(input).unwrap();
        let label = label.extra;
        assert_eq!(label.attribution(), "ip:");
    }

    #[test]
    fn parse_label_position() {
        let input = "  ip:";
        let input = Input::new(input);

        let (_, label) = Parsed::spanned(parse_label)(input).unwrap();
        assert_eq!(label.location_offset(), 2);
    }
}
