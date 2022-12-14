.intel_syntax noprefix
.section .text

.global _start

_start:
    .Lread_start:
    mov rax, 0 # Read
    mov rdi, 0 # From Stdin
    lea rsi, [rip+buffer] # To buffer
    mov rdx, [rip+buflen] # up to buflen bytes
    syscall

    # This syscall will tell us how many chars were written (-> RAX)

    # we use that as exit code, but subtract -1 because the \n is also counted
    mov rdi, rax
    dec rdi

    mov rax, 60 # Exit system call

    syscall


.section .data
buffer:
    .space 1024
buflen: # make sure we always have a null byte left
    .8byte 1023
