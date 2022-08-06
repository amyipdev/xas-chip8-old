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

pub fn run_output<T: crate::bbu::SymConv>(
    src: Vec<crate::lexer::LexSection<T>>,
    dest: &mut Vec<u8>,
    plat: &crate::platform::Platform,
) -> () {
    // TODO: unresolved symbols! while we build the label tree, we don't link them yet
    // TODO: dynamically size LabelTree entries... we want architecture pointer sizes
    // overall TODO fix pointer sizes so that they are correct and dynamic to architecture
    let mut lt: crate::bbu::outs::LabelTree<u64> = crate::bbu::outs::LabelTree::new();
    // code duplication issue - redone across many files
    // perhaps we should store the LT, offset, and position in a pre-generated structure?
    // after all run_output calls do run through the global run_output which can pass a struct
    // could actually pass everything as just one structure pointer... TODO
    // also, NOTE offset is only used for implicit binary linking
    let offset: u64 = crate::bbu::outs::get_offset(plat);
    let mut pos: u64 = 0;

    // TODO: list a label
    for section in src {
        for label_t in &section.labels {
            let label: (&crate::lexer::LexIdLabel<T>, Option<&String>) = label_t.extract();
            if let Some(n) = label.1 {
                // TODO: another candidate for `&str`ification
                lt.insert(n.to_string(), pos);
            }
            for op in label.0 {
                dest.extend(op.extract_bytes());
                pos += 2;
            }
        }
    }
}
