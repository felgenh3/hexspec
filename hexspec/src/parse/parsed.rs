//!
//! Parsed Value Attribution module
//!

use super::{error::Contextualized, Input, SpanResult};
use crate::parse::ParseError;
use nom::Parser;
use nom_locate::{position, LocatedSpan};

#[derive(PartialEq, Debug, Eq)]
pub struct Parsed<'a, T> {
    value: T,
    attribution: &'a str,
}

impl<'a, T> Copy for Parsed<'a, T> where T: Copy {}

impl<'a, T> Clone for Parsed<'a, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let attribution = self.attribution;
        let value = self.value.clone();

        Parsed { value, attribution }
    }
}

impl<'a, T> Parsed<'a, T> {
    pub fn spanned<P>(mut p: P) -> impl FnMut(Input<'a>) -> SpanResult<'a, T>
    where
        P: Parser<Input<'a>, T, Contextualized<ParseError>> + 'a,
    {
        move |input| {
            let (input, pos) = position(input)?;
            let start = input;
            let (input, value) = p.parse(input)?;
            let end = input;

            let context = unsafe { merge_contiguous_strings(&start, &end) };
            let attribution = context.trim();

            let diff = attribution.as_ptr() as usize - context.as_ptr() as usize;

            let parsed = Parsed { value, attribution };
            let span = unsafe { bump_offset(override_extra(pos, parsed), diff) };

            Ok((input, span))
        }
    }

    pub fn attribution(&self) -> &str {
        self.attribution
    }
}

impl<T> AsRef<T> for Parsed<'_, T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> std::ops::Deref for Parsed<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

/// Wildly unsafe ensure you know what you are doing
///
/// returns the string from the base pointer of start(including) up until(not including) the base pointer of end
unsafe fn merge_contiguous_strings<'a>(start: &'a str, end: &'a str) -> &'a str {
    let base = start.as_ptr();
    let begin = base as usize;
    let end = end.as_ptr() as usize;

    let len = end - begin;

    let slice = std::slice::from_raw_parts(base, len);
    std::str::from_utf8_unchecked(slice)
}

/// Prys open the LocatedSpan and moves the offset a head a certain amount
unsafe fn bump_offset<I, T>(input: LocatedSpan<I, T>, delta: usize) -> LocatedSpan<I, T>
where
    I: nom::AsBytes + Copy + Clone,
{
    LocatedSpan::new_from_raw_offset(
        input.location_offset() + delta,
        input.location_line(),
        *input.fragment(),
        input.extra,
    )
}

/// Reassigns the associated value with the LocatedSpan
unsafe fn override_extra<I, T, O>(input: LocatedSpan<I, T>, other: O) -> LocatedSpan<I, O>
where
    I: nom::AsBytes + Copy + Clone,
{
    LocatedSpan::new_from_raw_offset(
        input.location_offset(),
        input.location_line(),
        *input.fragment(),
        other,
    )
}
