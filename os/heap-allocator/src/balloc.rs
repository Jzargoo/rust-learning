use std::{alloc::GlobalAlloc, cell::UnsafeCell, ptr::{null, null_mut}, sync::atomic::{AtomicPtr, AtomicUsize}};
use std::ffi::c_void;

const HEAP_SIZE:usize = 16 * 1024; //16KB


unsafe extern "C" {
    fn sbrk(increment: isize) -> *mut c_void;
}

struct Node {
    size: u32,
    address: *mut u8,
    node: *mut Node
}

#[repr(align(16))]
pub struct Balloc{
    pub heap_start: AtomicPtr<u8>,
    pub heap_size: AtomicUsize,
    pub node: AtomicPtr<Node>, 
    pub overall_allocated_bytes: AtomicUsize
}

unsafe impl Sync for Balloc {}


unsafe impl GlobalAlloc for Balloc {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        todo!()
    }
}

impl Balloc {
    pub const fn new() -> Self{
        Balloc { 
            heap_start: AtomicPtr::new(null_mut()),
            heap_size: AtomicUsize::new(67),
            node: AtomicPtr::new(null_mut()),
            overall_allocated_bytes:AtomicUsize::new(0)
         }
    }
    pub fn init() -> Self {
        let void_ptr = unsafe {
           sbrk(HEAP_SIZE as isize) 
        };

        if void_ptr as isize == -1 {
            panic!("AAAAAAAAAAAAAAAA!")
        } else {
            
            let raw_ptr: *mut Node = void_ptr as *mut Node;
            
            let first_node = Node {
                size: (HEAP_SIZE - size_of::<Node>()) as u32,
                address: unsafe {raw_ptr.add(1) as *mut u8},
                node: null_mut()
            };

            unsafe {
                raw_ptr.write(
                    first_node
                ); 
            }

            Self { 
                heap_start: unsafe { AtomicPtr::new(raw_ptr.read().address) }, 
                heap_size: AtomicUsize::new(HEAP_SIZE), 
                node: AtomicPtr::new(raw_ptr), overall_allocated_bytes: AtomicUsize::new(0)
             }
        }
        
    }
}