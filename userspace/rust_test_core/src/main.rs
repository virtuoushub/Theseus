#![feature(asm)]
#![feature(lang_items)]
#![feature(start)]
#![feature(rustc_private)]
#![feature(compiler_builtins_lib)]
#![feature(unwind_attributes)]



#![no_std]


extern crate compiler_builtins;
extern crate libc;

// http://mainisusuallyafunction.blogspot.com/2015/01/151-byte-static-linux-binary-in-rust.html


#[start]
pub fn main(_argc: isize, _argv: *const *const u8 ) -> isize { 

	
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
			asm!("  mov rax, 60 ; \
					mov edi, 22 ; \
					syscall" : : : : "intel"
			);
		}
	// }

	test_me_func(5)
}

#[inline(never)]
pub fn test_me_func(a: isize) -> isize {
	a * 2
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
// #[unwind]
#[no_mangle]
pub extern "C" fn panic_fmt(_fmt: ::core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    loop {}
}


#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}