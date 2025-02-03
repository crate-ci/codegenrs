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
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

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

        if content != actual {
            // `difference` will allocation a `Vec` with  N*M elements.
            let allocation = content.lines().count() * actual.lines().count();
            if 1_000_000_000 < allocation {
                return Err(Box::new(CodeGenError {
                    message: format!("{} out of sync (too big to diff)", output.display()),
                }));
            } else {
                let changeset = difference::Changeset::new(&actual, &content, "\n");
                assert_ne!(changeset.distance, 0);
                return Err(Box::new(CodeGenError {
                    message: format!("{} out of sync:\n{changeset}", output.display()),
                }));
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
    let mut rustfmt = rustfmt
        .spawn()
        .map_err(|err| format!("could not run `rustfmt`: {err}"))?;
    write!(
        rustfmt
            .stdin
            .take()
            .expect("rustfmt was configured with stdin"),
        "{text}"
    )?;
    let output = rustfmt.wait_with_output()?;
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
}

#[derive(Clone, Debug)]
struct CodeGenError {
    message: String,
}

impl std::error::Error for CodeGenError {}

impl std::fmt::Display for CodeGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}
