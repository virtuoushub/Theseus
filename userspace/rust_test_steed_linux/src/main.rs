#![feature(asm)]


pub fn main() {

	// loop {
	// 	for _ in 0..100000 {
	// 		// just a busy wait for a bit
	// 	}
		unsafe {
			// rax -- syscall number
			// rdi -- first argument
			// rsi -- second argument
			// rdx -- third argument
			// r10 -- fourth argument
			// r9  -- fifth argument 
			// r8  -- sixth argument
			
			// asm!("  mov rax, 1 ; \
			// 		mov rdi, 1234  ; \
			// 		mov rsi, 2345  ; \
			// 		mov rdx, 3456  ; \
			// 		mov r10, 4567  ; \
			// 		mov r9 , 5678  ; \
			// 		mov r8 , 6789  ; \
			// 		syscall" : : : : "intel"
			// );

			// linux test
			// asm!("  mov rax, 60 ; \
			// 		mov edi, 22 ; \
			// 		syscall" : : : : "intel"
			// );
		}
	// }

	println!("hello: test_me_func(5)={}", test_me_func(5));
}

pub fn test_me_func(a: isize) -> isize {
	a * 2
}
