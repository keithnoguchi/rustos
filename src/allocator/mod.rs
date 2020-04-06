//! Heap allocators
extern crate alloc;
extern crate linked_list_allocator;

use alloc::alloc::Layout;
#[allow(unused_imports)]
use linked_list_allocator::LockedHeap;
use spin::{Mutex, MutexGuard};
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

/// Kernel heap start address.
pub const HEAP_START: usize = 0x_4444_4444_0000;
/// Kernel heap size.
pub const HEAP_SIZE: usize = 100 * 1024; // 100KiB

struct Locked<A> {
    inner: Mutex<A>,
}

impl<A> Locked<A> {
    #[allow(dead_code)]
    const fn new(inner: A) -> Self {
        Self {
            inner: Mutex::new(inner),
        }
    }
    fn lock(&self) -> MutexGuard<A> {
        self.inner.lock()
    }
}

/// Different allocator designs.
mod block;
mod bump;
mod list;

#[global_allocator]
static ALLOCATOR: Locked<block::Allocator> = Locked::new(block::Allocator::new());

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}

pub(crate) fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        mapper.map_to(page, frame, flags, frame_allocator)?.flush();
    }
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
    Ok(())
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
