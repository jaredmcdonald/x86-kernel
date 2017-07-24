global long_mode_start

section .text
bits 64
long_mode_start:
    ; load 0 into all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    extern rust_main
    call rust_main

    ; print a cool message
    mov rax, 0x2f722f612f6c2f62
    mov qword [0xb8000], rax
    mov rax, 0x2f212f212f682f67
    mov qword [0xb8008], rax
    hlt
