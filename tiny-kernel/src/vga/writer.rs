use crate::vga::color_code::ColorCode;
use crate::vga::printable_char::PrintableChar;
use crate::vga::vga_buffer;

#[allow(dead_code)]
pub struct Writer {
    column_offset: usize,
    raw_offset: usize,
    buffer: &'static mut vga_buffer::VgaBuffer,
    color_code: ColorCode,
    accessible: bool // True - it has space, false otherwise
}

#[allow(dead_code)]
impl Writer {
    pub fn new(color: ColorCode) -> Self{
        Writer { 
            column_offset: vga_buffer::BUFFER_WIDTH_INDEX,
            buffer: unsafe{ &mut *(0xb8000 as *mut vga_buffer::VgaBuffer) },
            raw_offset: vga_buffer::BUFFER_HEIGHT_INDEX,
            color_code: color,
            accessible: true
        }
    }

    pub fn write_byte(&mut self,char: u8) -> Result<(), &str> {
        if !self.accessible {
            return Err("buffer overflow");
        }

        if char == b'\n' {
            self.new_line();
            return Ok(());
        }

        let row_idx = vga_buffer::BUFFER_HEIGHT_INDEX - self.raw_offset;
        let col_idx = vga_buffer::BUFFER_WIDTH_INDEX - self.column_offset;

        let volatile = &mut self
        .buffer.0[row_idx][col_idx];
        
        volatile.write(
            PrintableChar::new(char, self.color_code)
        );

        if self.column_offset == 0 {
            if self.raw_offset == 0 {
                self.accessible=false;
            } else {
                self.raw_offset -= 1;
                self.column_offset = vga_buffer::BUFFER_WIDTH_INDEX;
            }
        } else {
            self.column_offset -= 1;
        }

        Ok(())

    }

    pub fn new_line(&mut self){
        todo!()
    }

    pub fn write_string(&mut self,query :&str) {
        for &byte in query.as_bytes() {
            let res = match byte {
                0x20..=0x7e| b'\n'=> {self.write_byte(byte)},
                _ => {self.write_byte(0xfe)}
            };
            if res.is_err() {
                break;
            }
        }
    }
}