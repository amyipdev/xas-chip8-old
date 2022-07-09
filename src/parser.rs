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

// TODO: box analysis
// how much should be stored in boxes... returning huge structs from functions
// isn't always the best idea, we might want to consider boxes (+ saves stack space)

// TODO: trait analysis
// figure out what needs to be implemented, and should be implemented
// and what should be removed and/or manually implemented for efficiency

// VecDeque is used for overall compatibility
// and to lower binary size. However, we don't use double-ended
// functionality - usage is purely FIFO
// TODO: analyze if there is a better solution
use std::collections::VecDeque;

// TODO: custom error support
// for now, we just use std::num::ParseIntError which is disgusting
// what's annoying is having to map to any existing error type

// TODO: manual implementation of Debug, Display for parsed structs

/// Instruction which has been parsed by a Parser.
/// Holds the instruction name as a string, and a vector of the arguments.
#[derive(Clone, Debug)]
pub struct ParsedInstruction {
    /// Holds a string representing the name of the instruction.
    pub instr: String,
    /// Holds a string representing the arguments of the instruction.
    pub args: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct ParsedMacro {
    pub mcr: String,
    pub args: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub enum ParsedOperation {
    Instruction(ParsedInstruction),
    Macro(ParsedMacro),
}

/// Structure holding a queue of parsed and to-be parsed entries.
/// Uses implemented methods for construction and interaction.
// TODO: document q-d meaning cross-project
pub struct Parser {
    /// Internally accessed queue of strings to be parsed.
    /// Each string represents exactly one line.
    q: VecDeque<String>,
    /// Internally accessed queue of instructions.
    /// Passed later to the lexer.
    d: VecDeque<ParsedOperation>,
}

impl std::str::FromStr for Parser {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: explicitly list type of split
        // TODO: consider putting directly in the Ok()
        let split = s.lines();
        Ok(Parser {
            q: VecDeque::from_iter(split.map(|s| s.to_string())),
            d: VecDeque::new(),
        })
    }
}

// TODO: more ways to import lines (iterators, files, etc)
// no need to worry about stdin - that's client-side

// TODO: optimize, this is terribly inefficient
// TODO: if keeping do-it-self, consider box or array of width 2
fn single_split(s: String) -> (String, String) {
    let mut flag: bool = true;
    let mut res: (String, String) = (String::new(), String::new());
    for c in s.chars() {
        if flag {
            if !c.is_whitespace() {
                res.0.push(c);
            } else {
                flag = false;
                res.1.push(c);
            }
        } else {
            res.1.push(c);
        }
    }
    res
}

impl Parser {
    pub fn pop_vdq(&mut self) -> VecDeque<ParsedOperation> {
        std::mem::replace(&mut self.d, VecDeque::new())
    }

    pub fn peek_queued(&self) -> Option<&String> {
        self.q.front()
    }

    fn pop_queued(&mut self) -> Option<String> {
        self.q.pop_front()
    }

    // TODO: find more efficient method
    pub fn drop_queued(&mut self) -> () {
        // discard value
        self.pop_queued();
    }

    pub fn peek_parsed(&self) -> Option<&ParsedOperation> {
        self.d.front()
    }

    pub fn pop_parsed(&mut self) -> Option<ParsedOperation> {
        self.d.pop_front()
    }

    pub fn parse_all(&mut self) -> () {
        while self.parse_line() {}
    }

    // TODO: parse_all
    pub fn parse_line(&mut self) -> bool {
        // rust doesn't know how to read assignment context properly
        // we need to predefine it for scoping issues... can't define as null
        // FIXME if there's a better way to null-init a string (change to note otherwise)
        let mut a: String = String::new();
        if let Some(s) = self.pop_queued() {
            a = s;
        } else {
            return false;
        }
        // TODO: consider changing more matches to regex (do perf analysis)
        // TODO: try limiting size of regex crate if possible given the few invocations
        let re: regex::Regex = regex::Regex::new(r"//|;").unwrap();
        // TODO: investigate removing unwrap somehow, what happens here? can it panic?
        a = re.split(a.trim_start()).next().unwrap().to_string();
        // skip empty lines
        if a == "" {
            return true;
        }
        let b: (String, String) = single_split(a);
        let mut ar: Vec<String> = vec![];
        if b.1.trim_start() != "" {
            for n in b.1.split(",") {
                ar.push(n.trim().to_string());
            }
        }
        let oar: Option<Vec<String>> = {
            if ar.len() != 0 {
                Some(ar)
            } else {
                None
            }
        };
        // Current implementations guarantee that there will always be one character
        // If single_split's implementation changes, change this
        // unwrap() is safe - no possible error/panic - because of this
        if b.0.chars().nth(0).unwrap() != '.' {
            self.d
                .push_back(ParsedOperation::Instruction(ParsedInstruction {
                    instr: b.0,
                    args: oar,
                }));
        } else {
            // TODO: put iterator type here
            let mut ns = b.0.chars();
            ns.next();
            self.d.push_back(ParsedOperation::Macro(ParsedMacro {
                mcr: ns.collect::<String>(),
                args: oar,
            }));
        }

        true
    }
}
