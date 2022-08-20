<h1 align="center">The Extendable Assembler Library</h1>
<h3 align="center">libxas, a Bell-style, open-source, fast, Rust-based assembler</h3>

<p align="center">
<img src="https://img.shields.io/github/license/amyipdev/libxas">
<img src="https://img.shields.io/tokei/lines/github/amyipdev/libxas">
<img src="https://img.shields.io/github/repo-size/amyipdev/libxas">
</p>

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


