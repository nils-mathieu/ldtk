BITS 64;

	; Check whether the first argument is 1 or 2 (standard output or standard error).
	cmp rdi, 1
	je do_nothing
	cmp rdi, 2
	je do_nothing

	; Otherwise, perform the `write` system call manually (we can't rely on the previous
	; implementation of `write` because we're overriding it).
	mov rax, 1
	syscall

	; If `rax` is above that number, an error occured and we need to write the errno.
	cmp rax, 0xfffffffffffff000
	ja failed

	; Everything went well!
	; The return value of the system call is already in `rax`, we can just return.
	ret

do_nothing:
	; Simply returm the thrid argument, indicating that everything went well (even though we did
	; nothing).
	mov rax, rdx
	ret

failed:
	; We won't even set the last error code (errno).
	mov rax, -1
	ret
