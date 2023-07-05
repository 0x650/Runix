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

macro_rules! define_struct_with_const_defaults {
    ($v:vis struct $name:ident { $($f_name:ident: $f_type:ty = $f_default:expr),* $(,)? }) => {
        #[repr(C, packed)]
        $v struct $name {
            $(
                $f_name: $f_type,
            )*
        }

        impl $name {
            pub const DEFAULT: $name = $name::new();

            pub const fn new() -> $name {
                $name {
                    $(
                        $f_name: $f_default,
                    )*
                }
            }
        }

        impl core::default::Default for $name {
            fn default() -> $name {
                $name::new()
            }
        }
    }
}

define_struct_with_const_defaults! {
    pub struct gdt_desc {
        limit: u16 = 0,
        base_low: u16 = 0,
        base_mid: u8 = 0,
        access: u8 = 0,
        granularity: u8 = 0,
        base_high: u8 = 0,
    }
}

define_struct_with_const_defaults! {
    pub struct tss_desc {
        length: u16 = 0,
        base_low: u16 = 0,
        base_mid: u8 = 0,
        flags1: u8 = 0,
        flags2: u8 = 0,
        base_high: u8 = 0,
        base_upper32: u32 = 0,
        reserved: u32 = 0,
    }
}

define_struct_with_const_defaults! {
    pub struct gdt_ptr {
        limit: u16 = 0,
        ptr: u64 = 0,
    }
}

define_struct_with_const_defaults! {
    pub struct gdtr {
        entries: [gdt_desc; 5] = [gdt_desc::DEFAULT; 5],
        tss: tss_desc = tss_desc::DEFAULT,
    }
}

define_struct_with_const_defaults! {
    pub struct tss {
        reserved: u32 = 0,
        rsp: [u64; 3] = [0; 3],
        reserved2: u64 = 0,
        ist: [u64; 7] = [0; 7],
        reserved3: u64 = 0,
        reserved4: u16 = 0,
        iomap_base: u16 = 0,
    }
}

pub static mut GDT: gdtr = gdtr { ..gdtr::DEFAULT };
pub static mut GDT_POINTER: gdt_ptr = gdt_ptr { ..gdt_ptr::DEFAULT };

extern "C" {
    fn segment_reload(cs: u8, seg: u8);
    fn tss_reload(seg: u8);
}

pub unsafe fn init() {
    // Kernel code
    GDT.entries[1].access = 0b10011010;
    GDT.entries[1].granularity = 0b00100000;

    // Kernel data
    GDT.entries[2].access = 0b10010010;

    // User data
    GDT.entries[3].access = 0b11110010;

    // User code
    GDT.entries[4].access = 0b11111010;
    GDT.entries[4].granularity = 0b00100000;

    // TSS
    GDT.tss.length = (core::mem::size_of::<tss>()) as u16;
    GDT.tss.flags1 = 0b10001001;

    // Set the pointer
    GDT_POINTER.limit = (core::mem::size_of::<gdtr>() - 1) as u16;
    GDT_POINTER.ptr = (&GDT as *const _) as u64;

    asm!("lgdt [{}]", in(reg) &GDT_POINTER, options(readonly, nostack, preserves_flags));
    segment_reload(8, 0x10);
    tss_reload(0x2B);
}
