use clap::Parser;
use hexspec::HexSpec;
use std::fs::File;
use std::io::{Read, Write};

use std::path::PathBuf;

/// Output the byte buffer described by a hexspec
#[derive(Parser)]
pub struct Bytes {
    /// A hexspec file to read
    path: PathBuf,

    /// Where to output bytes
    out: Option<PathBuf>,
}

impl super::Runner for Bytes {
    fn run(self) -> eyre::Result<()> {
        let hexspec = if self.path.display().to_string() == "-" {
            let mut stdin = std::io::stdin();
            let mut buf = String::new();
            stdin.read_to_string(&mut buf)?;

            HexSpec::parse(&buf)?
        } else {
            HexSpec::load(self.path)?
        };

        if let Some(path) = self.out {
            let mut out = File::create(path)?;

            out.write_all(hexspec.as_ref())?;
        } else {
            let mut out = std::io::stdout();

            out.write_all(hexspec.as_ref())?;
        }

        Ok(())
    }
}
