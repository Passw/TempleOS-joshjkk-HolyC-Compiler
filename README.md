<h1 align="center">HolyC Compiler</h1>

<div align="center">
       	<a href="#about">About</a>
  <span> • </span>
	      <a href="#installation">Installation</a>
  <span> • </span>
        <a href="#usage">Usage</a>
   <span> • </span>
        <a href="#contributing">Contributing</a>
  <p></p>
</div> 

## About

This project has been created to carry on the legacy of TempleOS. When finished, this compiler will allow the user to compile HolyC files into x86_64 assembly, to then compile with the [NASM Assembler](https://en.wikipedia.org/wiki/Netwide_Assembler).

### Disclaimer

This project is still in development, and **is not intended to be used at its current state.**

## Installation

This compiler is written in rust for safety reasons, so rust and cargo are needed. If you don't have those installed, go install them from the [rust-lang website](https://doc.rust-lang.org/cargo/getting-started/installation.html).

## Usage

After installing the requirements, run the following command:

``` bash
cargo run <file_to_compile>
```

An assembly file should be generated in the directory of the source file to compile.

### Assembling and linking

Use the [NASM Assembler](https://en.wikipedia.org/wiki/Netwide_Assembler) for assembling the output assembly file. You can do this with the following command:

``` bash
nasm -f elf64 file_to_assemble.asm -o file_to_assemble.o
ld file_to_assemble.o -o file_to_assemble.out
```

> '-f elf64' formats the output object file to be 64-bit

## Contributing

Any compiler enthusiasts are welcome to contribute to this compiler, this compiler is free and community driven.

## License

HolyC Compiler is under an [Apache 2.0](./LICENSE) license.
