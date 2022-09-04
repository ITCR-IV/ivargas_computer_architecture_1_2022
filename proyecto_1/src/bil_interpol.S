.equ O_TRUNC,		o1000
.equ O_CREAT,		o0100
.equ O_WRONLY,	o0001

.global _bil_interpol

// PARAM rdi: Pointer to file name to generate
// PARAM rsi: Pointer to [u8] buffer representing image
// PARAM edx: Image height
// PARAM ecx: Image width

_bil_interpol:
	push rbp
	mov rbp, rsp
	sub rsp, 28
	mov [rbp - 28], rdi
	mov [rbp - 20], rsi
	mov [rbp - 12], edx
	mov [rbp -  8], ecx

	add edx, ecx
	mov [rbp - 4], edx

	mov al, 104 // h
	call printChar
	mov al, 43 // +
	call printChar
	mov al, 119 // w
	call printChar
	mov al, 58 // :
	call printChar
	mov al, 32 // espacio
	call printChar
	mov al, [rbp - 4]
	add al, 48
	call printChar

	mov al, 10 // \n
	call printChar

	//mov rax, [rbp - 4]

	leave
	ret

// PARAM al: char to be printed
printChar:
	push rbp
	mov rbp, rsp

	sub rsp, 1
	mov [rsp], al

	mov rdi, 1        // stdout
	mov rsi, rsp			// point to char on stack
	mov rdx, 1				// length
	mov rax, 1
	syscall

	mov rsp, rbp
	pop rbp
	//leave
	ret
