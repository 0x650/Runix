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

#![no_std]
#![no_main]

use runix::arch::x86_64::{entry, asm};

use limine::LimineFramebufferRequest;
static FRAMEBUFFER_REQUEST: LimineFramebufferRequest = LimineFramebufferRequest::new(0);

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    entry::entry();
    asm::halt_forever();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    asm::halt_forever();
}
