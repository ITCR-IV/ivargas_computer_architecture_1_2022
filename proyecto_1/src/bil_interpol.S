.equ O_TRUNC,		o1000
.equ O_CREAT,		o0100
.equ O_WRONLY,	o0001

.global _bil_interpol

// PARAM rdi: Pointer to file name to generate
// PARAM rsi: Pointer to [u8] buffer representing image
// PARAM rdx: Image height
// PARAM rcx: Image width

_bil_interpol:
	push rbp
	mov rbp, rsp
	sub rsp, 20
	mov [rbp - 20], rdi
	mov [rbp - 16], rsi
	mov [rbp - 12], rdx
	mov [rbp -  8], rcx

	add rdx, rcx
	mov [rbp - 4], rdx

	//mov rdi, 104 // h
	//call printChar
	//mov rdi, 43 // +
	//call printChar
	//mov rdi, 119 // w
	//call printChar
	//mov rdi, 58 // :
	//call printChar
	//mov rdi, 32 // espacio
	//call printChar
	//mov rdi, [rbp - 4]
	//add rdi, 48
	//call printChar

	//mov rdi, 10 // \n
	//call printChar

	mov rax, [rbp - 4]

	leave
	ret

// PARAM rdi: char to be printed in lower 8 bytes
printChar:
	push rbp
	mov rbp, rsp

	sub rsp, 1
	mov [rsp], rdi

	mov rdi, 1        // stdout
	mov rsi, rsp			// point to char on stack
	mov rdx, 1				// length
	mov rax, 1
	syscall

	leave
	ret
