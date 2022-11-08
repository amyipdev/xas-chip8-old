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

// TODO: provide more options for inputs (files maybe? put them behind features??)
// TODO: reexport in top-level lib.rs for ease-of-use (look into how to do this properly)

// TODO: this is a library, so good error handling is a must
// need to get rid of ?s and .unwrap()s, and return a Result

// Parser is type-neutral, but everything from then on isn't...
pub fn assemble_full_source(src: &String, pl: &crate::platform::Platform) -> Vec<u8> {
    let mut p: crate::parser::Parser = crate::parser::Parser::from_str(src).unwrap();
    p.parse_all();
    log::trace!("eaf: parser stage completed, assembling file");
    // if only rust could return types from matches...
    match pl.arch {
        // code dup bt c8r, c8 TODO
        #[cfg(feature = "chip8-raw")]
        crate::platform::PlatformArch::ChipEightRaw => {
            assemble_full_source_gen::<
                crate::bbu::chip8_raw::Chip8Symbol,
                crate::bbu::chip8_raw::Chip8PtrSize,
            >(p.pop_vdq(), pl)
        }
        #[cfg(feature = "chip8")]
        crate::platform::PlatformArch::ChipEight => {
            assemble_full_source_gen::<
                crate::bbu::chip8_raw::Chip8Symbol,
                crate::bbu::chip8_raw::Chip8PtrSize,
            >(p.pop_vdq(), pl)
        }
        //_ => panic!("unknown arch")
    }
}

// naturally handles lexer-onwards
pub fn assemble_full_source_gen<T: crate::bbu::SymConv, U: crate::bbu::PtrSize>(
    s: std::collections::VecDeque<crate::parser::ParsedOperation>,
    p: &crate::platform::Platform,
) -> Vec<u8> {
    let mut l: crate::lexer::Lexer<T> = crate::lexer::Lexer::from_vdq(s, p.clone());
    l.lex_full_queue().expect("temporary error handling");
    let mut r: Vec<u8> = Vec::new();
    crate::bbu::outs::run_output::<T, U>(l.pop_vdq(), &mut r, p);
    r
}

// Copied TODOs from main:
// - TODO: have callee clone, or better yet, avoid cloning at all... see below
// - TODO: migrate as much as possible to &String or &str depending on context (not in structs)
// - TODO: pass as many things by reference as possible... question is, is it better to take ownership
//         (guaranteed only shows once) or limit function parameter transfer (faster, risks 2x:1 leak)
//         if going with more pass-by-reference, more memory usage auditing needs to be done
