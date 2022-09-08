//.equ O_TRUNC,		o1000
//.equ O_CREAT,		o0100
//.equ O_WRONLY,	o0001

.global _bil_interpol

// PARAM rdi: Pointer to file name to generate
// PARAM rsi: Pointer to [u8] buffer representing image
// PARAM edx: Image height
// PARAM ecx: Image width
_bil_interpol:
	push rbp
	mov rbp, rsp

	// Memory layout:
	// ----- rbp -----
	// -4 Image width (uint)
	// -8 Image height (uint)
	// -16 Pointer to [u8] array (char*)
	// -24 Pointer to file name string (char*)
	// -28 New image's height (uint)
	// -32 New image's width (uint)
	// -36 mod (int)
	// -40 inverse mod (int)
	// -48 Pointer to new image's array (char*)
	// -56 File descriptor of image file (int)
	// -64 r8 for rows
	// -72 r9 for columns
	// -80 r10
	// -88 Temporarily, return val

	sub rsp, 80
	mov [rbp - 4], ecx
	mov [rbp - 8], edx
	mov [rbp - 16], rsi
	mov [rbp - 24], rdi

	mov [rbp - 64], r8
	mov [rbp - 72], r9
	mov [rbp - 80], r10

	// add twice = mul by 3
	add edx, edx
	add edx, edx
	sub edx, 2
	mov [rbp - 28], edx

	add ecx, ecx
	add ecx, ecx
	sub ecx, 2
	mov [rbp - 32], ecx

	//// malloc memory for interpolated result
	mov eax, edx
	mul ecx // eax = new_h*new_w
	mov edi, eax
	call malloc
	mov [rbp - 48], rax

	mov r8d, 0
loopOriginalsR:
	cmp r8d, [rbp - 8]
	jge exitLoopOriginals // if r8 >= height, jump 

	mov r9d, 0
	loopOriginalsC:
		cmp r9d, [rbp - 4]
		jge exitLoopOriginalsC

		mov edi, r9d
		mov esi, r8d
		mov eax, [rbp - 4]
		call c2i

		add rax, [rbp - 16]
		mov ecx, [rax]

		// mul * 3
		add edi, edi 
		add edi, edi 
		add esi, esi 
		add esi, esi 
		mov eax, [rbp - 32] // get new_w
		call c2i
		add rax, [rbp - 48]

		mov [rax], ecx

		add r9d, 1
		jmp loopOriginalsC
		
		exitLoopOriginalsC:

	add r8d, 1
	jmp loopOriginalsR

exitLoopOriginals:

	//// open file
	//mov rdi, [rbp - 24]		// load file name ptr
	//mov rsi, 0b1001000001 // O_TRUNC | O_CREAT | O_WRONLY
	//mov rdx, 0b0110100100 // -rw-r--r-- (permissions in case of creation)
	//mov rax, 2						// sys_open
	//syscall
	//mov [rbp - 56], rax

	//// Close file
	//mov rdi, [rbp - 56]		// Get file descriptor
	//mov rax, 3						// sys_close
	//syscall
	//// Ignoring return :/

	mov rdi, [rbp - 48]
	call free

	// Restore callee-saved registers
	mov rdi, [rbp - 24]
	mov rsi, [rbp - 16]
	mov r8 , [rbp - 64]
	mov r9 , [rbp - 72]
	mov r10, [rbp - 80]

	mov rax, 0
	//mov rax, [rbp - 88]
	leave    
	ret

// PARAM edi: row (preserved)
// PARAM esi: col (preserved)
// PARAM eax: width
// RETURN eax: index
c2i:
	mul edi
	add eax, esi
	ret
