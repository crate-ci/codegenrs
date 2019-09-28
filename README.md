# codegenrs

> **Moving code-gen our of `build.rs`**

[![Build Status](https://dev.azure.com/crate-ci/crate-ci/_apis/build/status/codegenrs?branchName=master)](https://dev.azure.com/crate-ci/crate-ci/_build/latest?definitionId=7&branchName=master)
[![codecov](https://codecov.io/gh/crate-ci/codegenrs/branch/master/graph/badge.svg)](https://codecov.io/gh/crate-ci/codegenrs)
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/codegenrs.svg)
[![Crates Status](https://img.shields.io/crates/v/codegenrs.svg)](https://crates.io/crates/codegenrs)

## About

`codegenrs` makes it easy to get rid of code-gen in `build.rs`, reducing build times for your crate and those that depend on it

This is done by:
- Creating a child `[[bin]]` crate that does code-gen using `codegenrs`
- Do one-time code-gen and commit it
- Run the `--check` step in your CI to ensure your code-gen is neither out of
  date or been human edited.

## Usage

```toml
[dependencies]
codegenners = "0.1"
structopt = "0.3"
```

`imperative` example:
- output: [`wordlist_codegen.rs`](https://github.com/crate-ci/imperative/blob/master/src/wordlist_codegen.rs)
- generator: [`imperative-codegen`](https://github.com/crate-ci/imperative/tree/master/codegen)
- audit: [`azure-pipelines.yml`](https://github.com/crate-ci/imperative/blob/master/azure-pipelines.yml#L13)

## [Contribute](CONTRIBUTING.md)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

[Crates.io]: https://crates.io/crates/codegenrs
[Documentation]: https://docs.rs/codegenrs
