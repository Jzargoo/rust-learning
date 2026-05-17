pub const BUFFER_HEIGHT_INDEX: usize = 24;
pub const BUFFER_WIDTH_INDEX: usize = 79;


use volatile::Volatile;

use crate::vga::printable_char::PrintableChar;

#[allow(dead_code)]
#[repr(transparent)]
pub struct VgaBuffer(pub [[Volatile<PrintableChar>; BUFFER_WIDTH_INDEX + 1]; BUFFER_HEIGHT_INDEX + 1]);