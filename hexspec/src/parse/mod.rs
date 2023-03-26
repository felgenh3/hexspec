//!
//! A submodule to parse HexSpecs
//!

use self::parsed::Parsed;
use super::HexSpec;
use crate::Region;
use entities::parse_line;
use nom::IResult;
use nom_locate::LocatedSpan;
use std::collections::HashSet;

pub use error::{Context, Contextualized, ParseError};

mod content;
mod entities;
pub(crate) mod error;
mod interp;
mod parsed;
mod utils;

/// Keeps track of a type given some sort of text, given the text's position in it's source
pub type Span<'a, T> = LocatedSpan<&'a str, Parsed<'a, T>>;

/// A string wrapped with a cursor construct
type Input<'a> = LocatedSpan<&'a str>;
/// A Parse Result with a Span
type SpanResult<'a, T> = ParseResult<'a, Span<'a, T>>;
/// A Defacto Parser result for this library
type ParseResult<'a, T> = IResult<Input<'a>, T, Contextualized<ParseError>>;

/// A Label for a region of bytes
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Label<'a> {
    /// An absolute label
    ///
    /// This normally stands for high level regions,
    /// some of which can be broken down into sub regions
    ///
    /// Think the layers of the OSI networking stack.  IP/TCP/PHY
    Abs(&'a str),

    /// A relative label
    ///
    /// This normally stands for subparts of a regions
    ///
    /// Think fields in OSI, the dest address of IP/source port for TCP
    Rel(&'a str),
}

/// A Line intermediate Representation
#[derive(Debug, PartialEq, Eq)]
pub struct LineIR<'a> {
    /// An Optional label
    pub label: Option<Span<'a, Label<'a>>>,

    /// The bytes for this line
    pub bytes: Span<'a, Vec<u8>>,

    /// A line comment
    pub comment: Option<Span<'a, ()>>,
}

/// The immediate representation for a HexSpec
#[derive(Debug, PartialEq, Eq)]
pub struct HexSpecIR<'a> {
    /// The lines of source associated in the given file
    pub lines: Vec<LineIR<'a>>,
}

impl HexSpecIR<'_> {
    /// Parse the HexSpecIR
    pub fn parse(input: &str) -> Result<HexSpecIR, Contextualized<ParseError>> {
        let input = Input::new(input);
        let (_, spec) = nom::combinator::all_consuming(HexSpecIR::parse_nom)(input).map_err(
            |err| match err {
                nom::Err::Incomplete(_) => unreachable!(),
                nom::Err::Error(e) => e,
                nom::Err::Failure(e) => e,
            },
        )?;

        Ok(spec)
    }

    fn parse_nom(mut input: Input) -> IResult<Input, HexSpecIR, Contextualized<ParseError>> {
        let mut lines = Vec::new();

        while !input.is_empty() {
            let (next, line) = parse_line(input)?;
            lines.push(line);

            input = next;
        }

        Ok((input, HexSpecIR { lines }))
    }

    /// Generate a spec out of the immediate representation
    pub fn to_spec(self) -> Result<HexSpec, Contextualized<ParseError>> {
        let mut buf = Vec::new();
        let mut regions = Vec::new();

        let mut cur = 0;
        let mut tl_labels = HashSet::new();
        let mut abs_label_partial: Option<(&str, usize, HashSet<&str>)> = None;
        let mut rel_label_partial: Option<(&str, usize)> = None;

        for line in self.lines {
            //let LineIR { label, bytes, .. } = line;
            //let (next, InputLine { label, content, .. }) = parse_line(input)?;

            if let Some(input) = line.label {
                match input.extra.as_ref() {
                    Label::Abs(name) => {
                        if name != &"_" && tl_labels.contains(*name) {
                            return Err((input, ParseError::DuplicateLabel).into());
                        }

                        tl_labels.insert(*name);

                        if let Some((label, start, _)) = &mut abs_label_partial {
                            let start = *start;

                            if let Some((rel_label, start)) = rel_label_partial {
                                let label = format!("{}.{}", label, rel_label);
                                let end = cur;

                                regions.push(Region { label, start, end });
                                rel_label_partial = None;
                            }
                            let label = label.to_string();
                            let end = cur;

                            regions.push(Region { label, start, end });
                        }

                        match *name {
                            "_" => abs_label_partial = None,
                            _ => abs_label_partial = Some((name, cur, HashSet::new())),
                        }
                    }

                    Label::Rel(name) => {
                        if let Some((abs_label, _, rel_labels)) = &mut abs_label_partial {
                            if name != &"_" && rel_labels.contains(name) {
                                return Err((input, ParseError::DuplicateLabel).into());
                            }

                            rel_labels.insert(name);

                            if let Some((rel_label, start)) = rel_label_partial {
                                let label = format!("{}.{}", abs_label, rel_label);
                                let end = cur;

                                regions.push(Region { label, start, end });
                            }

                            match *name {
                                "_" => rel_label_partial = None,
                                _ => rel_label_partial = Some((name, cur)),
                            }
                        } else {
                            return Err((input, ParseError::UnanchoredRelativeLabel).into());
                        }
                    }
                }
            }

            let bytes = line.bytes.extra.as_slice();
            cur += bytes.len();
            buf.extend(bytes);
        }

        if let Some((label, start, _)) = abs_label_partial {
            if let Some((rel_label, start)) = rel_label_partial {
                let label = format!("{}.{}", label, rel_label);
                let end = cur;

                regions.push(Region { label, start, end });
            }

            let label = label.to_string();
            let end = cur;

            regions.push(Region { label, start, end });
        }

        Ok(HexSpec { buf, regions })
    }
}

#[cfg(feature = "parse")]
impl<'a> LineIR<'a> {
    /// check whether a line contains any syntatical element
    pub fn is_empty(&self) -> bool {
        self.label.is_none() && self.bytes.extra.as_ref().is_empty() && self.comment.is_none()
    }

    /// check if a line is only a comment
    pub fn is_comment(&self) -> bool {
        self.label.is_none() && self.bytes.extra.as_ref().is_empty() && self.comment.is_some()
    }

    /// Get the outright label
    pub fn label(&self) -> Option<&Label<'a>> {
        self.label.as_ref().map(|span| span.extra.as_ref())
    }

    /// Text that generated the label
    pub fn label_txt(&self) -> Option<&str> {
        self.label.as_ref().map(|span| span.extra.attribution())
    }

    /// Get the parsed bytes
    pub fn bytes(&self) -> &[u8] {
        self.bytes.extra.as_slice()
    }

    /// Text that generated the bytes
    pub fn bytes_txt(&self) -> &str {
        self.bytes.extra.attribution()
    }

    /// Text that is the comment
    pub fn comment_txt(&self) -> Option<&str> {
        self.comment.as_ref().map(|span| span.extra.attribution())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_ir_empty() {
        let spec = HexSpecIR::parse(
            r"


",
        );

        assert!(spec.is_ok());
    }

    #[test]
    fn parse_line() {
        let spec = HexSpecIR::parse(
            r"
sbe: ABCD ; Hello
",
        );

        assert!(spec.is_ok());
    }
}
