global longmode_start
extern rmain
section .text
bits 64
longmode_start:
    xor ax, ax
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov rax, 0x2f592f412f4b2f4f ; "OKAY"
    mov qword [0xb8000], rax
    call clear
    call rmain
    mov al, "?"
    mov byte [0xb8004], al
clear:
    push rdi
    push rax
    push rcx
    mov edi, 0xb8000
    mov rax, 0x1F201F201F201F20
    mov ecx, 500
    rep stosq
    pop rcx
    pop rax
    pop rdi
    ret
hang:
    cli
    hlt
    jmp hang