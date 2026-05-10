
use crate::balloc::Balloc;

mod balloc;
mod metadata;

#[global_allocator]
static GLOBAL_ALLOCATOR: Balloc = Balloc::new();

fn main(){

}