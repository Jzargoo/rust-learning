use std::{alloc::GlobalAlloc, ptr::null_mut, sync::atomic::{AtomicPtr, AtomicUsize, Ordering}};
use std::ffi::c_void;

use crate::metadata::{footer_t, header_t};


const HEAP_SIZE:usize = 16 * 1024; //16KB


unsafe extern "C" {
    fn sbrk(increment: isize) -> *mut c_void;
}

struct Node {
    size: usize,
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
        
        // INIT STAGE
        if self.heap_size.load(Ordering::Relaxed) == 0 {
            if let Some(balloc) = Balloc::init() {
                self.apply(balloc);   
            } else {
                return null_mut();
            }
        };
        
        let mut curr_node = self.node.load(Ordering::SeqCst);

        while !curr_node.is_null() {
            let align = layout.align_to(16);
            
            if align.is_err() { break; } 

            let aligned = align.unwrap();

            if (*curr_node).size >= (aligned.size() + size_of::<header_t>() + size_of::<footer_t>()) {
                cur
            } 

            curr_node = (*curr_node).node;
        }

        return null_mut();
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

    pub fn init() -> Option<Self> {
        let void_ptr = unsafe {
           sbrk(HEAP_SIZE as isize) 
        };

        if void_ptr as isize == -1 {
            Option::None
        } else {
            
            let raw_ptr: *mut Node = void_ptr as *mut Node;
            
            let first_node = Node {
                size: (HEAP_SIZE - size_of::<Node>()) as usize,
                address: unsafe {raw_ptr.add(1) as *mut u8},
                node: null_mut()
            };

            unsafe {
                raw_ptr.write(
                    first_node
                ); 
            }

            let balloc = Self { 
                heap_start: unsafe { AtomicPtr::new(raw_ptr.read().address) }, 
                heap_size: AtomicUsize::new(HEAP_SIZE), 
                node: unsafe { AtomicPtr::new(&mut raw_ptr.read()) },
                overall_allocated_bytes: AtomicUsize::new(0)
             };

            Option::Some(balloc)
        }
        
    }

    pub fn apply(&self, balloc: Balloc) {
        
        self.heap_size
            .store(
                balloc.heap_size.load(Ordering::SeqCst),
                Ordering::SeqCst
            ); 
        
        self.heap_start
            .store(
                balloc.heap_start.load(Ordering::SeqCst),
                Ordering::SeqCst
            ); 
        
        self.overall_allocated_bytes
            .store(
                0,
                Ordering::SeqCst
            );
        self.node
            .store(
                balloc.node.load(Ordering::SeqCst),
                Ordering::SeqCst
            );

    }
}