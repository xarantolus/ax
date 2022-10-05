.intel_syntax noprefix

.global _start

.section .text
_start:
    mov rax, 1 # write system call
    mov rdi, 1 # Stdout
    lea rsi, [rip+hws] # Pointer to data
    mov rdx, 14 # Number of bytes to write
    syscall

    mov rax, 60 # Exit system call
    mov rdi, 0 # Error code
    syscall


.section .data
hws:
    .asciz "Hello, World!\n"
