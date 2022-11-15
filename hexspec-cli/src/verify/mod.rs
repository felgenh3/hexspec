use clap::Parser;
use hexspec::HexSpec;
use std::io::Read;

use std::process::exit;
use std::{fs::File, path::PathBuf};

/// Verify a byte buffer
#[derive(Parser)]
pub struct Verify {
    /// A hexspec file
    filename: PathBuf,

    /// Byte input file
    input: Option<PathBuf>,
}

impl super::Runner for Verify {
    fn run(self) -> eyre::Result<()> {
        let hexspec = HexSpec::load(&self.filename)?;

        let mut bytes = Vec::new();

        if let Some(path) = &self.input {
            let mut file = File::open(path)?;
            file.read_to_end(&mut bytes)?;
        } else {
            let mut stdin = std::io::stdin();
            stdin.read_to_end(&mut bytes)?;
        }

        let discrepancies = hexspec.discrepancies(&bytes);

        let fail = !discrepancies.is_empty();

        for discrep in discrepancies {
            println!("{discrep}");
        }

        if fail {
            exit(1);
        }

        Ok(())
    }
}
