# codegenrs

> **Moving code-gen out of our `build.rs`**

[![codecov](https://codecov.io/gh/crate-ci/codegenrs/branch/master/graph/badge.svg)](https://codecov.io/gh/crate-ci/codegenrs)
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/codegenrs.svg)
[![Crates Status](https://img.shields.io/crates/v/codegenrs.svg)][Crates.io]

## About

`codegenrs` makes it easy to get rid of code-gen in `build.rs`, reducing build times for your crate and those that depend on it

This is done by:
- Creating a child `[[bin]]` crate that does code-gen using `codegenrs`
- Do one-time code-gen and commit it
- Run the `--check` step in your CI to ensure your code-gen is neither out of
  date or been human edited.

## Usage

`imperative` example:
- output: [`wordlist_codegen.rs`](https://github.com/crate-ci/imperative/blob/master/src/wordlist_codegen.rs)
- generator: [`imperative-codegen`](https://github.com/crate-ci/imperative/tree/master/tests/testsuite/codegen.rs)

## [Contribute](CONTRIBUTING.md)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/license/mit>)

at your option.

[Crates.io]: https://crates.io/crates/codegenrs
[Documentation]: https://docs.rs/codegenrs
