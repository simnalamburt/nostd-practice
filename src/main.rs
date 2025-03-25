#![no_std]
#![no_main]

use syscalls::{Sysno, syscall};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let _unused = unsafe { syscall!(Sysno::exit, 1) };

    #[allow(
        clippy::empty_loop,
        reason = "This loop is to handle the case where the `exit` syscall fails."
    )]
    loop {}
}

#[unsafe(no_mangle)]
unsafe fn _start() -> ! {
    syscall!(Sysno::write, 1, "Hello, World!\n".as_ptr(), 14).unwrap();
    syscall!(Sysno::exit, 0).unwrap();
    unreachable!();
}
