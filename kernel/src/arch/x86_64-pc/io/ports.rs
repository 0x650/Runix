/*
 * Copyright 2023 Runix Project Contributors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use core::arch::asm;

#[inline]
pub fn outb(port: u16, val: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") val,
            options(preserves_flags, nomem, nostack)
        );
    }
}

#[inline]
pub fn outw(port: u16, val: u16) {
    unsafe {
        asm!(
            "out dx, ax",
            in("dx") port,
            in("ax") val,
            options(preserves_flags, nomem, nostack)
        );
    }
}

#[inline]
pub fn outd(port: u16, val: u32) {
    unsafe {
        asm!(
            "out dx, eax",
            in("dx") port,
            in("eax") val,
            options(preserves_flags, nomem, nostack)
        );
    }
}

#[inline]
pub fn inb(port: u16) -> u8 {
    let ret: u8;

    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") ret,
            options(preserves_flags, nomem, nostack)
        );
    }

    return ret;
}

#[inline]
pub fn inw(port: u16) -> u16 {
    let ret: u16;

    unsafe {
        asm!(
            "in ax, dx",
            in("dx") port,
            out("ax") ret,
            options(preserves_flags, nomem, nostack)
        );
    }

    return ret;
}

#[inline]
pub fn ind(port: u16) -> u32 {
    let ret: u32;

    unsafe {
        asm!(
        "in eax, dx",
            in("dx") port,
            out("eax") ret,
            options(preserves_flags, nomem, nostack)
        );
    }

    return ret;
}
