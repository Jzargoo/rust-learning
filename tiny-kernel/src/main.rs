#![no_std]
#![no_main]

use core::panic::PanicInfo;

use crate::vga::{color_code::ColorCode, writer::Writer, color::Color};

mod vga;


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    write_smth();
    loop {}
}

#[panic_handler]
pub fn panic(_qi: &PanicInfo) -> ! {
    loop {}
}

fn write_smth() {
    let mut writer = Writer::new(ColorCode::make_color(
        Color::White, 
        Color::Brown)
    );

    writer.write_string(
    "Line 01 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 02 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 03 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 04 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 05 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 06 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 07 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 08 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 09 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 10 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 11 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 12 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 13 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 14 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 15 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 16 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 17 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 18 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 19 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 20 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 21 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 22 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 23 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 24 Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill Fill\
     Line 25 END OF THE BUFFER, FULL SCREEN FILLED PERFECTLY! NO MORE SPACE HERE!!!"
);

}