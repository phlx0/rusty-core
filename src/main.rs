#![no_std]
#![no_main]

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println("Hello from the kernel!");
    serial_println("It is running :)");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println("KERNEL PANIC");
    loop {}
}

fn serial_println(s: &str) {
    let mut port = Port::new(0x3F8);
    for b in s.bytes() {
        unsafe { port.write(b); }
    }
    unsafe { port.write(b'\n'); }
}

