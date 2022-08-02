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

// FIXME NOTE for some architectures, details DO need to be inferred
// For instance, if a 6502 absolute value is <0x100, it is always
// in the zeropage. Also, architectures differing from "official styles"
// (Intel for x86, or the BS for chip8 created, or 6502 w/ bell)
// trying to align with the system here need TODO style guides.
// So for instance, 6502 Immediate is $ (vs normal #$), Absolute is (none)
// vs normal $, abs-indir is still ($)
// NOTE this also means different assemblies have different label reference
// requirements. for instance, while x86 JMP accepts r/m as input, 6502
// wants direct. Some also has to be inferred, as 6502 LDA goes:
// (x-indexed zero page indirect implied)
// `LDA ($nn,X)` -> `LDA ($nn),X`, which doesn't exactly seem as-original.
//
// NOTE (+1major after initial x86) support implied instruction width
// (don't need to specify `movq` unless not-enough-context)

// TODO: specify difference between TODO, FIXME, and NOTE
// TODO: actually use returned Results (global)
// TODO: better error handling system/types FIXME
// FIXME declare any FIXME-s as breaking bugs post-v0.1

// TODO: clarify in README how "collection of different assemblers" (gas)
// is different from libxas (extendable framework)

// TODO: specify BBU = "better binutils" or "basic binary units"
// TODO: clarify vec bits are in exact order, v[0] = binary[0]
// TODO: move CPU targets into new module
// TODO: separate project name internally? separate readme?
//       ties into license declaration perhaps

// TODO: generic argument types (direct, label, memory, register, etc)
// TODO: general symbol implementation

pub mod chip8_raw;
pub mod outs;

/* reference code, not currently in use
 * TODO FIXME remove as dead once full ArchArg implementation, parser exists

// NOTE: T is architecture pointer type
// TODO: use that T elsewhere for labelmaking?
pub enum ArchArg<T> {
    // TODO: consider moving Mem into an ArchMem or the like
    // modeled off of ArchReg, tuples in an enum feels wonky
    // FIXME mem needs better support for x86 - segments
    Mem(T),
    Reg(ArchReg<T>),
    // TODO: other modes
    Dir(T)
}

// TODO: more of this type declaration instead of stray boxes
pub type Register = Box<dyn DeserializeReg>;

// FIXME: actually implement ArchArg
// TODO: see lexer, do arch handling here instead of in the lexer

// NOTE: r is register name
// NOTE: d is dereference info
// TODO: include support for x86 segment registers
// fs:(eax) or whatever
// maybe this belongs up there?
pub struct ArchReg<T> {
    pub r: Register,
    // deref with no offset = Some(0)
    // no deref = None
    // TODO: split this up into struct maybe? tuple is annoying
    // NOTE: Option<(Register, u8)> is shift, maybe make it a Type TODO
    //       - also consider TODO making the internal function-extractable
    //       - or something besides a tuple
    // FIXME support segment shifts like on x86
    pub d: Option<(T, Option<(Register, u8)>)>,
    // sample instruction: (i386)
    // 8d 74 c3 04       lea 0x4(%ebx,%eax,8),%esi
    // equ:              lea esi,[ebx + 8*eax + 4]
    // NOTE: the shift number given is the raw number,
    // but it must be a power of 2. it is then for x86
    // turned into its log2, so for this it would be 3
    // (which is the maximum permitted shift)
}

// FIXME reminder to check -> Self instead of -> Name
pub trait DeserializeReg {
    fn deserialize(i: &String) -> Self
    where
        Self: Sized;
}
*/

pub trait ArchInstruction {
    fn get_output_bytes(&self) -> Vec<u8>;
    fn get_lex(a: Option<Vec<String>>) -> Self
    where
        Self: Sized;
}

pub trait ArchMacro {
    fn get_output_bytes(&self) -> Vec<u8>;
    fn get_lex(a: Option<Vec<String>>) -> Self
    where
        Self: Sized;
}

// FIXME FIXME FIXME FIXME remove this code
// TODO: optimize, minimize code deduplication
// TODO: should doubled patterns (0x0x for example) be allowed?
// trim_start_matches might allow that
// TODO: proper error handling, return None instead of .unwrap()ing
// TODO: generally better suite of argument parsing
pub fn parse_unknown_radix_u16(s: &String) -> Option<u16> {
    // TODO: is making a copy of the string really necessary?
    // TODO: would if-elseif be faster?
    // TODO: when unwrap is handled correctly below this becomes
    // less necessary
    match s.len() {
        0 => return None,
        // this also might help do away with the last TODO
        // consider rewriting this whole function later
        // also, we need a TODO type generic version
        // something easily replicable across all sizes
        // more reason to TODO rewrite all of this
        // this just works as patchwork for now
        1 => {
            if s.chars().nth(0) == Some('0') {
                return Some(0);
            }
        }
        _ => {}
    }
    // TODO: dozenal support (XE-based maybe?)
    // TODO: explore how heavily features should be made
    Some(match s.chars().nth(0).unwrap() {
        '0' => match s.chars().nth(1).unwrap() {
            // lots of duplication here
            'x' => u16::from_str_radix(s.trim_start_matches("0x"), 16).unwrap(),
            'b' => u16::from_str_radix(s.trim_start_matches("0b"), 2).unwrap(),
            // octal is autotrimmed
            _ => u16::from_str_radix(s, 8).unwrap(),
        },
        // more duplication here
        '-' => u16::from_str_radix(s.trim_start_matches("-"), 10)
            .unwrap()
            .wrapping_neg(),
        _ => u16::from_str_radix(s, 10).unwrap(),
    })
}

/*
// Parsing system
// no symbol: memory location
// $: direct value
// %: register
// (): deref
// so, on x86 to move 0x35 to ptr@rax(shift-4):
// mov $0x35,-4(%rax)
// TODO evaluate if pub is necessary
// TODO should this be an impl on ArchArg?
// FIXME properly implement error handling with result, make own error type, etc etc etc
pub fn parse_argument<T>(p: &crate::platform::Platform, a: &String) -> Option<ArchArg<T>> {
    // TODO: find a better solution than many bools (bitfield?) bools take up 1 byte each?
    let mut f_end_parth: bool = false;

    // iterator to make processing easier
    // TODO copy iterator type to all Chars, Peekable<Chars>
    let mut i: std::iter::Peekable<std::str::Chars> = a.chars().peekable();
    // a parenthesis will always be the last character
    // TODO optimize, error handling FIXME
    if a.chars().last().unwrap() == ')' {
        f_end_parth = true;
        i.next_back();
    }

    None
}

 */

// TODO: UpperCamelCase/PascalCase these trait names
// TODO: Document all these types
pub trait PTR_SIZE: core::ops::Add + core::ops::Sub + Sized {}
pub trait DAT_SIZE: core::ops::Add + core::ops::Sub + Sized {}
pub trait DIS_SIZE: core::ops::Add + core::ops::Sub + Sized {}

pub type ArchReg = String;

pub enum ArgSymbol<T: PTR_SIZE, U: DAT_SIZE> {
    Unknown(String),
    Pointer(Box<T>),
    Data(Box<U>),
}

// TODO: Box all structs? NOTE-FIXME
pub enum ArchArg<T: PTR_SIZE, U: DAT_SIZE, V: DIS_SIZE> {
    // limited to Unknown or Data
    Direct(ArgSymbol<T, U>),
    // limited to Unknown or Pointer
    Memory(ArchMem<T, U>),
    // register
    Register(ArchIndivReg<V>)
}

pub struct ArchMem<T: PTR_SIZE, U: DAT_SIZE> {
    // segment register
    pub segr: Option<ArchReg>,
    // actual memory value, must be Unknown or Pointer
    pub v: ArgSymbol<T, U>
}

pub struct ArchIndivReg<V: DIS_SIZE> {
    // segment register
    pub segr: Option<ArchReg>,
    // offset; if register is dereferenced, Some(0)
    pub disp: Option<Box<V>>,
    // actual register
    pub reg: ArchReg,
    // shift information - u8 is shift amount, in number of bits, and ArchReg is the register to be shifted
    pub shift: Option<(u8, ArchReg)>
}
