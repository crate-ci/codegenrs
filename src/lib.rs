//! # codegenrs
//!
//! > **Moving code-gen our of `build.rs`**
//!
//! ## About
//!
//! `codegenrs` makes it easy to get rid of code-gen in `build.rs`, reducing your
//! and dependents' build times.  This is done by:
//! - Creating a child `[[bin]]` crate that does code-gen using `codegenrs`
//! - Do one-time code-gen and commit it
//! - Run the `--check` step in your CI to ensure your code-gen is neither out of
//!   date or been human edited.
//!
//!## Usage
//!
//!`imperative` example:
//! - output: [`wordlist_codegen.rs`](https://github.com/crate-ci/imperative/blob/master/src/wordlist_codegen.rs)
//! - generator: [`imperative-codegen`](https://github.com/crate-ci/imperative/tree/master/tests/codegen.rs)

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(clippy::branches_sharing_code)]

use std::io::Write;

#[cfg(feature = "clap")]
use clap::Args;

/// CLI arguments to `flatten` into your args
///
/// ## Example
///
/// ```rust
/// #[derive(clap::Parser)]
/// struct Args{
///    #[arg(short('i'), long)]
///    input: std::path::PathBuf,
///    #[command(flatten)]
///    codegen: codegenrs::CodeGenArgs,
/// }
/// ```
#[cfg(feature = "clap")]
#[derive(Debug, Args)]
pub struct CodeGenArgs {
    #[arg(short('o'), long)]
    output: std::path::PathBuf,

    #[arg(long)]
    check: bool,
}

#[cfg(feature = "clap")]
impl CodeGenArgs {
    /// Write or verify code-genned text.
    pub fn write_str(&self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        write_str(content, &self.output, self.check)
    }
}

/// Write or verify code-genned text.
///
/// See `CodeGenArgs` for `clap` integration.
pub fn write_str(
    content: &str,
    output: &std::path::Path,
    check: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if check {
        let content: String = normalize_line_endings::normalized(content.chars()).collect();

        let actual = std::fs::read_to_string(output)?;
        let actual: String = normalize_line_endings::normalized(actual.chars()).collect();

        if content == actual {
            println!("Success");
        } else {
            // `difference` will allocation a `Vec` with  N*M elements.
            let allocation = content.lines().count() * actual.lines().count();
            if 1_000_000_000 < allocation {
                eprintln!("{} out of sync (too big to diff)", output.display());
                return Err(Box::new(CodeGenError));
            } else {
                let changeset = difference::Changeset::new(&actual, &content, "\n");
                assert_ne!(changeset.distance, 0);
                eprintln!("{} out of sync:", output.display());
                eprintln!("{changeset}");
                return Err(Box::new(CodeGenError));
            }
        }
    } else {
        let mut file = std::io::BufWriter::new(std::fs::File::create(output)?);
        write!(file, "{content}")?;
    }

    Ok(())
}

/// CLI arguments to `flatten` into your args
///
/// ## Example
///
/// ```rust
/// #[derive(clap::Parser)]
/// struct Args{
///    #[arg(short('i'), long)]
///    input: std::path::PathBuf,
///    #[command(flatten)]
///    codegen: codegenrs::CodeGenArgs,
///    #[command(flatten)]
///    rustfmt: codegenrs::RustfmtArgs,
/// }
/// ```
#[cfg(feature = "clap")]
#[derive(Debug, Args)]
pub struct RustfmtArgs {
    #[arg(long)]
    rustfmt_config: Option<std::path::PathBuf>,
}

#[cfg(feature = "clap")]
impl RustfmtArgs {
    /// Write or verify code-genned text.
    pub fn reformat(
        &self,
        text: impl std::fmt::Display,
    ) -> Result<String, Box<dyn std::error::Error>> {
        rustfmt(text, self.rustfmt_config.as_deref())
    }
}

/// Run `rustfmt` on an in-memory string
pub fn rustfmt(
    text: impl std::fmt::Display,
    config: Option<&std::path::Path>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut rustfmt = std::process::Command::new("rustfmt");
    rustfmt
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());
    if let Some(config) = config {
        rustfmt.arg("--config-path").arg(config);
    }
    let mut rustfmt = rustfmt.spawn()?;
    write!(rustfmt.stdin.take().unwrap(), "{text}")?;
    let output = rustfmt.wait_with_output()?;
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
}

#[derive(Copy, Clone, Debug, derive_more::Display)]
#[display(fmt = "Code-gen failed")]
struct CodeGenError;

impl std::error::Error for CodeGenError {}
