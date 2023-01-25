BITS 64;
	; Check whether the first argument is 1.
	cmp rdi, 1
	jnz normal_write

	; If it is, then simply returm the thrid argument, indicating that everything went well (even
	; though we did nothing).
	mov rax, rdx
	ret

normal_write:
	; Otherwise, perform the `write` system call manually (we can't rely on the previous
	; implementation of `write` because we're overriding it).
	mov rax, 1
	syscall

	; If `rax` is negative, an error occured and we need to write the errno.
	cmp rax, 0xfffffffffffff000
	ja failed

	; Everything went well!
	; The return value of the system call is already in `rax`, we can just return.
	ret

failed:
	; We won't even set the last error code (errno).
	mov rax, -1
	ret
