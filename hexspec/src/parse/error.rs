use nom::error::ErrorKind;
use nom_locate::LocatedSpan;

use super::Input;

/// Error due to parsing a [crate::HexSpec]
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    /// An odd number of hex codes is given in a line
    #[error("missing nibble")]
    MissingNibble,

    /// A relative label has been declared without an corresponding absolute label
    #[error("reative label with no associated absolute label")]
    UnanchoredRelativeLabel,

    /// Dupliate labels have been declared on the same level
    #[error("duplicate label used")]
    DuplicateLabel,

    /// Unable to parse a number in the spec
    #[error("failed to parse: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    /// A generic nom oriented error case
    #[error("nom error")]
    Nom(ErrorKind),
}

impl<'a, T> From<(LocatedSpan<&'a str, T>, ParseError)> for Contextualized<ParseError> {
    fn from((i, error): (LocatedSpan<&'a str, T>, ParseError)) -> Self {
        let context = i.into();
        Contextualized { error, context }
    }
}

impl<'a, T> From<(LocatedSpan<&'a str, T>, ErrorKind)> for Contextualized<ParseError> {
    fn from((i, ek): (LocatedSpan<&'a str, T>, ErrorKind)) -> Self {
        let context = i.into();
        let error = ParseError::Nom(ek);

        Contextualized { error, context }
    }
}

impl<'a> nom::error::ParseError<Input<'a>> for Contextualized<ParseError> {
    fn from_error_kind(input: Input<'a>, kind: ErrorKind) -> Self {
        (input, kind).into()
    }

    fn append(_: Input<'a>, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a, T> From<LocatedSpan<&'a str, T>> for Context {
    fn from(input: LocatedSpan<&'a str, T>) -> Context {
        // Shouldn't panic (plz for give if it does)
        let line = String::from_utf8(input.get_line_beginning().to_vec()).unwrap();
        let lineno = input.location_line() as usize;
        let column = input.get_utf8_column();
        let filename = None;

        Context {
            filename,
            line,
            lineno,
            column,
        }
    }
}

/// A type T with context in it's source file
#[derive(Debug, PartialEq, Eq)]
pub struct Contextualized<T> {
    /// The underlying error
    pub error: T,

    /// The context of where the error was thrown
    pub context: Context,
}

/// Context for where an error occured
#[derive(PartialEq, Eq, Debug)]
pub struct Context {
    /// Which file this context error came from
    pub filename: Option<String>,

    /// the content of the laine that caused an error
    pub line: String,

    /// The line number that caused an error
    pub lineno: usize,

    /// The column where the error happend
    pub column: usize,
}

impl<T> AsRef<T> for Contextualized<T> {
    fn as_ref(&self) -> &T {
        &self.error
    }
}

impl<T> std::ops::Deref for Contextualized<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.error
    }
}

impl<T> std::error::Error for Contextualized<T> where T: std::error::Error {}

impl<T> std::fmt::Display for Contextualized<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let filename = self.context.filename.as_deref().unwrap_or("%");
        let lineno = self.context.lineno.to_string();
        let line = self.context.line.as_str();

        let bump = filename.len() + lineno.len() + 1;
        let column = self.context.column + bump;

        writeln!(fmt, "{filename}:{lineno}:{line}")?;
        writeln!(fmt, "{:column$}^ {}", "", self.error, column = column)?;

        Ok(())
    }
}
