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

use std::str::FromStr;

// TODO: make all arches (including chip8_raw) under features
// same with a lot of other parts

// TODO: better error handling
// TODO: reduce repetition of this
use crate::bbu::ArchInstruction;

pub type CHIP8_DAT_SIZE = crate::bbu::GenScal<u8>;
// TODO: generic displacement size for types without one
// (@u8 for size)
pub type CHIP8_DIS_SIZE = crate::bbu::GenScal<u8>;
pub type CHIP8_PTR_SIZE = crate::bbu::Gen12;

pub fn get_instruction(i: crate::parser::ParsedInstruction) -> impl crate::bbu::ArchInstruction {
    match i.instr.as_str() {
        "0NNN" | "0nnn" => Chip8_0NNN::get_lex(i.args),
        _ => panic!("unknown instruction error"),
    }
}

#[derive(Copy, Clone)]
pub struct CHIP8_ArchReg {
    n: u8,
}

impl FromStr for CHIP8_ArchReg {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 3 || s.len() < 2 {
            panic!("chip8_raw: unknown register")
        } else {
            // TODO: FIXME TODO ensure full UTF-8 compatibility crossproj
            let a: char = s.chars().nth(2).unwrap();
            Ok(CHIP8_ArchReg {
                n: char::to_digit(
                    if a == 'v' {
                        s.chars().nth(3).unwrap()
                    } else {
                        a
                    },
                    10,
                )
                .unwrap() as u8,
            })
        }
    }
}

impl crate::bbu::ArchReg for CHIP8_ArchReg {}

// chip8 "call" function
pub struct Chip8_0NNN {
    addr: CHIP8_PTR_SIZE,
}

// TODO: better logging/errors (file name, line number, proper messages, etc)
impl crate::bbu::ArchInstruction for Chip8_0NNN {
    fn get_output_bytes(&self) -> Vec<u8> {
        vec![
            ((self.addr.i & 0xF00) >> 8) as u8,
            (self.addr.i & 0xFF) as u8,
        ]
    }
    // TODO: fix all this
    fn get_lex(a: Option<Vec<String>>) -> Self {
        if let Some(ref i) = a {
            if i.len() != 1 {
                panic!("chip8_raw: improper number of arguments")
            }
            let b: crate::bbu::ArchArg<
                CHIP8_PTR_SIZE,
                CHIP8_DAT_SIZE,
                CHIP8_DIS_SIZE,
                CHIP8_ArchReg,
            > = crate::bbu::parse_arg(&a.unwrap()[0]).unwrap();
            if let crate::bbu::ArchArg::Memory(c) = b {
                if let crate::bbu::ArgSymbol::Pointer(d) = c.v {
                    Chip8_0NNN { addr: *d }
                } else {
                    panic!("no dynamic symbol support on chip8 yet")
                }
            } else {
                panic!("wrong operand type")
            }
        } else {
            panic!("chip8_raw: not enough arguments")
        }
        /*
        // TODO: better number checking, this is proof-of-concept
        if let Some(i) = a {
            if i.len() != 1 {
                panic!("Improper number of arguments")
            }
            // TODO: proper error handling if number is out of sizescope
            let v: u16 = crate::bbu::parse_unknown_radix_u16(&i[0]).unwrap();
            // instruction is only valid when >0x200, <0x1000
            if v < 0x200 || v > 0xfff {
                panic!("Call out of range")
            }
            Chip8_0NNN {
                args: (((v & 0xf00) >> 8) as u8, v as u8 & 0xff),
            }
        } else {
            panic!("No arguments provided, error")
        }
        */
    }
}
