use std::fmt::{Display, Formatter, Result};

use crate::{Discrepancy, ValidationError};

impl<'e, 'a> Display for Discrepancy<'e, 'a> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            Discrepancy::MissingRegion { region } => {
                let label = &region.label;
                let end = region.end;
                write!(
                    fmt,
                    "region \"{label}\" missing, expected a buffer with at least {end} bytes"
                )
            }

            Discrepancy::Changed {
                expect,
                actual,
                region,
            } => {
                let label = &region.label;
                let expect = HexRepr(expect);
                let actual = HexRepr(actual);
                write!(fmt, "region \"{label}\" expected {expect} but got {actual}")
            }
        }
    }
}

impl<'spec, 'bytes> Display for ValidationError<'spec, 'bytes> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for d in &self.discrepancies {
            writeln!(f, "{}", d)?;
        }

        Ok(())
    }
}

struct HexRepr<'a>(&'a [u8]);

impl<'a> Display for HexRepr<'a> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        let mut buffer = String::new();

        for byte in self.0 {
            let hi = (byte & 0xf0) >> 4;
            let lo = byte & 0x0f;

            buffer.push(match hi {
                dig if dig < 10 => (b'0' + dig) as char,
                dig => (b'A' + dig - 10) as char,
            });

            buffer.push(match lo {
                dig if dig < 10 => (b'0' + dig) as char,
                dig => (b'A' + dig - 10) as char,
            });
        }

        write!(fmt, "0x{buffer}")
    }
}
