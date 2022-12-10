.intel_syntax noprefix
.global main

# R10 is an address of a buffer for storing a single byte (char)
# Use r11 as input_string address, r12 as input_progress, r13 as input string length
# Then r14 as output string, r15 as output progress

.section .text
main:
	xor rbx, rbx
	xor rcx, rcx

	.LConfusedStonks_0:
	.LUpgradeMarker_0:
	test rsp, 0xF
	jz 1f
	sub rsp, 8
	call readchar
	add rsp, 8
	jmp 2f
	1: call readchar
	2:
	mov bl, BYTE PTR [r10]

	cmp bl, '\n'
	je .LSamePicture_0
	push rbx
	add rcx, 1
	jmp .LUpgradeMarker_0

	.LSamePicture_0:
	.LBananaMarker_0:
	cmp rcx, 0
	jg .LrcxWins_0
	jl .L0Wins_0
	.L0Wins_0:
	jmp .Luaaaaaua
	.LrcxWins_0:
	pop rbx

	mov BYTE PTR [R10], bl
	test rsp, 0xF
	jz 1f
	sub rsp, 8
	call writechar
	add rsp, 8
	jmp 2f
	1: call writechar
	2:

	sub rcx, 1
	jmp .LBananaMarker_0
	.Luaaaaaua:
	mov BYTE PTR [R10], '\n'
	test rsp, 0xF
	jz 1f
	sub rsp, 8
	call writechar
	add rsp, 8
	jmp 2f
	1: call writechar
	2:

	xor rax, rax


	# Don't include this in the emulator test itself
    # mov rax, 1 # write system call
    # mov rdi, 1 # Stdout
    # lea rsi, [rip+.Loutput_string] # Pointer to data
    # mov rdx, [rip+.Lstr_len] # Number of bytes to write
    # syscall

	ret

writechar:
	# Write character to .Loutput_string
	push rax
	push rbx

	lea rbx, [r14]
	add rbx, r15

	mov al, [R10]
	mov [rbx], al

	inc r15

	pop rbx
	pop rax
	ret


readchar:
	# Read next character from input_string
	push rbx

	lea rbx, [r11]
	add rbx, r12

	mov bl, [rbx]
	mov [R10], bl

	inc r12

	pop rbx
	ret
