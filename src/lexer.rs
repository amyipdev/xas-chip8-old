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

// TODO: readme (rst? md? decide)
// TODO: styling guide
//       - mod/crate/use order
// TODO: CI
// TODO: tests
// TODO: examples
// TODO: publish on crates.io
// TODO: fill out crate manifest
// TODO: use cargo-msrv to determine minimum rust version
// TODO: proper logging via interface w/ log crate (use all 5 levels)
//       - consider hiding behind a feature
// TODO: when doing readme, set up badges (don't forget codecov)
// TODO: benchmarks, tests, examples
// TODO: migrate TODOs to GitHub issues/milestones
// TODO: documentation + other repo setup
// TODO: set github to default to rebasing instead of merges

use std::collections::VecDeque;

// TODO: replace all (project-wide) lookup tables with static strings
// that go to enums with https://github.com/rust-phf/rust-phf

// TODO: turn this into a block comment
// Tree logic:
// - Vec<LexSection>
//   - LexSection S1 (flags0)
//     - LexIdLabel IL0
//       - LexInstruction 1
//       - LexInstruction 2
//       - LexMacro 3
//       - LexInstruction 4
//     - LexLabel L1
//       - LexInstruction 5
//       - LexInstruction 6
//     - LexLabel L2
//   - LexSection S2 (flags1)
//     - LexLabel L3
//
// Each section has flags that change output methods. This is needed
// for sections like rodata and BSS. Flags are set based on input ID
// type from platform.

pub enum LexOperation {
    Instruction(Box<dyn crate::bbu::ArchInstruction>),
    Macro(Box<dyn crate::bbu::ArchMacro>),
}

impl std::fmt::Debug for LexOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexOperation::Instruction(ref i) => {
                write!(f, "ArchInstruction: {:02x?}", i.get_output_bytes())
            }
            LexOperation::Macro(ref j) => write!(f, "ArchMacro: {:02x?}", j.get_output_bytes()),
        }
    }
}

// TODO: consider changing all instances working with lexop to extract/place
impl LexOperation {
    // TODO: has to be a better way to optimize this
    // pulling things out of enums is such a size increaser
    // would struct with associated non-holding enum be better?
    // does ZCA (zero-cost abstractions) apply here?
    // this needs a lot of analysis!
    // TODO: optimize double call
    pub fn extract_bytes(&self) -> Vec<u8> {
        match self {
            LexOperation::Instruction(a) => a.get_output_bytes(),
            LexOperation::Macro(b) => b.get_output_bytes()
        }
    }
}

pub type LexIdLabel = Vec<LexOperation>;

#[derive(Debug)]
pub struct LexLabel {
    pub name: String,
    pub ops: LexIdLabel,
}

// TODO: consider rep. man. debug, see for all Lex types
#[derive(Debug)]
pub enum LexLabelType {
    Base(LexIdLabel),
    Std(LexLabel),
}

// TODO: change anything interacting with LexLabelType to use extract
// TODO: make extract_mut to allow placement
impl LexLabelType {
    pub fn extract(&self) -> (&LexIdLabel, Option<&String>) {
        // TODO: is this best?
        match self {
            LexLabelType::Base(ref a) => (a, None),
            LexLabelType::Std(ref b) => (&b.ops, Some(&b.name))
        }
    }
}

#[derive(Debug)]
pub struct LexSection {
    pub name: String,
    pub labels: Vec<LexLabelType>,
}

// TODO: consider replacing with manual Debug trait
#[derive(Debug)]
pub struct Lexer {
    // This method requires that the Lexer takes ownership of a
    // Parser's d-out.
    q: VecDeque<crate::parser::ParsedOperation>,
    d: Vec<LexSection>,
    // Current section
    cs: Option<LexSection>,
    // Current label
    cl: Option<LexLabelType>,
    // Platform
    p: crate::platform::Platform,
}

// TODO: explore whether vec![] is better/worse than Vec::new()
impl Lexer {
    // push_label does not generate a new label because it could be called
    // on the last label of a section - we wouldn't want a new std label,
    // as that new section needs its own id label
    // TODO: better error handling!
    fn push_label(&mut self) -> () {
        // take ownership of label, Option::take automatically replaces with None
        // NHNF, but should indicate if the label didn't exist... no blind C-style crap
        if let Some(n) = self.cl.take() {
            // no harm no foul if self.cs is None, but could have better handling
            if let Some(ref mut p) = &mut self.cs {
                p.labels.push(n);
            }
        }
    }

    fn gen_label(&mut self, name: String) -> () {
        self.cl = Some(LexLabelType::Std(LexLabel {
            name: name,
            ops: vec![],
        }));
    }

    // Code concept is copied over from push_label
    // Effectively the same
    // TODO: fix the same NHNF issues with push_label here
    fn push_section(&mut self) -> () {
        self.push_label();
        // NHNF
        if let Some(n) = self.cs.take() {
            self.d.push(n);
        }
    }

    // TODO: proper error handling
    // TODO: allow specifying section parameters
    // TODO: shift burden of extraction onto caller, just take a string to make life easier here
    fn gen_section(&mut self, name: Option<Vec<String>>) -> () {
        if let Some(mut n) = name {
            if n.len() == 0 {
                panic!("improper arg amount passed");
            }
            // create new section
            self.cs = Some(LexSection {
                // TODO: explore speed issues with this
                // remove (might?) cause(s) a vec shuffle, which doesn't scale well (O(n))
                name: n.remove(0),
                labels: vec![],
            });
            // generate identity label
            self.cl = Some(LexLabelType::Base(vec![]));
        } else {
            panic!("improper arg amount passed");
        }
    }

    // TODO; note VDQ = VecDeque
    pub fn from_vdq(
        q: VecDeque<crate::parser::ParsedOperation>,
        p: crate::platform::Platform,
    ) -> Self {
        Lexer {
            // takes ownership of the VDQ
            q: q,
            d: vec![],
            cs: None,
            cl: None,
            p: p,
        }
    }

    // implementation taken from parser::Parser::pop_vdq
    // TODO: reduce code dup?
    pub fn pop_vdq(&mut self) -> Vec<LexSection> {
        std::mem::replace(&mut self.d, Vec::new())
    }

    // TODO: more line-by-line instructions
    // TODO: find a better solution than flushing both queues
    // TODO: this is the MAIN FUNCTION so DISTRIBUTE IT OUT MORE
    // TODO: + find more optimizations
    pub fn lex_full_queue(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(i) = self.q.pop_front() {
            if let crate::parser::ParsedOperation::Macro(j) = i {
                // Override for label beginnings
                if j.mcr.chars().last().unwrap() == ':' {
                    self.push_label();
                    // TODO: iterator optimization, don't take up as much stack space
                    let mut z = j.mcr.chars();
                    z.next_back();
                    self.gen_label(z.collect::<String>());
                    continue;
                }
                // TODO: reduce redundancy of push_section calls somehow...
                // TODO: maybe since push_section is safe-fail-out, add it to
                // gen_section()?
                // TODO: identify special sections by keeping their name, but
                // adding a flag of some kind to LexSection?
                // TODO: better matching system if not, this needs overhaul
                match j.mcr.to_lowercase().as_str() {
                    "section" | "sec" => {
                        self.push_section();
                        self.gen_section(j.args);
                    }
                    "text" | "code" => {
                        self.push_section();
                        // TODO: fix gen_section call, this is ugly asf
                        self.gen_section(Some(vec!["text".to_string()]));
                    }
                    "bss" | "data?" => {
                        self.push_section();
                        // TODO: see above
                        self.gen_section(Some(vec!["bss".to_string()]));
                    }
                    "rodata" | "const" => {
                        self.push_section();
                        self.gen_section(Some(vec!["rodata".to_string()]));
                    }
                    "data" | "dat" => {
                        self.push_section();
                        self.gen_section(Some(vec!["data".to_string()]));
                    }
                    // TODO: proper error handling!!
                    // consider just ignoring the error for now
                    // and continuing on
                    // also could be solved with logging issues
                    _ => panic!(),
                }
            // TODO: find better solution that else-if-let since it's guaranteed to end up here
            } else if let crate::parser::ParsedOperation::Instruction(j) = i {
                // push instruction onto current label
                self.push_instruction(j);
            }
        }
        self.push_section();
        Ok(())
    }

    // TODO: error handling
    fn push_instruction(&mut self, i: crate::parser::ParsedInstruction) -> () {
        // make sure we actually have a label to push to
        // labels are already guaranteed to have sections
        if let Some(ref mut j) = &mut self.cl {
            // lexing time
            // TODO: is there anything better here than match?
            // NOTE: _ is unreachable, this is a safety measure while indev
            // FIXME consider removing it in cleanup
            let op: LexOperation = match self.p.arch {
                crate::platform::PlatformArch::ChipEightRaw => {
                    LexOperation::Instruction(Box::new(crate::bbu::chip8_raw::get_instruction(i)))
                }
                _ => panic!("architecture not implemented yet"),
            };
            match j {
                LexLabelType::Base(ref mut a) => a,
                LexLabelType::Std(ref mut b) => &mut b.ops,
            }
            .push(op);
        }
        // NHNF
    }
}
