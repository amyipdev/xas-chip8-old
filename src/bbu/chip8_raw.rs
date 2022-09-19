// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * libxas: Extendable Assembler Library
 * Copyright (C) 2022 Amy Parker <apark0006@student.cerritos.edu>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA or visit
 * the GNU Project at <https://gnu.org/licenses>. The GNU General Public
 * License version 2 is available at, for your convenience,
 * <https://gnu.org/licenses/old-licenses/gpl-2.0.html>.
 */

use crate::errors::lpanic;

// TODO: Code organization and cleanup
// FIXME: too many issues wrt unnecessary symbol names/expansion, needs (gbl) cleanup

use std::str::FromStr;

// TODO: make all arches (including chip8_raw) under features
// same with a lot of other parts

// TODO: better error handling
// TODO: reduce repetition of this
use crate::bbu::ArchInstruction;
use crate::bbu::ArchMacro;

// TODO: push the shortening out throughout the file
// TODO: and same with this:
use crate::bbu::DatSize;
use crate::bbu::PtrSize;

pub type Chip8DatSize = crate::bbu::GenScal<u8>;
// TODO: generic displacement size for types without one
// (@u8 for size)
pub type Chip8DisSize = crate::bbu::GenScal<u8>;
pub type Chip8PtrSize = crate::bbu::Gen12;

pub type Chip8SymAlias = crate::bbu::ArgSymbol<Chip8PtrSize, Chip8DatSize>;

//pub type CHIP8_Symbol = crate::bbu::ArgSymbol<CHIP8_PTR_SIZE, CHIP8_DAT_SIZE>;
pub struct Chip8Symbol {
    pub i: Chip8SymAlias,
}

// TODO: make this, other parts of ArgSymbol construction a macro
impl crate::bbu::SymConv for Chip8Symbol {
    // FIXME FIXME FIXME: into_dat
    fn from_ptr<T: crate::bbu::PtrSize>(a: T) -> Self {
        Self {
            i: crate::bbu::ArgSymbol::Pointer(Box::new(
                <Chip8PtrSize as PtrSize>::from_int::<u16>(a.extract_int::<u16>()),
            )),
        }
    }
    fn from_dat<T: crate::bbu::DatSize>(a: T) -> Self {
        Self {
            i: crate::bbu::ArgSymbol::Data(Box::new(<Chip8DatSize as DatSize>::from_int::<u8>(
                a.extract_int::<u8>(),
            ))),
        }
    }
    fn into_ptr<T: crate::bbu::PtrSize, E: crate::bbu::Integral>(&self) -> T {
        T::from_int(crate::bbu::PtrSize::extract_int::<E>(
            &**self.i.unwrap_ptr().unwrap(),
        ))
    }
}

impl<T: crate::bbu::SymConv> crate::bbu::ArchSym<T> for Chip8Symbol {
    fn get_uk_sym(&self) -> Option<&String> {
        match &self.i {
            crate::bbu::ArgSymbol::UnknownPointer(i) | crate::bbu::ArgSymbol::UnknownData(i) => {
                Some(i)
            }
            _ => None,
        }
    }
    fn set_sym(&mut self, _a: T) -> () {
        unimplemented!()
    }
}

macro_rules! gim {
    ($n:ident,$i:ident) => {{
        Box::new(<$n as ArchInstruction<Chip8Symbol>>::get_lex($i.args))
    }};
}

pub fn get_instruction<T: crate::bbu::SymConv>(
    i: crate::parser::ParsedInstruction,
) -> Box<dyn ArchInstruction<T>> {
    match i.instr.to_lowercase().as_str() {
        // TODO: reduce code dup, tie into the macros beforehand??
        "0nnn" => gim!(Chip8_0NNN, i),
        "00e0" => gim!(Chip8_00E0, i),
        "00ee" => gim!(Chip8_00EE, i),
        "1nnn" => gim!(Chip8_1NNN, i),
        "2nnn" => gim!(Chip8_2NNN, i),
        "3xnn" => gim!(Chip8_3XNN, i),
        "4xnn" => gim!(Chip8_4XNN, i),
        "5xy0" => gim!(Chip8_5XY0, i),
        "6xnn" => gim!(Chip8_6XNN, i),
        "7xnn" => gim!(Chip8_7XNN, i),
        "8xy0" => gim!(Chip8_8XY0, i),
        "8xy1" => gim!(Chip8_8XY1, i),
        "8xy2" => gim!(Chip8_8XY2, i),
        "8xy3" => gim!(Chip8_8XY3, i),
        "8xy4" => gim!(Chip8_8XY4, i),
        "8xy5" => gim!(Chip8_8XY5, i),
        "8xy6" => gim!(Chip8_8XY6, i),
        "8xy7" => gim!(Chip8_8XY7, i),
        "8xye" => gim!(Chip8_8XYE, i),
        "9xy0" => gim!(Chip8_9XY0, i),
        "annn" => gim!(Chip8_ANNN, i),
        "bnnn" => gim!(Chip8_BNNN, i),
        "cxnn" => gim!(Chip8_CXNN, i),
        "dxyn" => gim!(Chip8_DXYN, i),
        "ex9e" => gim!(Chip8_EX9E, i),
        "exa1" => gim!(Chip8_EXA1, i),
        "fx07" => gim!(Chip8_FX07, i),
        "fx0a" => gim!(Chip8_FX0A, i),
        "fx15" => gim!(Chip8_FX15, i),
        "fx18" => gim!(Chip8_FX18, i),
        "fx1e" => gim!(Chip8_FX1E, i),
        "fx29" => gim!(Chip8_FX29, i),
        "fx33" => gim!(Chip8_FX33, i),
        "fx55" => gim!(Chip8_FX55, i),
        "fx65" => gim!(Chip8_FX65, i),
        _ => lpanic("unknown instruction error"),
    }
}

#[derive(Copy, Clone)]
pub struct Chip8ArchReg {
    n: u8,
}

impl FromStr for Chip8ArchReg {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 3 || s.len() < 2 {
            lpanic("chip8_raw: unknown register")
        } else {
            // NOTE: consider full UTF-8 support
            let a: char = s.chars().nth(2).unwrap();
            Ok(Chip8ArchReg {
                n: char::to_digit(
                    if a == 'v' {
                        s.chars().nth(3).unwrap()
                    } else {
                        a
                    },
                    16,
                )
                .unwrap() as u8,
            })
        }
    }
}

impl crate::bbu::ArchReg for Chip8ArchReg {}

pub type Chip8Arg = crate::bbu::ArchArg<Chip8PtrSize, Chip8DatSize, Chip8DisSize, Chip8ArchReg>;

// TODO: is vec![] best here?
fn chip8_placeholder() -> Vec<u8> {
    vec![0u8, 0u8]
}

// TODO: review of public vs pub(crate) vs private api
const CHIP8_INSTR_LEN: u8 = 2u8;

// lots of code duplication with get_output_bytes TODO FIXME NOTE, somen with get_lex

macro_rules! make_std_const {
    ($nm:ident,$offs:expr) => {
        pub struct $nm {}
        impl<T: crate::bbu::SymConv> ArchInstruction<T> for $nm {
            fn get_output_bytes(&self) -> Vec<u8> {
                Vec::from($offs.to_be_bytes())
            }
            fn get_lex(_a: Option<Vec<String>>) -> Self {
                Self {}
            }
            fn check_symbols(&self) -> bool {
                false
            }
            fn get_symbols(&self) -> Option<Vec<(&String, crate::bbu::SymbolPosition)>> {
                None
            }
            fn get_length(&self) -> crate::bbu::SymbolPosition {CHIP8_INSTR_LEN}
            fn get_placeholder(&self) -> Vec<u8> {
                chip8_placeholder()
            }
            fn fulfill_symbol(&mut self, _s: &T, _p: crate::bbu::SymbolPosition) -> () {}
        }
    };
}

macro_rules! make_std_nnn {
    ($nm:ident,$offs:expr) => {
        pub struct $nm {
            addr: Chip8SymAlias,
        }
        impl<T: crate::bbu::SymConv> ArchInstruction<T> for $nm {
            fn get_output_bytes(&self) -> Vec<u8> {
                Vec::from(($offs | self.addr.unwrap_ptr().unwrap().i).to_be_bytes())
            }
            fn get_lex(a: Option<Vec<String>>) -> Self {
                Self { addr: get_nnn(a) }
            }
            fn check_symbols(&self) -> bool {
                match self.addr {
                    crate::bbu::ArgSymbol::UnknownPointer(_) => true,
                    _ => false,
                }
            }
            fn get_symbols(&self) -> Option<Vec<(&String, crate::bbu::SymbolPosition)>> {
                let r = match self.addr {
                    crate::bbu::ArgSymbol::UnknownPointer(ref a) => Some(vec![(a, 0)]),
                    _ => None,
                };
                r
            }
            fn get_length(&self) -> crate::bbu::SymbolPosition {CHIP8_INSTR_LEN}
            fn get_placeholder(&self) -> Vec<u8> {
                chip8_placeholder()
            }
            fn fulfill_symbol(&mut self, s: &T, p: crate::bbu::SymbolPosition) -> () {
                match p {
                    0 => {
                        self.addr = crate::bbu::ArgSymbol::Pointer(Box::new(
                            s.into_ptr::<Chip8PtrSize, u16>(),
                        ))
                    }
                    _ => lpanic("c8r: unknown positional"),
                }
            }
        }
    };
}

// TODO: would it be more space-efficient to split the formation
//       and vec creation instructions into a function, so there's
//       only one copy of it in the code?
// NOTE: optimize
macro_rules! make_std_xnn {
    ($nm:ident,$offs:expr) => {
        pub struct $nm {
            x: Chip8ArchReg,
            d: Chip8SymAlias,
        }
        impl<T: crate::bbu::SymConv> ArchInstruction<T> for $nm {
            fn get_output_bytes(&self) -> Vec<u8> {
                Vec::from(
                    ($offs | ((self.x.n as u16) << 8) | (self.d.unwrap_data().unwrap().i as u16))
                        .to_be_bytes(),
                )
            }
            fn get_lex(a: Option<Vec<String>>) -> Self {
                let b: (Chip8ArchReg, Chip8SymAlias) = get_xnn(a);
                Self { x: b.0, d: b.1 }
            }
            fn check_symbols(&self) -> bool {
                match self.d {
                    crate::bbu::ArgSymbol::UnknownData(_) => true,
                    _ => false,
                }
            }
            fn get_symbols(&self) -> Option<Vec<(&String, crate::bbu::SymbolPosition)>> {
                match self.d {
                    crate::bbu::ArgSymbol::UnknownData(ref a) => Some(vec![(a, 0)]),
                    _ => None,
                }
            }
            fn get_length(&self) -> crate::bbu::SymbolPosition {CHIP8_INSTR_LEN}
            fn get_placeholder(&self) -> Vec<u8> {
                chip8_placeholder()
            }
            fn fulfill_symbol(&mut self, s: &T, p: crate::bbu::SymbolPosition) -> () {
                match p {
                    0 => {
                        self.d =
                            crate::bbu::ArgSymbol::Data(Box::new(s.into_ptr::<Chip8DatSize, u8>()))
                    }
                    _ => lpanic("c8r: unknown positional"),
                }
            }
        }
    };
}

// TODO: general for archinstruction, is vec best? could boxed slice work better?
macro_rules! make_std_xy {
    ($nm:ident,$offs:expr) => {
        pub struct $nm {
            s: Chip8ArchReg,
            d: Chip8ArchReg,
        }
        impl<T: crate::bbu::SymConv> ArchInstruction<T> for $nm {
            fn get_output_bytes(&self) -> Vec<u8> {
                Vec::from(
                    ($offs | ((self.d.n as u16) << 8) | ((self.s.n as u16) << 4)).to_be_bytes(),
                )
            }
            fn get_lex(a: Option<Vec<String>>) -> Self {
                let b: (Chip8ArchReg, Chip8ArchReg) = get_xy(a);
                Self { s: b.0, d: b.1 }
            }
            // implementation is the same as for make_std_const
            fn check_symbols(&self) -> bool {
                false
            }
            fn get_symbols(&self) -> Option<Vec<(&String, crate::bbu::SymbolPosition)>> {
                None
            }
            fn get_length(&self) -> crate::bbu::SymbolPosition {CHIP8_INSTR_LEN}
            fn get_placeholder(&self) -> Vec<u8> {
                chip8_placeholder()
            }
            fn fulfill_symbol(&mut self, _s: &T, _p: crate::bbu::SymbolPosition) -> () {}
        }
    };
}

// TODO: create type names for each arg combo type to make tuples easier
// XYN = $N,%vX,%vY
macro_rules! make_std_xyn {
    ($nm:ident,$offs:expr) => {
        // TODO: in the future, let's stray away from needing Copy and Clone
        pub struct $nm {
            n: Chip8SymAlias,
            x: Chip8ArchReg,
            y: Chip8ArchReg,
        }
        impl<T: crate::bbu::SymConv> ArchInstruction<T> for $nm {
            fn get_output_bytes(&self) -> Vec<u8> {
                // TODO: warn on overflow
                Vec::from(
                    ($offs
                        | ((self.x.n as u16) << 8)
                        | ((self.y.n as u16) << 4)
                        | ((self.n.unwrap_data().unwrap().i as u16) & 0xf))
                        .to_be_bytes(),
                )
            }
            fn get_lex(a: Option<Vec<String>>) -> Self {
                let b: (Chip8SymAlias, Chip8ArchReg, Chip8ArchReg) = get_xyn(a);
                Self {
                    n: b.0,
                    x: b.1,
                    y: b.2,
                }
            }
            fn check_symbols(&self) -> bool {
                match self.n {
                    crate::bbu::ArgSymbol::UnknownData(_) => true,
                    _ => false,
                }
            }
            fn get_symbols(&self) -> Option<Vec<(&String, crate::bbu::SymbolPosition)>> {
                match self.n {
                    crate::bbu::ArgSymbol::UnknownData(ref a) => Some(vec![(a, 0)]),
                    _ => None,
                }
            }
            fn get_length(&self) -> crate::bbu::SymbolPosition {CHIP8_INSTR_LEN}
            fn get_placeholder(&self) -> Vec<u8> {
                chip8_placeholder()
            }
            fn fulfill_symbol(&mut self, s: &T, p: crate::bbu::SymbolPosition) -> () {
                match p {
                    0 => {
                        self.n =
                            crate::bbu::ArgSymbol::Data(Box::new(s.into_ptr::<Chip8DatSize, u8>()))
                    }
                    _ => lpanic("c8r: unknown positional"),
                }
            }
        }
    };
}

// named efx because all efx instructions start with 0xE or 0xF
// NOTE wish I had a macro for making these macros
macro_rules! make_std_efx {
    ($nm:ident,$offs:expr) => {
        pub struct $nm {
            x: Chip8ArchReg,
        }
        impl<T: crate::bbu::SymConv> ArchInstruction<T> for $nm {
            fn get_output_bytes(&self) -> Vec<u8> {
                Vec::from(($offs | ((self.x.n as u16) << 8)).to_be_bytes())
            }
            fn get_lex(a: Option<Vec<String>>) -> Self {
                Self { x: get_efx(a) }
            }
            // implementation is the same as for make_std_const as well
            fn check_symbols(&self) -> bool {
                false
            }
            fn get_symbols(&self) -> Option<Vec<(&String, crate::bbu::SymbolPosition)>> {
                None
            }
            fn get_length(&self) -> crate::bbu::SymbolPosition {CHIP8_INSTR_LEN}
            fn get_placeholder(&self) -> Vec<u8> {
                chip8_placeholder()
            }
            fn fulfill_symbol(&mut self, _s: &T, _p: crate::bbu::SymbolPosition) -> () {}
        }
    };
}

make_std_nnn!(Chip8_0NNN, 0u16);
make_std_const!(Chip8_00E0, 0xe0u16);
make_std_const!(Chip8_00EE, 0xeeu16);
make_std_nnn!(Chip8_1NNN, 0x1000u16);
make_std_nnn!(Chip8_2NNN, 0x2000u16);
make_std_xnn!(Chip8_3XNN, 0x3000u16);
make_std_xnn!(Chip8_4XNN, 0x4000u16);
make_std_xy!(Chip8_5XY0, 0x5000u16);
make_std_xnn!(Chip8_6XNN, 0x6000u16);
make_std_xnn!(Chip8_7XNN, 0x7000u16);
make_std_xy!(Chip8_8XY0, 0x8000u16);
make_std_xy!(Chip8_8XY1, 0x8001u16);
make_std_xy!(Chip8_8XY2, 0x8002u16);
make_std_xy!(Chip8_8XY3, 0x8003u16);
make_std_xy!(Chip8_8XY4, 0x8004u16);
make_std_xy!(Chip8_8XY5, 0x8005u16);
make_std_xy!(Chip8_8XY6, 0x8006u16);
make_std_xy!(Chip8_8XY7, 0x8007u16);
make_std_xy!(Chip8_8XYE, 0x800eu16);
make_std_xy!(Chip8_9XY0, 0x9000u16);
make_std_nnn!(Chip8_ANNN, 0xa000u16);
make_std_nnn!(Chip8_BNNN, 0xb000u16);
make_std_xnn!(Chip8_CXNN, 0xc000u16);
make_std_xyn!(Chip8_DXYN, 0xd000u16);
make_std_efx!(Chip8_EX9E, 0xe09eu16);
make_std_efx!(Chip8_EXA1, 0xe0a1u16);
make_std_efx!(Chip8_FX07, 0xf007u16);
make_std_efx!(Chip8_FX0A, 0xf00au16);
make_std_efx!(Chip8_FX15, 0xf015u16);
make_std_efx!(Chip8_FX18, 0xf018u16);
make_std_efx!(Chip8_FX1E, 0xf01eu16);
make_std_efx!(Chip8_FX29, 0xf029u16);
make_std_efx!(Chip8_FX33, 0xf033u16);
make_std_efx!(Chip8_FX55, 0xf055u16);
make_std_efx!(Chip8_FX65, 0xf065u16);

// FIXME: switch to argcheck
// TODO condense similarly to get_xnn
fn get_nnn(a: Option<Vec<String>>) -> Chip8SymAlias {
    if let Some(ref i) = a {
        if i.len() != 1 {
            lpanic("c8r: wrong arg count")
        }
        let b: Chip8Arg = crate::bbu::parse_arg(&a.unwrap()[0]).unwrap();
        // TODO: avoid this clone in the future
        b.unwrap_memory().unwrap().v.clone()
    } else {
        lpanic("c8r: not enough args")
    }
}

// this logic structure is repeated a lot
// TODO consider condensing it somehow
fn get_xnn(a: Option<Vec<String>>) -> (Chip8ArchReg, Chip8SymAlias) {
    let b: Vec<Chip8Arg> = argcheck(&a, 2);
    (
            *b[1].unwrap_register().unwrap().reg,
            // TODO: also avoid this clone
            b[0].unwrap_direct().unwrap().clone(),
    )
    /*
    if let Some(ref i) = a {
        if i.len() != 2 {
            lpanic("c8r: not enough args")
        }
        // TODO: move these directly in
        // data
        let b: Chip8Arg = crate::bbu::parse_arg(&a.as_ref().unwrap()[0]).unwrap();
        // register X
        let c: Chip8Arg = crate::bbu::parse_arg(&a.as_ref().unwrap()[1]).unwrap();
        (
            *c.unwrap_register().unwrap().reg,
            // TODO: also avoid this clone
            b.unwrap_direct().unwrap().clone(),
        )
    } else {
        lpanic("c8r: not enough args")
    }*/
}

// NOTE: is tuple the best option here? Would an array be better?
fn get_xy(a: Option<Vec<String>>) -> (Chip8ArchReg, Chip8ArchReg) {
    //if let Some(ref i) = a {
    //    if i.len() != 2 {
    //        panic!("c8r: not enough args")
    //    }
    let b: Vec<Chip8Arg> = argcheck(&a, 2);
    (
        *b[0].unwrap_register().unwrap().reg,
        *b[1].unwrap_register().unwrap().reg,
    )
    //} else {
    //    panic!("c8r: not enough args")
    //}
}

// res.0 limited to 4 bits
fn get_xyn(a: Option<Vec<String>>) -> (Chip8SymAlias, Chip8ArchReg, Chip8ArchReg) {
    let b: Vec<Chip8Arg> = argcheck(&a, 3);
    (
        // TODO FIXME: get rid of the clones!!
        b[0].unwrap_direct().unwrap().clone(),
        *b[1].unwrap_register().unwrap().reg,
        *b[2].unwrap_register().unwrap().reg,
    )
}

fn get_efx(a: Option<Vec<String>>) -> Chip8ArchReg {
    let b: Vec<Chip8Arg> = argcheck(&a, 1);
    *b[0].unwrap_register().unwrap().reg
}

// TODO: make this a utility public function
// TODO: better error handling (log)
// TODO: dedup panic message
fn argcheck(a: &Option<Vec<String>>, i: usize) -> Vec<Chip8Arg> {
    if let Some(ref b) = a {
        if b.len() != i {
            lpanic("argument check failed")
        } else {
            Vec::from_iter(b.into_iter().map(|x| crate::bbu::parse_arg(&x).unwrap()))
        }
    } else {
        lpanic("argument check failed")
    }
}

// TODO genericize for LE, BE

// TODO FIXME NOTE: throw warnings when number is truncated!

macro_rules! gmm {
    ($n:ident,$i:ident) => {{
        Box::new(<crate::bbu::$n as ArchMacro>::get_lex($i.args))
    }}
}

// TODO: consider putting these in lexer maybe? idk
// TODO FIXME: add symbols to macros
pub fn get_macro(
    i: crate::parser::ParsedMacro,
) -> Box<dyn ArchMacro> {
    match i.mcr.to_lowercase().as_str() {
        "byte" => gmm!(BigByte, i),
        _ => lpanic("macro not supported")
    }
}
