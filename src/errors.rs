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

use log::error;
// TODO; consider colorizing output

static mut LOGGING_HINT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn hint_logged() -> () {
    unsafe { LOGGING_HINT.store(true, std::sync::atomic::Ordering::Relaxed) };
}

pub fn lpanic(s: &str) -> ! {
    if unsafe { LOGGING_HINT.load(std::sync::atomic::Ordering::Relaxed) } {
        error!("libxas: error: {}", s);
    } else {
        println!("libxas: nolog: E: {}", s);
    }
    std::process::exit(127i32);
}
