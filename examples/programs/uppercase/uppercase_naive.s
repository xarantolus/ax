.intel_syntax noprefix
.section .text

.global _start

_start:
    .Lread_start:
    mov rax, 0 # Read
    mov rdi, 0 # From Stdin
    lea rsi, [rip+char] # To char
    mov rdx, 1 # one byte
    syscall

    # Error handling: reading <= 0 bytes is an error
    cmp rax, 0
    JLE .Lloop_end

    mov al, [rip+char]
    cmp al, 'a'
    jl .Ljust_write
    cmp al, 'z'
    jg .Ljust_write

    # Set the "32 bit" to zero, this turns 'a' into 'A'
    and al, 0b11011111
    mov byte ptr [rip + char], al

    .Ljust_write:
    mov rax, 1 # Write
    mov rdi, 1 # To stdout
    lea rsi, [rip+char] # from char
    mov rdx, 1 # one byte
    syscall

    JMP .Lread_start

    .Lloop_end:


    mov rax, 60 # Exit system call
    mov rdi, 0 # Error code

    syscall


.section .data
char:
    .space 1
