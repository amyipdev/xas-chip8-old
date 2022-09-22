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

/*
pub fn run_output<T: crate::bbu::SymConv, U: crate::bbu::PtrSize>(
    src: Vec<crate::lexer::LexSection<T>>,
    // TODO: since dest needs to be empty for symbols to work,
    // consider just returning the Vec... or, better yet, manually clearing it!
    // to deal with overhead, I think Vec is just a pointer...
    dest: &mut Vec<u8>,
    plat: &crate::platform::Platform,
) -> () {
    let offs: U = get_offset(plat);
    // NOTE: vecs only support indices of usize
    // this should be fine but is still a potential concern
    let mut cp: U = U::from_int::<usize>(0usize);
    let mut lt: crate::bbu::outs::LabelTree<T> = crate::bbu::outs::LabelTree::new();
    let mut st: crate::bbu::outs::UnresSymTree<T, U> = crate::bbu::outs::UnresSymTree::new();

    // NOTE: as a general philosophy, consumption is completely okay
    for section in src {
        for label in section.labels {
            let label_parts: (crate::lexer::LexIdLabel<T>, Option<String>) = label.extract();
            if let Some(n) = label_parts.1 {
                // TODO: this is inefficient
                let mut np: U = cp.clone();
                // TODO: support referencing out to data
                np.add_ptr(offs);
                lt.insert(n.clone(), T::from_ptr(np));
            }
            for op in label_parts.0 {
                let b: Vec<u8> = match op {
                    // TODO TODO TODO TODO TODO TODO TODO: symbol support in macros
                    crate::lexer::LexOperation::Macro(m) => m.get_output_bytes(),
                    crate::lexer::LexOperation::Instruction(n) => {
                        if n.check_symbols() {
                            // TODO: remove instantiation
                            let r = n.get_placeholder();
                            st.push((n, cp));
                            r
                        } else {
                            n.get_output_bytes()
                        }
                    } //_ => panic!("rawbin: unsupported op type"),
                };
                cp.add_int(b.len());
                dest.extend(b);
            }
        }
    }
    // now for the linkage magic
    for i in &mut st {
        // i.1 = the position this needs to be updated at
        // see above the issue with vecs with a length of usize
        // TODO: only used once, put in directly
        let np: usize = i.1.extract_int::<usize>();
        // This is code I really really hate... thanks rust data safety
        // Like, the reasoning for it completely makes sense. Data invalidation.
        // Just still very annoying that this has to be done. I know for certain
        // that I'm not risking anything, and that my code is safe. But the compiler
        // isn't smart enough to know that. So either I work with unsafe ptrs,
        // or I just do this. For now I choose this.
        let mut symresv: Vec<(&T, u8)> = Vec::new();
        // guaranteed to be Some(V), TODO optimize
        for s in i.0.get_symbols().unwrap() {
            if lt.contains_key(s.0) {
                symresv.push((&lt[s.0], s.1));
            } else {
                // Rawbin spec is to panic if a symbol can't be resolved
                // to avoid unintentionally unsafe code. (NOTE: error handle instead future)
                // This isn't necessarily the case for formats which are outputting
                // intermediate object representations. In the future, Rawbin
                // (through values on its enum varient) TODO should accept an option to
                // create a symbol resolution file, listing symbols and their positions.
                // This also leads to the need to TODO override offsets.
                lpanic(&format!("rawbin: unresolvable symbol: {}", s.0))
            }
        }
        // now that the previous for is complete, the immutable reference is dropped
        // we can now safely, per rust access rules, mutate
        for z in symresv {
            i.0.fulfill_symbol(z.0, z.1);
        }
        // all the symbols are now fulfilled, so use the helper to insert the instr in
        crate::bbu::outs::vec_update(&i.0.get_output_bytes(), dest, np);
    }
}*/

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
        _ => panic!("unknown arch"),
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
                    //_ => lpanic("unsupported op type"),
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
