use crate::vga::color_code::ColorCode;

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) struct PrintableChar{
    char: u8,
    color: ColorCode    
}

impl PrintableChar {
    pub fn new(char: u8, color: ColorCode) -> Self{
        Self { char, color }
    } 
}