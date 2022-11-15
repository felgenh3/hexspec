use clap::Parser;

mod bytes;
mod err;
mod fmt;
mod verify;

/// Format a hexspec file
#[derive(Parser)]
#[clap(name = "hexspec")]
enum Cli {
    Fmt(fmt::Fmt),
    Bytes(bytes::Bytes),
    Verify(verify::Verify),
}

fn main() -> eyre::Result<()> {
    err::SimpleHandler::install();
    let cli = Cli::parse();
    cli.run()
}

trait Runner {
    fn run(self) -> eyre::Result<()>;
}

impl Runner for Cli {
    fn run(self) -> eyre::Result<()> {
        match self {
            Cli::Fmt(fmt) => fmt.run(),
            Cli::Bytes(bytes) => bytes.run(),
            Cli::Verify(verify) => verify.run(),
        }
    }
}
