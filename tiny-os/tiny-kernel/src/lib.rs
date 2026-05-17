#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::vga::{color_code::ColorCode, writer::Writer, color::Color};


mod vga;
mod boot;

#[panic_handler]
pub fn panic(_qi: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
fn kernel_main() -> ! {
    write_smth();
    loop {}
}


fn write_smth() {
    let mut writer = Writer::new(
    ColorCode::make_color(
                    Color::Brown,
        Color::LightGreen
        )
    );
    writer.write_string("fds");
}