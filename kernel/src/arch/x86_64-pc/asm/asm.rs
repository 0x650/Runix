use core::arch::asm;

#[inline]
pub fn halt() {
    unsafe {
        asm!("hlt");
    }
}

#[inline]
pub fn cli() {
    unsafe {
        asm!("cli");
    }
}

#[inline]
pub fn sti() {
    unsafe {
        asm!("sti");
    }
}

#[inline]
pub fn pause() {
    unsafe {
        asm!("pause");
    }
}

#[inline]
pub fn nop() {
    unsafe {
        asm!("nop");
    }
}

#[inline]
pub fn halt_forever() -> ! {
    cli();

    loop {
        halt();
    }
}