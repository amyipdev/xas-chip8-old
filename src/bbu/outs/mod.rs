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

pub mod rawbin;

// T is the architecture's pointer size
pub type LabelTree<T> = std::collections::HashMap<String, T>;

pub fn run_output<T: crate::bbu::SymConv>(
    src: Vec<crate::lexer::LexSection<T>>,
    dest: &mut Vec<u8>,
    plat: &crate::platform::Platform,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: opt?
    match plat.target {
        crate::platform::PlatformTarget::RawBinary => rawbin::run_output(src, dest, plat),
    }
    Ok(())
}

// TODO: see run_output notes about dynamic pointer sizes
// NOTE: this is only for things which use automatic linking
pub fn get_offset(p: &crate::platform::Platform) -> u64 {
    match (&p.arch, &p.target) {
        (
            crate::platform::PlatformArch::ChipEightRaw,
            crate::platform::PlatformTarget::RawBinary,
        ) => 0x200,
    }
}
