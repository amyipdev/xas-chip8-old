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

#[cfg(feature = "rawbin")]
pub mod rawbin;

// T = crate::bbu::SymConv
pub type LabelTree<T> = std::collections::HashMap<String, T>;

// TODO: fix inherent cloning issues with String
// also, TODO: it is absolutely not necessary for these to be ordered
// T = crate::bbu::SymConv, U = crate::bbu::PTR_SIZE
pub type UnresSymTree<T, U> = Vec<(Box<dyn crate::bbu::ArchMcrInst<T>>, U)>;

// TODO NOTE: utility function
// o = offset
pub fn vec_update(s: &Vec<u8>, d: &mut Vec<u8>, o: usize) -> () {
    // TODO: make this more efficient, this is slow asf
    // also, TODO: make this work for Vec<T: Integral>
    for e in 0..s.len() {
        d[o + e] = s[e];
    }
}

pub fn run_output<T: crate::bbu::SymConv, U: crate::bbu::PtrSize>(
    src: Vec<crate::lexer::LexSection<T>>,
    dest: &mut Vec<u8>,
    plat: &crate::platform::Platform,
) -> () {
    // each combination could need something different, so each is explicit
    // TODO generate this with macros
    match plat.target {
        #[cfg(feature = "rawbin")]
        crate::platform::PlatformTarget::RawBinary => rawbin::run_output::<T, U>(src, dest, plat),
        //_ => panic!("unsupported combination"),
    }
}
