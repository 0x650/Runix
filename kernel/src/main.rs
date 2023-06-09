#![no_std]
#![no_main]

use core::arch::asm;

#[path = "./arch/x86_64-pc/entry/entry.rs"]
mod entry;

#[path = "./arch/x86_64-pc/asm/asm.rs"]
mod asm;

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
