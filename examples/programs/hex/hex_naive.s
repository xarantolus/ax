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
    mov bl, al

    # Only the 4 lowest bits of al and bl are significant:
    and al, 0b1111
    shr bl, 4

    # Now we decide whether we use '0' as starting point or 'a' for al
    cmp al, 10
    JGE .Lal_g10
    add al, '0'
    JMP .Lal_end
    .Lal_g10:
    add al, ('a'-10) # Since we have an offset of 10 + offset from 'a', we need to subtract 10
    .Lal_end:

    # Same for bl
    cmp bl, 10
    JGE .Lbl_g10
    add bl, '0'
    JMP .Lbl_end
    .Lbl_g10:
    add al, ('a'-10)
    .Lbl_end:

    # Switch the bytes around due to endianness
    mov [output+1], al
    mov [output], bl

    .Ljust_write:
    mov rax, 1 # Write
    mov rdi, 1 # To stdout
    lea rsi, [rip+output] # from char
    mov rdx, 2 # two bytes
    syscall

    JMP .Lread_start

    .Lloop_end:


    mov rax, 1 # Write
    mov rdi, 1 # To stdout
    lea rsi, [rip+newline]
    mov rdx, 1 # one byte
    syscall

    mov rax, 60 # Exit system call
    mov rdi, 0 # Error code

    syscall


.section .data
output:
.space 2

char: .space 1

newline: .ascii "\n"
