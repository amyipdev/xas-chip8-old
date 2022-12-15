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

use crate::bbu::ArgSymbol;
use crate::bbu::DatSize;
use crate::bbu::GenScal;
use crate::bbu::PtrSize;
use crate::bbu::SymConv;
use crate::errors::lpanic;
//use crate::bbu::DisSize;
use crate::bbu::parse_arg;
use crate::bbu::tlen;
use crate::bbu::ArchMcrInst;
use crate::bbu::RcSym;

use std::str::FromStr;

pub type P6502DatSize = GenScal<u8>;
// TODO: since C8 doesn't use dissize,
// should we make a type of size zero for it?
pub type P6502DisSize = GenScal<u8>;
pub type P6502PtrSize = GenScal<u16>;
pub type P6502SymA = ArgSymbol<P6502PtrSize, P6502DatSize>;
pub struct P6502Symbol {
    pub i: P6502SymA,
}
#[derive(Copy, Clone)]
pub enum P6502ArchReg {
    A,
    X,
    Y,
}
impl FromStr for P6502ArchReg {
    // TODO: get better error type
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            lpanic("p6502: unknown register");
        }
        Ok(match s.chars().nth(2).unwrap() {
            'a' | 'A' => Self::A,
            'x' | 'X' => Self::X,
            'y' | 'Y' => Self::Y,
            _ => lpanic("p6502: unknown register"),
        })
    }
}
impl crate::bbu::ArchReg for P6502ArchReg {}
pub type P6502Arg = crate::bbu::ArchArg<P6502PtrSize, P6502DatSize, P6502DisSize, P6502ArchReg>;

// TODO: make a macro that can automatically generate these
// since they're usually about the same
impl SymConv for P6502Symbol {
    fn from_ptr<T: PtrSize>(a: T) -> Self {
        Self {
            i: ArgSymbol::Pointer(Box::new(<P6502PtrSize as PtrSize>::from_int::<u16>(
                a.extract_int::<u16>(),
            ))),
        }
    }
    fn from_dat<T: DatSize>(a: T) -> Self {
        Self {
            i: ArgSymbol::Data(Box::new(<P6502DatSize as DatSize>::from_int::<u8>(
                a.extract_int::<u8>(),
            ))),
        }
    }
    fn into_ptr<T: PtrSize, E: crate::bbu::Integral>(&self) -> T {
        T::from_int(PtrSize::extract_int::<E>(&**self.i.unwrap_ptr().unwrap()))
    }
}

// TODO: same deal here, should be able to generate these with a macro
impl<T: SymConv> crate::bbu::ArchSym<T> for P6502Symbol {
    fn get_uk_sym(&self) -> Option<RcSym> {
        match &self.i {
            ArgSymbol::UnknownPointer(i) | ArgSymbol::UnknownData(i) => Some(i.clone()),
            _ => None,
        }
    }
    fn set_sym(&mut self, _a: T) -> () {
        unimplemented!()
    }
}

// TODO: make a global gim, that takes in the identifier, i, as
// well as the identifier of the symbol type
macro_rules! gim {
    ($n:ident,$i:ident) => {{
        Box::new(<$n as ArchMcrInst<P6502Symbol>>::get_lex($i.args))
    }};
}

pub fn get_instruction<T: SymConv>(i: crate::parser::ParsedInstruction) -> Box<dyn ArchMcrInst<T>> {
    match i.instr.to_lowercase().as_str() {
        "inx" => gim!(P6502_inx, i),
        "iny" => gim!(P6502_iny, i),
        "dex" => gim!(P6502_dex, i),
        "dey" => gim!(P6502_dey, i),
        "tax" => gim!(P6502_tax, i),
        "txa" => gim!(P6502_txa, i),
        "tay" => gim!(P6502_tay, i),
        "tya" => gim!(P6502_tya, i),
        "tsx" => gim!(P6502_tsx, i),
        "txs" => gim!(P6502_txs, i),
        "pha" => gim!(P6502_pha, i),
        "pla" => gim!(P6502_pla, i),
        "php" => gim!(P6502_php, i),
        "plp" => gim!(P6502_plp, i),
        "rts" => gim!(P6502_rts, i),
        "rti" => gim!(P6502_rti, i),
        "clc" => gim!(P6502_clc, i),
        "sec" => gim!(P6502_sec, i),
        "cld" => gim!(P6502_cld, i),
        "sed" => gim!(P6502_sed, i),
        "cli" => gim!(P6502_cli, i),
        "sei" => gim!(P6502_sei, i),
        "clv" => gim!(P6502_clv, i),
        "brk" => gim!(P6502_brk, i),
        "nop" => gim!(P6502_nop, i),
        "asl" => {
            if arg_is_accum(&i.args) {
                return gim!(P6502A_asl, i);
            }
            unimplemented!()
        }
        "lsr" => {
            if arg_is_accum(&i.args) {
                return gim!(P6502A_lsr, i);
            }
            unimplemented!()
        }
        "rol" => {
            if arg_is_accum(&i.args) {
                return gim!(P6502A_rol, i);
            }
            unimplemented!()
        }
        "ror" => {
            if arg_is_accum(&i.args) {
                return gim!(P6502A_ror, i);
            }
            unimplemented!()
        }
        "lda" => {
            match tlen(&i.args) {
                1 => {
                    if arg_is_dir(&i.args) {
                        return gim!(P6502I_lda, i);
                    }
                    unimplemented!()
                }
                _ => unimplemented!()
            }
        }
        "ldx" => {
            match tlen(&i.args) {
                1 => {
                    if arg_is_dir(&i.args) {
                        return gim!(P6502I_ldx, i);
                    }
                    unimplemented!()
                }
                _ => unimplemented!()
            }
        }
        "ldy" => {
            match tlen(&i.args) {
                1 => {
                    if arg_is_dir(&i.args) {
                        return gim!(P6502I_ldy, i);
                    }
                    unimplemented!()
                }
                _ => unimplemented!()
            }
        }
        "adc" => {
            match tlen(&i.args) {
                1 => {
                    if arg_is_dir(&i.args) {
                        return gim!(P6502I_adc, i);
                    }
                    unimplemented!()
                }
                _ => unimplemented!()
            }
        }
        "sbc" => {
            match tlen(&i.args) {
                1 => {
                    if arg_is_dir(&i.args) {
                        return gim!(P6502I_sbc, i);
                    }
                    unimplemented!()
                }
                _ => unimplemented!()
            }
        }
        "and" => {
            match tlen(&i.args) {
                1 => {
                    if arg_is_dir(&i.args) {
                        return gim!(P6502I_and, i);
                    }
                    unimplemented!()
                }
                _ => unimplemented!()
            }
        }
        "ora" => {
            match tlen(&i.args) {
                1 => {
                    if arg_is_dir(&i.args) {
                        return gim!(P6502I_ora, i);
                    }
                    unimplemented!()
                }
                _ => unimplemented!()
            }
        }
        "eor" => {
            match tlen(&i.args) {
                1 => {
                    if arg_is_dir(&i.args) {
                        return gim!(P6502I_eor, i);
                    }
                    unimplemented!()
                }
                _ => unimplemented!()
            }
        }
        "cmp" => {
            match tlen(&i.args) {
                1 => {
                    if arg_is_dir(&i.args) {
                        return gim!(P6502I_cmp, i);
                    }
                    unimplemented!()
                }
                _ => unimplemented!()
            }
        }
        "cpx" => {
            match tlen(&i.args) {
                1 => {
                    if arg_is_dir(&i.args) {
                        return gim!(P6502I_cpx, i);
                    }
                    unimplemented!()
                }
                _ => unimplemented!()
            }
        }
        "cpy" => {
            match tlen(&i.args) {
                1 => {
                    if arg_is_dir(&i.args) {
                        return gim!(P6502I_cpy, i);
                    }
                    unimplemented!()
                }
                _ => unimplemented!()
            }
        }
        "bit" => {
            match tlen(&i.args) {
                1 => {
                    if arg_is_dir(&i.args) {
                        return gim!(P6502I_bit, i);
                    }
                    unimplemented!()
                }
                _ => unimplemented!()
            }
        }
        "hlt" | // illegal opcodes time!
        "kil" |
        "jam" => gim!(P6502_hlt, i),
        _ => lpanic("p6502: unknown instruction")
    }
}

// This goes back to the whole concept of
// Why can't we create these macros with macros?
macro_rules! make_implied {
    ($nm:ident,$op:expr) => {
        // TODO: do these really need to be public?
        #[allow(non_camel_case_types)]
        pub struct $nm {}
        impl<T: SymConv> ArchMcrInst<T> for $nm {
            fn get_output_bytes(&self) -> Vec<u8> {
                Vec::from(($op as u8).to_be_bytes())
            }
            fn get_lex(_a: Option<Vec<String>>) -> Self {
                Self {}
            }
            fn check_symbols(&self) -> bool {
                false
            }
            // TODO: replace tuple with UnresSymInfo
            fn get_symbols(&self) -> crate::bbu::USIWrap {
                None
            }
            fn get_length(&self) -> crate::bbu::SymbolPosition {
                1
            }
            // TODO: wait, with the current rawbin structure,
            // are placeholders really necessary anymore?
            fn get_placeholder(&self) -> Vec<u8> {
                // this will never be called
                // if it is, something is wrong
                unimplemented!()
            }
            fn fulfill_symbol(&mut self, _s: &T, _p: crate::bbu::SymbolPosition) -> () {}
        }
    };
}

macro_rules! make_immediate {
    ($nm:ident,$op:expr) => {
        // TODO: do these really need to be public?
        #[allow(non_camel_case_types)]
        pub struct $nm {
            j: P6502SymA
        }
        impl<T: SymConv> ArchMcrInst<T> for $nm {
            fn get_output_bytes(&self) -> Vec<u8> {
                vec![($op as u8), self.j.unwrap_data().unwrap().i as u8]
            }
            fn get_lex(a: Option<Vec<String>>) -> Self {
                if let Some(ref i) = a {
                    if i.len() != 1 {
                        lpanic("p6502: wrong argument count");
                    }
                    let z: P6502Arg = parse_arg(&a.unwrap()[0]).unwrap();
                    Self {j: z.unwrap_direct().unwrap().clone()}
                } else {
                    lpanic("p6502: received None for a");
                }
            }
            fn check_symbols(&self) -> bool {
                match self.j {
                    ArgSymbol::UnknownData(_) => true,
                    _ => false,
                }
            }
            // TODO: replace tuple with UnresSymInfo
            fn get_symbols(&self) -> crate::bbu::USIWrap {
                match self.j {
                    ArgSymbol::UnknownData(ref a) => Some(vec![(a.clone(), 0)]),
                    _ => None,
                }
            }
            fn get_length(&self) -> crate::bbu::SymbolPosition {
                2
            }
            // TODO: wait, with the current rawbin structure,
            // are placeholders really necessary anymore?
            fn get_placeholder(&self) -> Vec<u8> {
                unimplemented!()
            }
            fn fulfill_symbol(&mut self, s: &T, p: crate::bbu::SymbolPosition) -> () {
                match p {
                    0 => self.j = ArgSymbol::Data(Box::new(s.into_ptr::<P6502DatSize, u8>())),
                    _ => lpanic("p6502: unknown positional"),
                }
            }
        }
    }
}

// TODO: shorten all symbol names
// implied
make_implied!(P6502_inx, 0xe8);
make_implied!(P6502_iny, 0xc8);
make_implied!(P6502_dex, 0xca);
make_implied!(P6502_dey, 0x88);
make_implied!(P6502_tax, 0xaa);
make_implied!(P6502_txa, 0x8a);
make_implied!(P6502_tay, 0xa8);
make_implied!(P6502_tya, 0x98);
make_implied!(P6502_tsx, 0xba);
make_implied!(P6502_txs, 0x9a);
make_implied!(P6502_pha, 0x48);
make_implied!(P6502_pla, 0x68);
make_implied!(P6502_php, 0x08);
make_implied!(P6502_plp, 0x28);
make_implied!(P6502_rts, 0x60);
make_implied!(P6502_rti, 0x40);
make_implied!(P6502_clc, 0x18);
make_implied!(P6502_sec, 0x38);
make_implied!(P6502_cld, 0xd8);
make_implied!(P6502_sed, 0xf8);
make_implied!(P6502_cli, 0x58);
make_implied!(P6502_sei, 0x78);
make_implied!(P6502_clv, 0xb8);
make_implied!(P6502_brk, 0x00);
make_implied!(P6502_nop, 0xea);
// accumulator
make_implied!(P6502A_asl, 0x0a);
make_implied!(P6502A_lsr, 0x4a);
make_implied!(P6502A_rol, 0x2a);
make_implied!(P6502A_ror, 0x6a);
// immediates
make_immediate!(P6502I_lda, 0xa9);
make_immediate!(P6502I_ldx, 0xa2);
make_immediate!(P6502I_ldy, 0xa0);
make_immediate!(P6502I_adc, 0x69);
make_immediate!(P6502I_sbc, 0xe9);
make_immediate!(P6502I_and, 0x29);
make_immediate!(P6502I_ora, 0x09);
make_immediate!(P6502I_eor, 0x49);
make_immediate!(P6502I_cmp, 0xc9);
make_immediate!(P6502I_cpx, 0xe0);
make_immediate!(P6502I_cpy, 0xc0);
make_immediate!(P6502I_bit, 0x89);
// illegal opcodes
make_implied!(P6502_hlt, 0x02);

fn arg_is_accum(i: &Option<Vec<String>>) -> bool {
    // thanks to the wonderful rust devs, combining the if
    // statements is unstable. wonderful!
    // TODO: safety checks (gbl) to prevent panics
    if let Some(v) = i {
        return v.len() == 1 && v[0].to_lowercase() == "%a";
    }
    false
}

fn arg_is_dir(i: &Option<Vec<String>>) -> bool {
    if let Some(v) = i {
        return v[0].chars().next() == Some('$');
    }
    false
}
