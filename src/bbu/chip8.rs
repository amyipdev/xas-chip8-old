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

use crate::bbu::chip8_raw;
use crate::errors::lpanic;

use crate::bbu::ArchMacro;

// TODO: utility, global it - readd in p
// TODO: probably a better way than this
fn arg_is_register(a: &Option<Vec<String>> /*, p: usize*/) -> bool {
    if let Some(crate::bbu::ArchArg::Register(_)) = crate::bbu::parse_arg::<
        chip8_raw::Chip8PtrSize,
        chip8_raw::Chip8DatSize,
        chip8_raw::Chip8DisSize,
        chip8_raw::Chip8ArchReg,
    >(&a.as_ref().unwrap()[0])
    {
        true
    } else {
        false
    }
}

macro_rules! gim {
    ($n:ident,$i:ident) => {{
        Box::new(<chip8_raw::$n as crate::bbu::ArchMcrInst<
            chip8_raw::Chip8Symbol,
        >>::get_lex($i.args))
    }};
}

// TODO: macro for generating arg_is_register sets,
// there's a lot of code duplication here
pub fn get_instruction<T: crate::bbu::SymConv>(
    i: crate::parser::ParsedInstruction,
) -> Box<dyn crate::bbu::ArchMcrInst<T>> {
    match i.instr.to_lowercase().as_str() {
        "mcr" => gim!(Chip8_0NNN, i),
        "cls" => gim!(Chip8_00E0, i),
        "ret" => gim!(Chip8_00EE, i),
        "jmp" => gim!(Chip8_1NNN, i),
        "call" => gim!(Chip8_2NNN, i),
        // TODO: better "detection" function maybe?
        // we could make it so that arguments are checked always
        // getting out of panicking and into proper error handling
        // this is the Correct Robust Way(TM)
        "je" => {
            if arg_is_register(&i.args) {
                gim!(Chip8_5XY0, i)
            } else {
                gim!(Chip8_3XNN, i)
            }
        }
        "jne" => {
            if arg_is_register(&i.args) {
                gim!(Chip8_9XY0, i)
            } else {
                gim!(Chip8_4XNN, i)
            }
        }
        "mov" => {
            if arg_is_register(&i.args) {
                gim!(Chip8_8XY0, i)
            } else {
                gim!(Chip8_6XNN, i)
            }
        }
        "add" => {
            if arg_is_register(&i.args) {
                gim!(Chip8_8XY4, i)
            } else {
                gim!(Chip8_7XNN, i)
            }
        }
        "sub" => gim!(Chip8_8XY5, i),
        "or" => gim!(Chip8_8XY1, i),
        "and" => gim!(Chip8_8XY2, i),
        "xor" => gim!(Chip8_8XY3, i),
        "lsc" => gim!(Chip8_8XYE, i),
        "rsc" => gim!(Chip8_8XY6, i),
        "ref" => gim!(Chip8_ANNN, i),
        "ljmp" => gim!(Chip8_BNNN, i),
        "rnd" => gim!(Chip8_CXNN, i),
        "draw" => gim!(Chip8_DXYN, i),
        "kye" => gim!(Chip8_EX9E, i),
        "kyn" => gim!(Chip8_EXA1, i),
        "gdl" => gim!(Chip8_FX07, i),
        "gky" => gim!(Chip8_FX0A, i),
        "sdl" => gim!(Chip8_FX15, i),
        "sds" => gim!(Chip8_FX18, i),
        "pti" => gim!(Chip8_FX1E, i),
        "lds" => gim!(Chip8_FX29, i),
        "bcd" => gim!(Chip8_FX33, i),
        "dmp" => gim!(Chip8_FX55, i),
        "lod" => gim!(Chip8_FX65, i),
        _ => lpanic("unknown instruction error"),
    }
}

pub fn get_macro(i: crate::parser::ParsedMacro) -> Box<dyn ArchMacro> {
    chip8_raw::get_macro(i)
}
