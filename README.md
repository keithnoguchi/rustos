# rustos

[![drone]](https://cloud.drone.io/keithnoguchi/rustos)
[![crate]](https://lib.rs/rustos)
[![docs]](https://docs.rs/rustos)

Philipp Oppermann's *awesome* [Writing an OS in Rust]

- Bare bones
  - [A Freestanding Rust Binary] : [post01.rs]
  - [A Minimal Rust Kernel] : [post02.rs]
  - [VGA Text Mode] : [post03.rs]
  - [Testing] : [post04.rs]
    - [tests/basic_boot.rs]
    - [tests/should_panic.rs]
- Interrupts
  - [CPU Exceptions] : [post05.rs]
  - [Double Faults] : [post06.rs]
    - [tests/page_fault.rs]
    - [tests/stack_overflow.rs]
  - [Hardware Interrupts] : [post07.rs]
- Memroy Management
  - [Introduction to Paging] : [post08.rs]
  - [Paging Implementation] : [post09.rs]

## Examples

Current [main.rs]:

```rust
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
```

Happy Hackin'!

[drone]: https://cloud.drone.io/api/badges/keithnoguchi/rustos/status.svg
[crate]: https://img.shields.io/crates/v/rustos.svg
[docs]: https://docs.rs/rustos/badge.svg
[writing an os in rust]: https://os.phil-opp.com/
[a freestanding rust binary]: https://os.phil-opp.com/freestanding-rust-binary/
[a minimal rust kernel]: https://os.phil-opp.com/minimal-rust-kernel/
[vga text mode]: https://os.phil-opp.com/vga-text-mode/
[testing]: https://os.phil-opp.com/testing/
[cpu exceptions]: https://os.phil-opp.com/cpu-exceptions/
[double faults]: https://os.phil-opp.com/double-fault-exceptions/
[hardware interrupts]: https://os.phil-opp.com/hardware-interrupts/
[introduction to paging]: https://os.phil-opp.com/paging-introduction/
[paging implementation]: https://os.phil-opp.com/paging-implementation/
[main.rs]: src/main.rs
[post01.rs]: examples/post01.rs
[post02.rs]: examples/post02.rs
[post03.rs]: examples/post03.rs
[post04.rs]: examples/post04.rs
[tests/basic_boot.rs]: tests/basic_boot.rs
[tests/should_panic.rs]: tests/should_panic.rs
[post05.rs]: examples/post05.rs
[post06.rs]: examples/post06.rs
[tests/page_fault.rs]: tests/page_fault.rs
[tests/stack_overflow.rs]: tests/stack_overflow.rs
[post07.rs]: examples/post07.rs
[post08.rs]: examples/post08.rs
[post09.rs]: examples/post09.rs
