global segment_reload
global tss_reload

segment_reload:
    push rdi
    lea rax, [rel .flush]
    push rax
    retfq
.flush:
    mov eax, esi
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    ret

tss_reload:
    mov ax, di
    ltr ax
    ret
