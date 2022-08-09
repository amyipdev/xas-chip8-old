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

// TODO: proper documentation of everything
// NOTE: EAF = Easy Assembler Frontend
// (or Extendible Assembler Frontend)
// (or Extendible Assembler interFace)

// TODO: function with automatic platform detection
// TODO: function to easily take a compiler combo (chip8-bin) and generate Platform

// TODO: provide more options for inputs (files maybe? put them behind features??)
// TODO: reexport in top-level lib.rs for ease-of-use (look into how to do this
// properly - function wrapping sounds unfun)

// TODO: this is a library, so good error handling is a must
// need to get rid of ?s and .unwrap()s, and return a Result
// TODO: should we clone here? or assume string consumption?
// NOTE: this function should either be distributed out into a baseline function
// called "assemble", or be treated as the baseline function itself
// as it takes in a flexible &str (TODO: consider moving to u8 input? might be bad)
// and returns Vec<u8>, which is the most flexible of all - and doesn't generate
// any data/configuration on its own
/*
pub fn assemble_full_source(src: &String, pl: &crate::platform::Platform) -> Vec<u8> {
    // TODO: better error handling
    let mut p: crate::parser::Parser = crate::parser::Parser::from_str(src).unwrap();
    p.parse_all();
    // TODO: fix EAF, Lexer, Platform to work based on type placement
    let mut l: crate::lexer::Lexer<crate::bbu::chip8_raw::CHIP8_Symbol> =
        crate::lexer::Lexer::from_vdq(p.pop_vdq(), pl.clone());
    // TODO: apparently another untouched result
    l.lex_full_queue();
    let mut r: Vec<u8> = vec![];
    // TODO: handle yielded Result apparently?
    crate::bbu::outs::run_output(l.pop_vdq(), &mut r, pl);
    r
}
 */

// Parser is type-neutral, but everything from then on isn't...
pub fn assemble_full_source(src: &String, pl: &crate::platform::Platform) -> Vec<u8> {
    let mut p: crate::parser::Parser = crate::parser::Parser::from_str(src).unwrap();
    p.parse_all();
    // if only rust could return types from matches...
    (match pl.arch {
        crate::platform::PlatformArch::ChipEightRaw => {
            assemble_full_source_gen::<
                crate::bbu::chip8_raw::CHIP8_Symbol,
                crate::bbu::chip8_raw::CHIP8_PTR_SIZE,
            >
        }
    })(p.pop_vdq(), pl)
}

// naturally handles lexer-onwards
pub fn assemble_full_source_gen<T: crate::bbu::SymConv, U: crate::bbu::PTR_SIZE>(
    s: std::collections::VecDeque<crate::parser::ParsedOperation>,
    p: &crate::platform::Platform,
) -> Vec<u8> {
    let mut l: crate::lexer::Lexer<T> = crate::lexer::Lexer::from_vdq(s, p.clone());
    l.lex_full_queue();
    let mut r: Vec<u8> = Vec::new();
    crate::bbu::outs::run_output::<T, U>(l.pop_vdq(), &mut r, p);
    r
}

// Copied TODOs from main:
// - TODO: instead of manual .next(), consider .skip(1) or else
// - TODO: put cloning responsibilities on the callee, not the caller, whenever ownership doesn't need to be taken
// - TODO: once again, we might want to pass &String to allow for mutability! clone if needed
// -       thought: wait, does &String not allow &str/&'static str?
// - TODO: pass as many things by reference as possible... question is, is it better to take ownership
//         (guaranteed only shows once) or limit function parameter transfer (faster, risks 2x:1 leak)
//         if going with more pass-by-reference, more memory usage auditing needs to be done
