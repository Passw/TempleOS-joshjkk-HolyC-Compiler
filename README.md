# HolyC Compiler

A compiler for HolyC, outside of TempleOS.

[![Build](https://github.com/joshjkk/HolyC-Compiler/actions/workflows/Build.yml/badge.svg?branch=master)](https://github.com/joshjkk/HolyC-Compiler/actions/workflows/Build.yml)

## Disclaimer

This project is still in development, and is not intended to be used at its current state. Please don't use this at the moment.

## About

I have made a repo for [exporting TempleOS files](https://github.com/joshjkk/TempleOS-Mounter), so why not make a HolyC compiler to go with it? Obviously the graphics library is missing, but hopefully the standard library will be implemented when finished.

## Installation and Usage

First, make sure you have rust and cargo installed. If not, go install it from the [rust-lang website](https://doc.rust-lang.org/cargo/getting-started/installation.html).

After installing and cloning, run the following command:

``` bash
cargo run <file_to_compile>
```

> As of now, the file is not compiled, only lexed and parsed. Compilation will be implemented in upcoming updates.

## License

HolyC Compiler is under an Apache 2.0 license.
