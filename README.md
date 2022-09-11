<h1 align="center">The Extendable Assembler Library</h1>
<h3 align="center">libxas, a Bell-style, open-source, fast, Rust-based assembler</h3>

<p align="center">
<img src="https://img.shields.io/github/license/amyipdev/libxas">
<img src="https://img.shields.io/tokei/lines/github/amyipdev/libxas">
<img src="https://img.shields.io/github/repo-size/amyipdev/libxas">
</p>

TODO: Links, table-of-contents

The Extendable Assembler Library, or **libxas**, is a power tool for assembly programmers, toolchain writers, 
virtualization/emulation enthusiasts, and more. Using its powerful, efficient, and easy-to-integrate Rust stack,
libxas allows anyone to easily write assembly for a wide span of different platforms, quickly assemble it into 
many different output formats, and write extensions which expand the library's capabilities. It is:

* **Fast**. libxas focuses on speed. Zero-cost abstractions are used to provide a robust, extendable platform 
without compromising on speed.
* **Safe**. Being built in Rust, libxas has high safety and a low potential for vulnerabilities. This is especially
critical in environments where using opcode functionalities for foreign data transformation requires a fully secure,
inescapable environment.
* **Extendable**. The libxas stack can easily be built upon for writing assemblers, build toolchains, and integrating
new architectures and output formats. 

### The Stack

(TODO: Stack diagram)

The libxas stack is designed to be as extendable and modular as possible. Individual output formats and input 
architectures, as well as various features, are easy to disable and compile out.

#### BBU

BBU - "**B**etter **B**in**u**tils" - is the heart of libxas. It holds the various ISA backends with opcode 
translation and optimization, as well as any architecture-specific information which is passed during output.
It also contains ***outs***, a modular output file system which allows for easily implementing new architectures
across a variety of object file formats.

#### EAF

Most usecases of libxas involve simply using the already existing toolchains to assemble lines from various sources.
EAF - the "**E**asy **A**ssembler **F**rontend" - simplifies this process significantly. With a robust API for Rust
programs to link to, EAF abstracts away all the gritty internal details of libxas and presents a clean frontend for
development. Everything is automatically processed between input and output buffers using **Platform**, the format
information system used throughout libxas.

#### XASP

The E**x**tendable **A**ssembler **S**yntax Processor is comprised of two in-house modules: the **Parser**, which
handles macro recognition and instruction formatting, and the **Lexer**, which helps transforms macros and
instructions into structures from BBU modules. These have extreme flexibility through powerful leverage of Rust 
generics, allowing complex architectures to seamlessly integrate. 

### Philosophy

There are absolutely times when toolchains should be built for people "in-the-know". The LLVM and GCC toolchains do 
their job very well, and are designed to be fully integrated at all stages of compiler creation. Tools like LLVM IR
provide powerful abstraction and optimization. For a cutting-edge compiler, those types of tightly-woven toolchains
and libraries are critical. However, not every use case is the next GCC. Often, people just want something simple and 
easy to develop with. When dealing with assembly, this is especially important; assembly development time is extremely
high, so time working with toolchains should be minimized. 

To ensure that anyone can quickly and painlessly work with libxas, we are committed to simple, highly documented
interfaces. Anyone with a new idea for an output format or ISA should not have to worry about initial time developing
either a proof-of-concept or a stable production toolchain. 

### Licensing and Libraries

libxas is licensed under the GNU General Public License, version 2 (or, at your option, any later version). This was
chosen to ensure user freedom while also guaranteeing derivations are available to the entire community.

We also use the following libraries, which have their own respective licenses:
* log: v0.4.17, **MIT** and/or Apache-2.0
* num_traits: v0.2.15, **MIT** and/or Apache-2.0

### Usage 

If you're looking to use individual parts of the stack, check their (TBD) documentation. 
Otherwise, you're probably looking for how to use EAF.

1. Make your Cargo project and add `libxas` as a dependency.
2. Store your assembly in a `String`. If you're working with files, use `std::fs::read_to_string` or something similar.
3. Create a `platform::Platform` instance using one of the creation functions.
4. Call `eaf::assemble_full_source` with your assembly and `Platform`.
5. Read/store the resulting `Vec<u8>` as your output.

### Contributions 

Contributions are always welcome, and very much appreciated. You can:
* Report bugs and suggest new features in Issues
* Resolve TODOs/FIXMEs/NOTEs or make other contributions (please submit a Pull Request)
* Help document libxas (instructions TBA)

### Compatibility

TODO (MSRV)

### Architecture Support 

|        | chip8-raw          | chip8              |
|--------|--------------------|--------------------|
| rawbin | :white_check_mark: | :white_check_mark: |
