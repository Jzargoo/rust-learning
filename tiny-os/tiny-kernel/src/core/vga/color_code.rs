use crate::vga::color::Color;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub(crate)struct ColorCode(u8);


#[allow(dead_code)]
impl ColorCode {
    pub fn make_color (foreground: Color, background: Color) -> Self{
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}