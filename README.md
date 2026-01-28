[![Apache License](https://img.shields.io/github/license/jonaspleyer/approx-derive?style=flat-square)](https://opensource.org/license/apache-2-0)
[![Test](https://img.shields.io/github/actions/workflow/status/jonaspleyer/approx-derive/test.yml?label=Test&style=flat-square)](https://github.com/jonaspleyer/approx-derive/actions)
[![Crate](https://img.shields.io/crates/v/approx-derive.svg?style=flat-square)](https://crates.io/crates/approx-derive)
![Crates.io Total Downloads](https://img.shields.io/crates/d/approx-derive?style=flat-square)
![Codecov](https://img.shields.io/codecov/c/github/jonaspleyer/approx-derive?style=flat-square)

# approx_derive

`approx-derive` extends the popular [`approx`](https://docs.rs/approx/latest/approx/)
by two derive macros `AbsDiffEq` and `RelativeEq`.
This allows to quickly derive implementations for comparing these types with the macros provided in
[`approx`](https://docs.rs/approx/latest/approx/) crate.

# Documentation
Visit [docs.rs](https://docs.rs/approx-derive/latest/approx_derive/) to view the documentation.

# Note
This crate was designed to be operated with the `approx` and `approxim` crates.
With the archiving of `approxim`, we will no longer support this crate actively and version `0.2.8`
remains the last supported compatible version.
