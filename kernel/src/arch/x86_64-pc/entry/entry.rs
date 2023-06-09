#[path = "../serial/serial.rs"]
mod serial;

#[path = "../asm/asm.rs"]
mod asm;

pub fn entry() {
    serial::init();
    serial::print("Hello x86_64-pc World!\n");

    asm::halt_forever();
}