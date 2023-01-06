.intel_syntax noprefix

.section .rodata
.Largv: .asciz "argv values:\n"
.Largv_len: .quad 13

.Lnewline: .asciz "\n"
.Lnewline_len: .quad 1

.section .text
.global _start
_start:
	pop rdi # pop argc from stack

	# save argc in r12
	#define Vargc r12
	mov Vargc, rdi

	// write "argv values:\n"
	mov rax, 1 # write system call
    mov rdi, 1 # Stdout
    lea rsi, [rip+.Largv] # pointer to string, argv[i]
    mov rdx, [rip+.Largv_len] # Number of bytes to write
    syscall

	mov r13, Vargc
	// Now iterate over all argv elements and print them
	.Lstart_argv:
	cmp r13, 0
	je .Lend_argv

    pop rsi # pointer to string, argv[i]
	# strlen:
	xor rax, rax
	push rbx
	.Lstrlen_loop:
		mov bl, [rsi+rax]
		cmp bl, 0
		je .Lstrlen_end
		inc rax
		jmp .Lstrlen_loop
	.Lstrlen_end:
	pop rbx

	mov rdx, rax # number of bytes to write
	mov rax, 1 # write system call
    mov rdi, 1 # Stdout
	# rsi already points to argv[i]
    syscall

	call newline
	dec r13
	JMP .Lstart_argv

	.Lend_argv:
	mov rdi, Vargc
    mov rax, 0x3c
    syscall

	mov [0], rax

newline:
	# push all syscall clobber registers
	push rax
	push rdi
	push rsi
	push rdx

	# write newline
	mov rax, 1 # write system call
	mov rdi, 1 # Stdout
	lea rsi, [rip+.Lnewline] # pointer to newline
	mov rdx, 1
	syscall

	# restore all syscall clobber registers
	pop rdx
	pop rsi
	pop rdi
	pop rax

	ret
