use super::Runner;
use clap::Parser;
use eyre::Context;
use hexspec::parse::{HexSpecIR, LineIR};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Format a hexspec file
#[derive(Parser)]
pub struct Fmt {
    /// Write back to the file
    #[clap(short, long)]
    write: bool,

    /// Files to format (stdin if no files exist)
    files: Vec<PathBuf>,
}

impl Runner for Fmt {
    fn run(self) -> eyre::Result<()> {
        if self.files.is_empty() {
            if self.write {
                eyre::bail!("can not write back to standard in");
            }

            let mut content = String::new();
            std::io::stdin()
                .read_to_string(&mut content)
                .wrap_err("failed to read stdin")?;

            let ir = HexSpecIR::parse(&content).wrap_err("unable to identify hexspec")?;

            let formatted = format(&ir);

            std::io::stdout()
                .write_all(formatted.as_bytes())
                .wrap_err("failed to write to stdout")?;
        } else if self.write {
            let mut all_worked = true;
            for path in self.files {
                if let Err(err) = write_back(path.as_ref()) {
                    all_worked = false;
                    eprintln!("Failed to format {}: {}", path.display(), err);
                }
            }

            if !all_worked {
                eyre::bail!("failed to format some files");
            }
        } else {
            for path in self.files {
                let content = std::fs::read_to_string(&path)?;

                let ir = HexSpecIR::parse(&content).wrap_err("unable to identify hexspec")?;

                let formatted = format(&ir);

                std::io::stdout()
                    .write_all(formatted.as_bytes())
                    .wrap_err("failed to write to stdout")?;
            }
        }

        Ok(())
    }
}

fn write_back(path: &Path) -> eyre::Result<()> {
    let content = std::fs::read_to_string(&path)?;

    let ir = HexSpecIR::parse(&content).wrap_err("unable to identify hexspec")?;

    let formatted = format(&ir);

    let mut file = File::open(path)?;

    file.write_all(formatted.as_bytes())?;

    Ok(())
}

fn format(input: &HexSpecIR) -> String {
    let mut out = String::new();

    for section in sections(input) {
        let mut colon = None::<usize>;
        let mut bytes_len = None::<usize>;

        for line in &section {
            colon = max(colon, line.label_txt().map(|s| s.len()));
            bytes_len = max(bytes_len, Some(line.bytes_txt().len()));
        }

        let mut comments = true;

        for line in section {
            comments &= line.is_comment();

            if comments {
                if let Some(comment) = line.comment_txt() {
                    out.push_str(comment);
                    out.push('\n');
                }

                continue;
            }

            if let Some(len) = colon {
                let real_space_len = if let Some(label) = line.label_txt() {
                    len - label.len()
                } else {
                    len
                };

                for _ in 0..real_space_len {
                    out.push(' ');
                }
            }

            let byte_len = line.bytes_txt().len();
            if let Some(label) = line.label_txt() {
                out.push_str(label);
            }

            if byte_len != 0 && colon.is_some() {
                out.push(' ');
            }

            out.push_str(line.bytes_txt());

            if let Some(comment) = line.comment_txt() {
                if byte_len != 0 {
                    if let Some(len) = bytes_len {
                        for _ in 0..len - byte_len {
                            out.push(' ');
                        }
                    }
                }

                out.push(' ');
                out.push_str(comment);
            }

            out.push('\n');
        }

        out.push('\n');
    }

    out
}

fn sections<'a>(input: &'a HexSpecIR) -> Vec<Vec<&'a LineIR<'a>>> {
    let mut res = Vec::new();
    let mut cur = Vec::new();

    for line in &input.lines {
        if line.is_empty() {
            if !cur.is_empty() {
                res.push(cur);
                cur = Vec::new();
            }
        } else {
            cur.push(line);
        }
    }

    if !cur.is_empty() {
        res.push(cur);
    }

    res
}

fn max(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}
