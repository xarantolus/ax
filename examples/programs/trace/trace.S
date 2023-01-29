.intel_syntax noprefix

.text
.global _start
_start:
	call first_level

	mov rax, 60
	mov rdi, 0
	syscall


first_level:
	JMP .Lnext

	# this should never be executed
	mov al, [0]

	.Lnext:
	call second_level

	call second_level_two

	ret

second_level_two:
	mov rbx, 0
	ret


second_level:
	call third_level

	call third_level

	ret

third_level:
	mov rcx, 0
	mov rax, 0
	.Lloop_start:
		add rcx, 1
	.Lloop_cond:
	cmp rcx, 10
	jne .Lloop_start

	ret
