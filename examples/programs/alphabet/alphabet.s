.intel_syntax noprefix

.global _start

.section .text
_start:
    mov al, 'a'
    mov r11, 0

    .Lloop_start:
        mov byte ptr [alphabet+r11], al
        add al, 1
        add r11, 1

        cmp al, 'z'
    JLE .Lloop_start

    mov rax, 1 # write system call
    mov rdi, 1 # Stdout
    lea rsi, [alphabet] # Pointer to data
    mov rdx, 27 # Number of bytes to write
    syscall

    mov rax, 60 # Exit system call
    mov rdi, 0 # Error code

    syscall


.section .data
alphabet:
    .space 26
    .asciz "\n"
