//! Writing an [OS] in Rust
//!
//! [os]: https://os.phil-opp.com
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate bootloader;
extern crate rustos;
extern crate x86_64;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rustos::println;
use x86_64::{
    structures::paging::{MapperAllSizes, Page},
    VirtAddr,
};

entry_point!(start_kernel);

fn start_kernel(boot_info: &'static BootInfo) -> ! {
    println!("Welcome to the real world!");

    rustos::init();

    // frame allocator.
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { rustos::memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { rustos::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // dump the virtual to physical mapping.
    let addresses = [
        // the identity-mapped vga buffer page.
        0xb8000,
        // some code page.
        0x0020_1008,
        // some stack page.
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0.
        boot_info.physical_memory_offset,
    ];
    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    // write the string "New!" to the screen!
    let page = Page::containing_address(VirtAddr::new(0));
    create_example_mapping(page, &mut mapper, &mut frame_allocator);
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();
    println!("It did not crash!!!");
    rustos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}

use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, PhysFrame, Size4KiB, UnusedPhysFrame,
    },
    PhysAddr,
};

fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let unused_frame = unsafe { UnusedPhysFrame::new(frame) };
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = mapper.map_to(page, unused_frame, flags, frame_allocator);
    map_to_result.expect("map_to failed").flush();
}
