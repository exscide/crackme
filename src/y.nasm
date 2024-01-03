BITS 64
mov rax,0	; result

; r8 = input ptr

.loop:
	mov al, [r8]  ; store current char
	mov r9b, al

	test r9b, r9b ; check for null
	je .end

	; do magic
	xor al, 0b11100110
	ror al, 4

	rol rax, 8

	add r8, 1     ; increment the pointer

	jmp .loop

.end:
; store pw in r9
mov r9d, 0xf95d496c
xor r9d, -1
rol r9, 32
mov r10d, 0x186af900
xor r10d, -1
add r9, r10
xor r9, -1

mov r10b, 0	; "return" value

cmp rax, r9	
jne .return

mov r10b, 1

.return:
jmp QWORD rcx
