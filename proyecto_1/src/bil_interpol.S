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
	// -88 r11
	// -96 r12
	// -104 r13
	// -112 r14
	// -120 r15
	// -128 Temporarily, return val

	sub rsp, 128
	mov [rbp - 4], ecx
	mov [rbp - 8], edx
	mov [rbp - 16], rsi
	mov [rbp - 24], rdi

	mov [rbp - 64], r8
	mov [rbp - 72], r9
	mov [rbp - 80], r10
	mov [rbp - 88], r11
	mov [rbp - 96], r12
	mov [rbp - 104], r13
	mov [rbp - 112], r14
	mov [rbp - 120], r15

	mov r15, 0
	mov [rbp - 128], r15

	// add twice = mul by 3
	mov r15d, edx
	add edx, r15d
	add edx, r15d
	sub edx, 2
	mov [rbp - 28], edx

	mov r15d, ecx
	add ecx, r15d
	add ecx, r15d
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
		mov cl, [rax]

		// mul * 3
		mov r15d, edi
		add edi, r15d
		add edi, r15d
		mov r15d, esi
		add esi, r15d
		add esi, r15d
		mov eax, [rbp - 32] // get new_w
		call c2i
		add rax, [rbp - 48]

		mov [rax], cl

		add r9d, 1
		jmp loopOriginalsC
		
		exitLoopOriginalsC:

	add r8d, 1
	jmp loopOriginalsR

exitLoopOriginals:


	mov r8d, 0
loopVerticalR:
	cmp r8d, [rbp - 28]
	jge exitLoopVertical // if r8 >= height, jump 

	mov r9d, 0
	loopVerticalC:
		cmp r9d, [rbp - 4]
		jge exitLoopVerticalC


		mov edx, 0
		mov eax, r8d
		mov r15, 3
		div r15

		mov [rbp - 36], edx

		// if mod == 0: continue
		test edx, edx
		jz continueVerticalC

		neg edx
		add edx, 3
		mov [rbp - 40], edx

		// r10d = new_r//3
		mov r10d, eax

		// Get up_ref into r11
		mov edi, eax
		mov esi, r9d
		mov eax, [rbp - 4]
		call c2i

		add rax, [rbp - 16]
		mov r11b, [rax]

		// Get down_ref into r12
		add r10d, 1
		mov edi, r10d
		mov esi, r9d
		mov eax, [rbp - 4]
		call c2i

		add rax, [rbp - 16]
		mov r12b, [rax]

		// Get new_i into r13d
		mov edi, r8d
		mov esi, r9d
		add esi, r9d
		add esi, r9d
		mov eax, [rbp - 32]
		call c2i

		mov r13d, eax

		// Calculate new_image[new_i]

		mov eax, [rbp - 40]
		mul r11d
		mov r14d, eax

		mov eax, [rbp - 36]
		mul r12d

		add eax, r14d
		mov r15, 3
		div r15

		add r13, [rbp - 48]
		mov [r13], al
		continueVerticalC:

		add r9d, 1
		jmp loopVerticalC
		
		exitLoopVerticalC:

	add r8d, 1
	jmp loopVerticalR

exitLoopVertical:

	mov r8d, 0
loopHorizontalR:
	cmp r8d, [rbp - 28]
	jge exitLoopHorizontal // if r8 >= height, jump 

	mov r9d, 0
	loopHorizontalC:
		cmp r9d, [rbp - 32]
		jge exitLoopHorizontalC


		// mod
		mov edx, 0
		mov eax, r9d
		mov r15, 3
		div r15

		mov [rbp - 36], edx

		// if mod == 0: continue
		test edx, edx
		jz continueHorizontalC

		// inv_mod
		neg edx
		add edx, 3
		mov [rbp - 40], edx

		// r10d = new_c - mod
		mov r10d, r9d
		sub r10d, [rbp - 36]

		// Get left_ref into r11
		mov edi, r8d
		mov esi, r10d
		mov eax, [rbp - 32]
		call c2i

		add rax, [rbp - 48]
		mov r11b, [rax]

		// Get right_ref into r12
		add r10d, 3 // r10d = new_c + mod
		mov edi, r8d
		mov esi, r10d
		mov eax, [rbp - 32]
		call c2i

		add rax, [rbp - 48]
		mov r12b, [rax]

		// Get new_i into r13d
		mov edi, r8d
		mov esi, r9d
		mov eax, [rbp - 32]
		call c2i

		mov r13d, eax

		// Calculate new_image[new_i]

		mov eax, [rbp - 40]
		mul r11d
		mov r14d, eax

		mov eax, [rbp - 36]
		mul r12d

		add eax, r14d
		mov r15, 3
		div r15

		add r13, [rbp - 48]
		mov [r13], al
		continueHorizontalC:

		add r9d, 1
		jmp loopHorizontalC
		
		exitLoopHorizontalC:

	add r8d, 1
	jmp loopHorizontalR

exitLoopHorizontal:

	// open file
	mov rdi, [rbp - 24]		// load file name ptr
	mov rsi, 0b1001000001 // O_TRUNC | O_CREAT | O_WRONLY
	mov rdx, 0b0110100100 // -rw-r--r-- (permissions in case of creation)
	mov rax, 2						// sys_open
	syscall
	mov [rbp - 56], rax

	// write to file
	mov eax, [rbp - 28]
	mov edx, [rbp - 32]
	mul edx // eax = new_h*new_w
	mov eax, eax
	mov rdx, rax // Size of buffer

	mov rdi, [rbp - 56]		// Get file descriptor
	mov rsi, [rbp - 48]		// Pointer to buffer
	mov rax, 1						// sys_write
	syscall

	// Close file
	mov rdi, [rbp - 56]		// Get file descriptor
	mov rax, 3						// sys_close
	syscall
	// Ignoring return :/

	mov rdi, [rbp - 48]
	call free

	// Restore callee-saved registers
	mov rdi, [rbp - 24]
	mov rsi, [rbp - 16]

	mov r8 , [rbp - 64]
	mov r9 , [rbp - 72]
	mov r10, [rbp - 80]
	mov r11, [rbp - 88]
	mov r12, [rbp - 96]
	mov r13, [rbp - 104]
	mov r14, [rbp - 112]
	mov r15, [rbp - 120]

	mov rax, [rbp - 128]
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
