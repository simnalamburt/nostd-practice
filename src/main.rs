#![no_std]
#![no_main]

use core::str::from_utf8_unchecked;
use no_std_io::io::{Cursor, Write};

use isrc::Isrc;
use lazycsv::Csv;
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
    let mut buffer = [0u8; 1024];
    let mut writer = Cursor::new(&mut buffer[..]);

    let csv = Csv::new(b"a,b,c\n1,2,3\n100,200,300\n");
    for row in csv.into_rows() {
        let [r0, r1, r2] = row.unwrap();

        writeln!(
            writer,
            "{}\t{}\t{}",
            unsafe { from_utf8_unchecked(r0.buf) },
            unsafe { from_utf8_unchecked(r1.buf) },
            unsafe { from_utf8_unchecked(r2.buf) }
        )
        .unwrap();
    }
    writeln!(writer).unwrap();

    let isrc = Isrc::from_code("AA6Q72000047").unwrap();
    let code = isrc.to_code_fixed();
    writeln!(writer, "ISRC: {}", unsafe { from_utf8_unchecked(&code) }).unwrap();

    syscall!(
        Sysno::write,
        1,
        writer.get_ref().as_ptr(),
        writer.position()
    )
    .unwrap();
    syscall!(Sysno::exit, 0).unwrap();
    unreachable!();
}
