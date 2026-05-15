#![no_std]
#![no_main]

use core::panic::PanicInfo;

const MEANING: &[u8] = b"hello, world! ";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let buffer = 0xb8000 as *mut u8;

    for (i, &byte) in MEANING.iter().enumerate() {
        unsafe {
            *buffer.offset(i as isize * 2) = byte;
            *buffer.offset(i as isize * 2 + 1) = 0x02
        }
    }

    loop {}
}



#[panic_handler]
pub fn panic(_qi: &PanicInfo) -> ! {
    loop {}
}