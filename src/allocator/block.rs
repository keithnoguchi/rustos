//! Fixed-size [Block] Allocator
//!
//! [block]: https://os.phil-opp.com/allocator-designs/#fixed-size-block-allocator
extern crate linked_list_allocator;
use super::alloc::alloc::{GlobalAlloc, Layout};
use core::{
    mem,
    ptr::{self, NonNull},
};

const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

type LockedAllocator = super::Locked<Allocator>;

unsafe impl GlobalAlloc for LockedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut inner = self.lock();
        match Allocator::list_index(&layout) {
            Some(index) => match inner.list_heads[index].take() {
                Some(node) => {
                    inner.list_heads[index] = node.next.take();
                    node as *mut Node as *mut u8
                }
                None => {
                    let block_size = BLOCK_SIZES[index];
                    let block_align = block_size;
                    let layout = Layout::from_size_align(block_size, block_align).unwrap();
                    inner.fallback_alloc(layout)
                }
            },
            None => inner.fallback_alloc(layout),
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut inner = self.lock();
        match Allocator::list_index(&layout) {
            Some(index) => {
                let new_node = Node {
                    next: inner.list_heads[index].take(),
                };
                // make sure the block is bigger than node to hold.
                assert!(mem::size_of::<Node>() <= BLOCK_SIZES[index]);
                assert!(mem::align_of::<Node>() <= BLOCK_SIZES[index]);
                #[allow(clippy::cast_ptr_alignment)]
                let new_node_ptr = ptr as *mut Node;
                new_node_ptr.write(new_node);
                inner.list_heads[index] = Some(&mut *new_node_ptr);
            }
            None => {
                let ptr = NonNull::new(ptr).unwrap();
                inner.fallback_allocator.deallocate(ptr, layout);
            }
        }
    }
}

pub(super) struct Allocator {
    list_heads: [Option<&'static mut Node>; BLOCK_SIZES.len()],
    fallback_allocator: linked_list_allocator::Heap,
}

impl Allocator {
    pub(super) const fn new() -> Self {
        Self {
            list_heads: [None; BLOCK_SIZES.len()],
            fallback_allocator: linked_list_allocator::Heap::empty(),
        }
    }
    pub(super) unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.fallback_allocator.init(heap_start, heap_size);
    }
    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }
    fn list_index(layout: &Layout) -> Option<usize> {
        let required_block_size = layout.size().max(layout.align());
        BLOCK_SIZES.iter().position(|&s| s >= required_block_size)
    }
}

struct Node {
    next: Option<&'static mut Node>,
}
