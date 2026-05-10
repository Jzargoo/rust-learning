#[repr(C)]
pub struct Context {
    esp: u32,
    ebx: u32,
    esi: u32,
    edi: u32,
    ebp: u32,
}

impl Context{
    pub fn new(esp: u32) -> Self {
        Self {
            esp: esp,
            ebx: 0,
            esi: 0,
            edi: 0,
            ebp: 0,
        }
    }
}