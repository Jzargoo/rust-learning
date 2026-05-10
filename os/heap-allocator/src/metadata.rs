#[repr(C)]
pub struct header_t{
    size: u64, // 8
    busy: bool, //1 
    magic_number:i32 //4 
    // 3
}

#[repr(C)]
pub struct footer_t{
    size: u64, //8
    magic_number:i32 //4
    //4
}
