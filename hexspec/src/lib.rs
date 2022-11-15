//!
//! A library/language for testing hex based byte buffers
//!

#![warn(missing_docs)]

#[cfg(test)]
mod test;

mod fmt;
pub mod parse;

pub use parse::{Context, Contextualized, ParseError};
use std::path::Path;

/// An error describing how loading a HexSpec can go wrong
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    /// Failure relating to Operating System IO
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    /// Failure in parsing a HexSpec
    #[error("unable to parse hexspec:\n{0}")]
    Parse(#[from] Contextualized<ParseError>),
}

/// A Spec describing what bytes should be in a buffer
#[derive(Debug, PartialEq, Eq)]
pub struct HexSpec {
    buf: Vec<u8>,
    regions: Vec<Region>,
}

/// A sub range of a HexSpec which is given a name
#[derive(Debug, PartialEq, Eq)]
pub struct Region {
    label: String,
    start: usize,
    end: usize,
}

impl AsRef<[u8]> for HexSpec {
    fn as_ref(&self) -> &[u8] {
        self.buf.as_ref()
    }
}

impl std::ops::Deref for HexSpec {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.buf.as_ref()
    }
}

impl std::ops::Index<&str> for HexSpec {
    type Output = Region;

    fn index(&self, index: &str) -> &Region {
        for r in &self.regions {
            if r.label == index {
                return r;
            }
        }

        panic!("no region named \"{index}\" found");
    }
}

impl HexSpec {
    /// Load a [HexSpec] from a file
    pub fn load(p: impl AsRef<Path>) -> Result<HexSpec, LoadError> {
        let content = std::fs::read_to_string(p.as_ref())?;
        HexSpec::parse(content)
    }

    /// Parse a [HexSpec] from a string
    pub fn parse(input: impl AsRef<str>) -> Result<HexSpec, LoadError> {
        let ir = parse::HexSpecIR::parse(input.as_ref())?;
        let spec = ir.to_spec()?;

        Ok(spec)
    }

    /// Iterator over regions in the [HexSpec]
    pub fn regions(&self) -> Regions {
        Regions {
            cur: 0,
            regions: &self.regions,
        }
    }

    /// look through all discrepancies and panic if any exist
    pub fn validate<'e, 'a>(&'e self, buf: &'a [u8]) -> Result<(), Vec<Discrepancy<'e, 'a>>> {
        let d = self.discrepancies(buf);

        if !d.is_empty() {
            Err(d)
        } else {
            Ok(())
        }
    }

    /// Lists discrepancies between the HexSpec and a given buffer
    pub fn discrepancies<'e, 'a>(&'e self, buf: &'a [u8]) -> Vec<Discrepancy<'e, 'a>> {
        let mut discrepancies = Vec::new();

        for region in &self.leaves() {
            if region.contains(buf) {
                let expect = region.subsection(&self.buf);
                let actual = region.subsection(buf);

                if actual != expect {
                    discrepancies.push(Discrepancy::Changed {
                        region,
                        actual,
                        expect,
                    });
                }
            } else {
                discrepancies.push(Discrepancy::MissingRegion { region })
            }
        }

        discrepancies
    }

    fn leaves(&self) -> Vec<&Region> {
        let mut set = std::collections::HashSet::new();
        let mut res = Vec::new();

        for region in &self.regions {
            if let Some((abs, _)) = region.label.split_once('.') {
                set.insert(abs);
            }

            if !set.contains(&region.label.as_str()) {
                res.push(region);
            }
        }

        res
    }
}

impl Region {
    /// Returns whether a region fits into a given buf
    ///
    /// ```
    /// # use hexspec::*;
    /// # use std::path::PathBuf;
    /// let spec = HexSpec::load(PathBuf::from("specs/example.hexspec")).unwrap();
    /// let region = &spec["sofh.sig"];
    ///
    /// assert!(region.contains(&[00, 00, 00, 0x3F, 0xEB, 0x50, 0x37, 0x00]));
    /// assert!(!region.contains(&[]));
    /// ```
    pub fn contains(&self, buf: &[u8]) -> bool {
        self.end <= buf.len()
    }

    /// Get's a a subsection of a byte buffer that represents it's value in a buffer
    ///
    /// ```
    /// # use hexspec::*;
    /// # use std::path::PathBuf;
    /// let spec = HexSpec::load(PathBuf::from("specs/example.hexspec")).unwrap();
    /// let region = &spec["sofh.sig"];
    ///
    /// assert_eq!(region.subsection(&[00, 00, 00, 0x3F, 0xEB, 0x50, 0x37, 0x00]), &[0xEB, 0x50]);
    /// ```
    pub fn subsection<'a>(&self, buf: &'a [u8]) -> &'a [u8] {
        &buf[self.start..self.end]
    }
}

/// Embodies a difference of the expectation of the byte buffer and what is actually there
#[derive(Debug, PartialEq, Eq)]
pub enum Discrepancy<'expectation, 'actual> {
    /// The given buffer does not have enough length to be tested
    MissingRegion {
        /// The region being tested
        region: &'expectation Region,
    },

    /// The given buffer did not have the expected byte values
    Changed {
        /// Expected byte values
        expect: &'expectation [u8],

        /// Actual byte values
        actual: &'actual [u8],

        /// The region being tested
        region: &'expectation Region,
    },
}

/// An Iterator over regions of a [HexSpec]
pub struct Regions<'a> {
    cur: usize,
    regions: &'a [Region],
}

impl<'a> Iterator for Regions<'a> {
    type Item = &'a Region;

    fn next(&mut self) -> Option<&'a Region> {
        if self.cur >= self.regions.len() {
            return None;
        }

        let cur = self.cur;
        self.cur += 1;

        Some(&self.regions[cur])
    }
}

/// An error when validating a spec
#[derive(Debug)]
pub struct ValidationError<'spec, 'bytes> {
    discrepancies: Vec<Discrepancy<'spec, 'bytes>>,
}

impl<'spec, 'bytes> std::error::Error for ValidationError<'spec, 'bytes> {}
