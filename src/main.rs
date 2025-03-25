#![no_std]
#![no_main]

use syscalls::{Sysno, syscall};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
unsafe fn _start() -> ! {
    syscall!(Sysno::write, 1, "Hello, World!\n".as_ptr(), 14).unwrap();
    syscall!(Sysno::exit, 0).unwrap();
    loop {}
}
