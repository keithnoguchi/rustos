//! [Bump] allocator.
//!
//! [bump]: https://os.phil-opp.com/allocator-designs/#bump-allocator
use super::alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;

type LockedAllocator = super::Locked<Allocator>;

unsafe impl GlobalAlloc for LockedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut inner = self.lock();
        let alloc_start = super::align_up(inner.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };
        if alloc_end > inner.heap_end {
            ptr::null_mut()
        } else {
            inner.next = alloc_end;
            inner.allocations += 1;
            alloc_start as *mut u8
        }
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut inner = self.lock();
        inner.allocations -= 1;
        if inner.allocations == 0 {
            inner.next = inner.heap_start;
        }
    }
}

pub(super) struct Allocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl Allocator {
    #[allow(dead_code)]
    pub(super) const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }
    #[allow(dead_code)]
    pub(super) unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}
