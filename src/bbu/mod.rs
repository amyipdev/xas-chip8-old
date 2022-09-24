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

use crate::errors::lpanic;

#[cfg(feature = "chip8")]
pub mod chip8;
#[cfg(feature = "chip8-raw")]
pub mod chip8_raw;
pub mod outs;

pub type SymbolPosition = u8;

pub type USIWrap = Option<Vec<(RcSym, SymbolPosition)>>;

pub trait SymConv {
    fn from_ptr<T: PtrSize>(a: T) -> Self;
    fn from_dat<T: DatSize>(a: T) -> Self;
    fn into_ptr<T: PtrSize, E: Integral>(&self) -> T;
}

pub trait ArchSym<T: SymConv> {
    fn get_uk_sym(&self) -> Option<RcSym>;
    fn set_sym(&mut self, a: T) -> ();
}

// TODO: really hate using vecs for this
pub trait ArchMcrInst<T: SymConv> {
    fn get_output_bytes(&self) -> Vec<u8>;
    fn check_symbols(&self) -> bool;
    fn get_symbols(&self) -> USIWrap;
    fn get_length(&self) -> SymbolPosition;
    // TODO: deprecate? (general project cleaning/deprecation)
    // TODO: feature `no-deprecated`, removes all deprecated code
    // TODO: feature `no-log`, removes logging
    // NOTE: maybe do the same for colored logging?
    fn get_placeholder(&self) -> Vec<u8>;
    // NOTE: should this return Result<>? Shouldn't be able to fail...
    fn fulfill_symbol(&mut self, s: &T, p: SymbolPosition) -> ();
    // TODO: is it better to put Sized in the ArchMcrInst definition?
    fn get_lex(a: Option<Vec<String>>) -> Self
    where
        Self: Sized;
}

// TODO FIXME: support symbols in macros
pub trait ArchMacro {
    fn get_output_bytes(&self) -> Vec<u8>;
    fn get_lex(a: Option<Vec<String>>) -> Self
    where
        Self: Sized;
    fn get_length(&self) -> SymbolPosition;
}

// TODO NOTE FIXME consider refactoring all of this into parser, so it happens pre-lexing?? maybe??

// TODO NOTE FIXME determine Box vs immutable reference for trait object size fitting, as well as the usage of boxes in general

// TODO: UpperCamelCase/PascalCase these trait names
// TODO: Document all these types
// TODO: NOTE clean definition duplication
pub trait PtrSize: Copy + Clone + std::str::FromStr<Err = std::num::ParseIntError> + Sized {
    // FIXME: transmute function
    // FIXME FIXME: native sizing?
    fn from_int<T: Integral>(a: T) -> Self;
    fn extract_int<T: Integral>(&self) -> T;
    fn add_int<T: Integral>(&mut self, a: T) -> ();
    fn add_ptr(&mut self, a: Self) -> ();
}
pub trait DatSize: Copy + Clone + std::str::FromStr<Err = std::num::ParseIntError> + Sized {
    fn from_int<T: Integral>(a: T) -> Self;
    fn extract_int<T: Integral>(&self) -> T;
}
pub trait DisSize: Copy + Clone + std::str::FromStr<Err = std::num::ParseIntError> + Sized {}

// NOTE NOTE NOTE
//
// This next section of code is currently sample types for PTR_SIZE, DAT_SIZE, etc
// These likely should be moved to some form of dedicated section

// TODO NOTE extremely utility
pub trait Integral:
    num_traits::WrappingNeg
    + num_traits::NumAssign
    + num_traits::cast::FromPrimitive
    + num_traits::cast::NumCast
    + Copy
    + Clone
{
}
impl<T> Integral for T where
    T: num_traits::WrappingNeg
        + num_traits::NumAssign
        + num_traits::cast::FromPrimitive
        + num_traits::cast::NumCast
        + Copy
        + Clone
{
}

// FIXME FIXME FIXME: See #5
//
// There is currently a major issue with casting between unequally sized
// sizes. If the cast is of a number larger than the type can support,
// it panics. We could make this a defined, official behavior - a "feature",
// even - but for now, this is considered a KNOWN BUG.

#[derive(Copy, Clone)]
pub struct GenScal<T: Integral> {
    pub i: T,
}

impl<T: Integral> std::str::FromStr for GenScal<T> {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            i: parse_ukr::<T>(s).unwrap(),
        })
    }
}

// TODO: reduce code dup
impl<T: Integral> PtrSize for GenScal<T> {
    fn from_int<E: Integral>(a: E) -> Self {
        Self {
            i: num_traits::cast::cast(a).unwrap(),
        }
    }
    fn extract_int<E: Integral>(&self) -> E {
        num_traits::cast::cast(self.i).unwrap()
    }
    fn add_int<E: Integral>(&mut self, a: E) -> () {
        self.i += num_traits::cast::cast(a).unwrap()
    }
    fn add_ptr(&mut self, a: Self) -> () {
        self.i += a.i
    }
}
impl<T: Integral> DatSize for GenScal<T> {
    fn from_int<E: Integral>(a: E) -> Self {
        Self {
            i: num_traits::cast::cast(a).unwrap(),
        }
    }
    fn extract_int<E: Integral>(&self) -> E {
        num_traits::cast::cast(self.i).unwrap()
    }
}
impl<T: Integral> DisSize for GenScal<T> {}

// TODO: macro for these odd-ball types
#[derive(Copy, Clone)]
pub struct Gen12 {
    pub i: u16,
}

impl std::str::FromStr for Gen12 {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            i: parse_ukr::<u16>(s).unwrap() & 0xfff,
        })
    }
}

// TODO minimize code dup
impl PtrSize for Gen12 {
    fn from_int<E: Integral>(a: E) -> Self {
        Self {
            i: num_traits::cast::cast::<E, u16>(a).unwrap() & 0xfffu16,
        }
    }
    fn extract_int<E: Integral>(&self) -> E {
        num_traits::cast::cast(self.i).unwrap()
    }
    // TODO: make all size limitations happen on extraction..?
    fn add_int<E: Integral>(&mut self, a: E) -> () {
        self.i += num_traits::cast::cast::<E, u16>(a).unwrap();
        self.i &= 0xfff;
    }
    fn add_ptr(&mut self, a: Self) -> () {
        self.i += a.i;
        self.i &= 0xfff;
    }
}
impl DatSize for Gen12 {
    fn from_int<E: Integral>(a: E) -> Self {
        Self {
            i: num_traits::cast::cast::<E, u16>(a).unwrap() | 0xfffu16,
        }
    }
    fn extract_int<E: Integral>(&self) -> E {
        num_traits::cast::cast(self.i).unwrap()
    }
}
impl DisSize for Gen12 {}

//pub struct Dat12

// NOTE another utility suite
// Unwrapping aid
fn uw_aid<T, E>(r: Result<T, E>) -> T {
    match r {
        Ok(a) => a,
        Err(_) => lpanic("unwrapping failed"),
    }
}

// NOTE: definitely a utility function, TODO
// TODO FIXME better error handling
fn parse_ukr<T: Integral>(s: &str) -> Option<T> {
    // TODO: optimize, this is horribly inefficient to have to recollect the iterator
    // End immediately on empty strings
    if s.len() == 0 {
        return None;
    }
    let sign = if s.chars().next() == Some('-') { 1 } else { 0 };
    let v: T;
    // TODO typehint
    let sl = &s[sign..];
    // Slices are valid still one over, and since the length is >=1, starting at 1 is fine
    // TODO minimize code dup
    if sl.starts_with("0x") {
        v = uw_aid(T::from_str_radix(&sl[2..], 16));
    } else if sl.starts_with("0b") {
        v = uw_aid(T::from_str_radix(&sl[2..], 2));
    } else if sl.starts_with("0d") {
        v = uw_aid(T::from_str_radix(&sl[2..], 12));
    } else if sl.starts_with("0") {
        v = uw_aid(T::from_str_radix(&sl[1..], 8));
    } else {
        // NOTE is _radix really necessary here?
        v = uw_aid(T::from_str_radix(sl, 10));
    }
    Some(if sign == 1 { v.wrapping_neg() } else { v })
}

// TODO NOTE before 1.0, get a full no-std impl working for all of the crate
// TODO FIXME better error type
pub trait ArchReg: Copy + Clone + std::str::FromStr<Err = std::num::ParseIntError> + Sized {}

// TODO: migrate to Rc<str> to avoid cache misses
pub type RcSym = std::rc::Rc<str>;

#[derive(Clone)]
pub enum ArgSymbol<T: PtrSize, U: DatSize> {
    UnknownPointer(RcSym),
    UnknownData(RcSym),
    Pointer(Box<T>),
    Data(Box<U>),
}

// condense these unwrapper functions TODO
impl<T: PtrSize, U: DatSize> ArgSymbol<T, U> {
    pub fn unwrap_unknown_ptr(&self) -> Option<RcSym> {
        if let ArgSymbol::UnknownPointer(n) = self {
            return Some(n.clone());
        } else {
            return None;
        }
    }

    pub fn unwrap_unknown_data(&self) -> Option<RcSym> {
        if let ArgSymbol::UnknownData(n) = self {
            return Some(n.clone());
        } else {
            return None;
        }
    }

    pub fn unwrap_ptr(&self) -> Option<&Box<T>> {
        if let ArgSymbol::Pointer(n) = self {
            return Some(n);
        } else {
            return None;
        }
    }

    pub fn unwrap_data(&self) -> Option<&Box<U>> {
        if let ArgSymbol::Data(n) = self {
            return Some(n);
        } else {
            return None;
        }
    }
}

// TODO: Box all structs? NOTE-FIXME
pub enum ArchArg<T: PtrSize, U: DatSize, V: DisSize, W: ArchReg> {
    // limited to Unknown or Data
    Direct(ArgSymbol<T, U>),
    // limited to Unknown or Pointer
    Memory(ArchMem<T, U, W>),
    // register
    Register(ArchIndivReg<V, W>),
}

impl<T: PtrSize, U: DatSize, V: DisSize, W: ArchReg> ArchArg<T, U, V, W> {
    pub fn unwrap_direct(&self) -> Option<&ArgSymbol<T, U>> {
        if let ArchArg::Direct(n) = self {
            return Some(n);
        } else {
            return None;
        }
    }

    pub fn unwrap_memory(&self) -> Option<&ArchMem<T, U, W>> {
        if let ArchArg::Memory(n) = self {
            return Some(n);
        } else {
            return None;
        }
    }

    pub fn unwrap_register(&self) -> Option<&ArchIndivReg<V, W>> {
        if let ArchArg::Register(n) = self {
            return Some(n);
        } else {
            return None;
        }
    }
}

pub struct ArchMem<T: PtrSize, U: DatSize, W: ArchReg> {
    // segment register
    pub segr: Option<Box<W>>,
    // actual memory value, must be Unknown or Pointer
    pub v: ArgSymbol<T, U>,
}

pub struct ArchIndivReg<V: DisSize, W: ArchReg> {
    // segment register
    pub segr: Option<Box<W>>,
    // offset; if register is dereferenced, Some(0)
    pub disp: Option<Box<V>>,
    // actual register
    pub reg: Box<W>,
    // shift information - u8 is shift amount, in number of bits, and ArchReg is the register to be shifted
    pub shift: Option<(u8, Box<W>)>,
}

// TODO: consider struct, boxed struct
// V: DIS_SIZE, W: ArchReg
type RegClauseInfo<V, W> = (Option<Box<V>>, Box<W>, Option<(u8, Box<W>)>);

// TODO better error handling, Result instead of Option
// TODO should this be an impl on ArchArg? makes a lot more sense
// TODO FIXME return here and do major cleanup
// TODO FIXME FIXME FIXME FIXME this should be a FromStr off ArchArg once error handling is correct
pub fn parse_arg<T: PtrSize, U: DatSize, V: DisSize, W: ArchReg>(
    s: &String,
) -> Option<ArchArg<T, U, V, W>> {
    // Detect the presence of a segment register
    let cv: Vec<String> = s.split(":").map(|x| x.to_string()).collect();
    if cv.len() != 1 {
        if cv.len() == 2 {
            let sr: Box<W> = Box::new(W::from_str(&cv[0]).unwrap());
            if cv[1].contains('%') {
                let vs: RegClauseInfo<V, W> = parse_reg_clause(&cv[1]);
                return Some(ArchArg::Register(ArchIndivReg {
                    segr: Some(sr),
                    disp: vs.0,
                    reg: vs.1,
                    shift: vs.2,
                }));
            } else {
                return Some(ArchArg::Memory(ArchMem {
                    segr: Some(sr),
                    v: extract_mem_symbol(&cv[1]),
                }));
            }
        } else {
            return None;
        }
    }
    // cv length must be 1
    if cv[0].contains('%') {
        // register time
        // NOTE NOTE NOTE NOTE TODO: there's a lot of code duplication here that needs to be cleaned up
        let vs: RegClauseInfo<V, W> = parse_reg_clause(&cv[0]);
        return Some(ArchArg::Register(ArchIndivReg {
            segr: None,
            disp: vs.0,
            reg: vs.1,
            shift: vs.2,
        }));
    } else {
        if cv[0].chars().next() == Some('$') {
            return Some(ArchArg::Direct(get_direct_value(
                &cv[0].split_at(1).1.to_string(),
            )));
        } else {
            return Some(ArchArg::Memory(ArchMem {
                segr: None,
                v: extract_mem_symbol(&cv[0]),
            }));
        }
    }
}

fn get_direct_value<T: PtrSize, U: DatSize>(s: &String) -> ArgSymbol<T, U> {
    // presume string already has `$` stripped
    // TODO: octal support with 0o
    // TODO: check for full dozenal support
    // NOTE: document that this restricts labels to beginning with letters
    if s.chars().next().unwrap().is_numeric() {
        return ArgSymbol::Data(Box::new(U::from_str(s).unwrap()));
    } else {
        return ArgSymbol::UnknownData(RcSym::from(s.as_str()));
    }
}

// TODO NOTE random thought: a lot of parsing and lexing can be multithreaded, maybe make an option to do that in the future
// for very large files would see heavy acceleration
// also, NOTE CFI support, whatever the hell it is

fn parse_reg_clause<V: DisSize, W: ArchReg>(s: &String) -> RegClauseInfo<V, W> {
    // First determine if there's an internal clause, there must be to even have a displacement
    let mut disp: Option<Box<V>> = None;
    let mut p: String = s.clone();
    if p.contains('(') {
        // item is claused, now split on the parentheses
        // TODO: is there a better method than Vec?
        let v: Vec<String> = p.split('(').map(|x| x.to_string()).collect();
        p = v[1].trim_end_matches(')').to_string();
        if v[0] != "" {
            disp = Some(Box::new(V::from_str(&v[0]).unwrap()));
        }
    }
    // predicate now has no parentheses
    // TODO: helper method to take splits and not explicitly map?
    // NOTE may be able to do &str directly here and in more NOTE places around the project
    let secs: Vec<String> = p.split(',').map(|x| x.to_string()).collect();
    // size guaranteed >= 1
    let reg: Box<W> = Box::new(W::from_str(&secs[0]).unwrap());
    let mut shift: Option<(u8, Box<W>)> = None;
    if secs.len() >= 2 {
        shift = Some((
            if secs.len() == 3 {
                secs[2].parse().unwrap()
            } else {
                0
            },
            Box::new(W::from_str(&secs[1]).unwrap()),
        ));
    }
    (disp, reg, shift)
}

fn extract_mem_symbol<T: PtrSize, U: DatSize>(s: &String) -> ArgSymbol<T, U> {
    let ns: String = trim_parentheses(s);
    // NOTE bell technically requires that if the dollar sign is in parentheses, accept it as valid
    // we ignore that for now, but it's a future consideration
    // also, dollar with no parentheses is an error
    // TODO in general, more illegal syntax catching

    // + NOTE better error handling
    // NOTE TODO don't run double, also should it be .any(|x|)?
    if ns.chars().next().unwrap().is_numeric() || ns.chars().next().unwrap() == '-' {
        return ArgSymbol::Pointer(Box::new(T::from_str(&ns).unwrap()));
    } else {
        return ArgSymbol::UnknownPointer(RcSym::from(ns.as_str()));
    }
}

// TODO NOTE utils function?
fn trim_parentheses(s: &String) -> String {
    s.trim_start_matches('(').trim_end_matches(')').to_string()
}

pub struct GenSymbol<T: PtrSize + DatSize`> {
    pub i: ArgSymbol<T, T>
}

impl<V: PtrSize + DatSize> crate::bbu::SymConv for GenSymbol<V> {
    fn from_ptr<T: PtrSize>(a: T) -> Self {
        Self {
            i: crate::bbu::ArgSymbol::Pointer(Box::new(
                <V as PtrSize>::from_int::<V>(a.extract_int::<V>()),
            )),
        }
    }
    fn from_dat<T: DatSize>(a: T) -> Self {
        Self {
            i: crate::bbu::ArgSymbol::Data(Box::new(<V as DatSize>::from_int::<V>(
                a.extract_int::<V>(),
            ))),
        }
    }
    fn into_ptr<T: PtrSize, E: Integral>(&self) -> T {
        T::from_int(crate::bbu::PtrSize::extract_int::<E>(
            &**self.i.unwrap_ptr().unwrap(),
        ))
    }

}

macro_rules! be_mcr {
    ($nm:ident,$u:ty,$len:expr) => {
        pub struct $nm {
            x: $u,
        }
        impl ArchMcrInst<GenSymbol<$u>> for $nm {
            fn get_output_bytes(&self) -> Vec<$u> {
                unimplemented!()
            }
            fn check_symbols(&self) -> bool {
                unimplemented!()
            }
            fn get_symbols(&self) -> Option<Vec<UnresSymInfo>> {
                unimplemented!()
            }
            fn get_placeholder(&self) -> Vec<u8> {
                unimplemented!()
            }
            fn fulfill_symbol(&mut self, s: &T, p: SymbolPosition) -> () {
                unimplemented!()
            }
            fn get_lex(a: Option<Vec<String>>) -> Self {
                Self {
                    x: parse_ukr(&a.unwrap()[0]).unwrap(),
                }
            }
            fn get_length(&self) -> SymbolPosition {$len}
        }
    };
}

macro_rules! le_mcr {
    ($nm:ident,$u:ty,$len:expr) => {
        pub struct $nm {
            x: $u,
        }
        impl ArchMcrInst<$u> for $nm {
            fn get_output_bytes(&self) -> Vec<$u> {
                unimplemented!()
            }
            fn check_symbols(&self) -> bool {
                unimplemented!()
            }
            fn get_symbols(&self) -> Option<Vec<UnresSymInfo>> {
                unimplemented!()
            }
            fn get_placeholder(&self) -> Vec<u8> {
                unimplemented!()
            }
            fn fulfill_symbol(&mut self, s: &T, p: SymbolPosition) -> () {
                unimplemented!()
            }
            fn get_lex(a: Option<Vec<String>>) -> Self {
                Self { x: parse_ukr(&a.unwrap()[0]).unwrap() }
            }
            fn get_length(&self) -> SymbolPosition {$len}
        }
    };
}

be_mcr!(BigByte, GenScal<u8>);
//be_mcr!(BigWord, u16);
