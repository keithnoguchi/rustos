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
use rustos::{serial_print, serial_println};
use x86_64::{structures::paging::MapperAllSizes, PhysAddr, VirtAddr};

entry_point!(test_kernel);

fn test_kernel(boot_info: &'static BootInfo) -> ! {
    serial_print!("tests::page_table::test_kernel... ");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { rustos::memory::init_page_table(phys_mem_offset) };
    let addresses = [
        // the identity-mapped vga buffer page.
        0xb8000,
        // some code page.
        0x0020_1008,
        // some stack page.
        0x0100_0020_1a10,
    ];
    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let want = unsafe { translate_addr(virt, phys_mem_offset) };
        let got = mapper.translate_addr(virt);
        assert_eq!(got, want);
    }
    serial_println!("[ok]");
    test_main();
    rustos::hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info);
}

unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}

fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::{page_table::FrameError, PageTable};

    let (level_4_table_frame, _) = Cr3::read();
    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut frame = level_4_table_frame;
    for &index in &table_indexes {
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }
    Some(frame.start_address() + u64::from(addr.page_offset()))
}
