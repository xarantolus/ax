.intel_syntax noprefix

.section .rodata
.Largv: .ascii "--------------------------------------------------\nargv values:\n--------------------------------------------------\n"
.Largv_len: .quad .Largv_len - .Largv

.Lenvp: .ascii "--------------------------------------------------\nenvp values:\n--------------------------------------------------\n"
.Lenvp_len: .quad .Lenvp_len - .Lenvp

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
	call strlen

	mov rdx, rax # number of bytes to write
	mov rax, 1 # write system call
    mov rdi, 1 # Stdout
	# rsi already points to argv[i]
    syscall

	call newline
	dec r13
	JMP .Lstart_argv

	.Lend_argv:

	# after argv values, we have 8 zero bytes on the stack
	pop rsi

	# now we can write "envp values:\n"
	mov rax, 1 # write system call
	mov rdi, 1 # Stdout
	lea rsi, [rip+.Lenvp] # pointer to string, argv[i]
	mov rdx, [rip+.Lenvp_len] # Number of bytes to write
	syscall

	# envp, just like argv, is a null-terminated array of string pointers
	# unlike argv, we don't have a count, so we just iterate until we find null
	.Lstart_envp:
	pop rsi # pointer to string, envp[i]
	cmp rsi, 0
	je .Lend_envp

	call strlen

	mov rdx, rax # number of bytes to write
	mov rax, 1 # write system call
	mov rdi, 1 # Stdout
	# rsi already points to envp[i]
	syscall

	call newline

	JMP .Lstart_envp
	.Lend_envp:

	mov rdi, Vargc
    mov rax, 0x3c
    syscall

	mov [0], rax

newline:
	push rax
	push rdi
	push rsi
	push rdx

	mov rax, 1 # write system call
	mov rdi, 1 # Stdout
	lea rsi, [rip+.Lnewline] # pointer to newline
	mov rdx, 1
	syscall

	pop rdx
	pop rsi
	pop rdi
	pop rax

	ret

# strlen(char *rsi) -> rax
strlen:
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
	ret
