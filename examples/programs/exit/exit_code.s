.intel_syntax noprefix
.section .text

.global _start

_start:
    mov rax, 60 # Exit system call
    mov rdi, 13 # Error code

    syscall
