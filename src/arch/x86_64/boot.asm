MBALIGN  equ  1 << 0            ; align loaded modules on page boundaries
MEMINFO  equ  1 << 1            ; provide memory map
FLAGS    equ  MBALIGN | MEMINFO ; this is the Multiboot 'flag' field
MAGIC    equ  0x1BADB002        ; 'magic number' lets bootloader find the header
CHECKSUM equ -(MAGIC + FLAGS)   ; checksum of above, to prove we are multiboot

section .header
multiboot:
    dd MAGIC
    dd FLAGS
    dd CHECKSUM
multiboot_end:

global start
section .text
bits 32
start:
    mov dword [0xb8000], 0x2f4b2f4f ; OK with light-gray fg on green bg
finish:
    cli
    hlt
    jmp finish