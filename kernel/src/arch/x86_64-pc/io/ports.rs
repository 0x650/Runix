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