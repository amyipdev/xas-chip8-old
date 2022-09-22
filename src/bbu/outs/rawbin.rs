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

// TODO: proj global, utilize burns (fn FNNAME(self), which drops the object)

// todo drop
// TODO: featurize
// TODO: no-std

// TODO: make OptionLeaf a globally available concept
enum OptionLeaf<T: crate::bbu::SymConv> {
    Constant(Vec<u8>),
    Symbol(Box<dyn crate::bbu::ArchMcrInst<T>>),
}

type VecOptTree<T> = Vec<OptionLeaf<T>>;

// TODO: move offsets into another part of BBU maybe? probably arch pages?
// TODO: const fn somehow
fn get_offset<T: crate::bbu::PtrSize>(p: &crate::platform::Platform) -> T {
    #[allow(unreachable_patterns)]
    match &p.arch {
        #[cfg(feature = "chip8-raw")]
        crate::platform::PlatformArch::ChipEightRaw => T::from_int(0x200),
        #[cfg(feature = "chip8")]
        crate::platform::PlatformArch::ChipEight => T::from_int(0x200),
        _ => lpanic("rawbin: unknown arch"),
    }
}

pub fn run_output<T: crate::bbu::SymConv, U: crate::bbu::PtrSize>(
    src: Vec<crate::lexer::LexSection<T>>,
    // TODO: since dest needs to be empty for symbols to work,
    // consider just returning the Vec... or, better yet, manually clearing it!
    // to deal with overhead, I think Vec is just a pointer...
    dest: &mut Vec<u8>,
    plat: &crate::platform::Platform,
) -> () {
    let offs: U = get_offset(plat);
    let mut cp: U = U::from_int::<usize>(0usize);
    let mut lt: crate::bbu::outs::LabelTree<T> = crate::bbu::outs::LabelTree::new();
    let mut dt: VecOptTree<T> = Vec::new();

    for section in src {
        for label in section.labels {
            // TODO: support non-label symbols (constants, computed constants)
            let label_parts: (crate::lexer::LexIdLabel<T>, Option<String>) = label.extract();
            if let Some(n) = label_parts.1 {
                let mut np: U = cp.clone();
                // TODO: support referencing out to data
                np.add_ptr(offs);
                lt.insert(n.clone(), T::from_ptr(np));
            }
            for op in label_parts.0 {
                let b = match op {
                    // macros will be dead soon anyways
                    crate::lexer::LexOperation::Macro(m) => {
                        let l = m.get_length();
                        dt.push(OptionLeaf::Constant(m.get_output_bytes()));
                        l
                    }
                    crate::lexer::LexOperation::Instruction(n) => {
                        let l = n.get_length(); // pre-save before move
                        if !n.check_symbols() {
                            // TODO: lazy-load bytes to save copy penalty
                            dt.push(OptionLeaf::Constant(n.get_output_bytes()));
                        } else {
                            dt.push(OptionLeaf::Symbol(n))
                        }
                        l
                    }
                };
                cp.add_int(b);
            }
        }
    }

    for i in dt {
        match i {
            OptionLeaf::Constant(n) => dest.extend(n),
            OptionLeaf::Symbol(mut m) => {
                for s in m.get_symbols().unwrap() {
                    if lt.contains_key(&*s.0) {
                        m.fulfill_symbol(&lt[&*s.0], s.1);
                    } else {
                        lpanic(&format!("rawbin: unresolvable symbol: {}", s.0))
                    }
                }
                dest.extend(m.get_output_bytes());
            }
        }
    }
}
