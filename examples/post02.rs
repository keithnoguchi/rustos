//! [A Minimal Rust Kernel]
//!
//! [a minimal rust kernel]: https://os.phil-opp.com/minimal-rust-kernel/
//!
//! # Examples
//!
//! ```sh
//! $ cargo xrun --target x86_64-os.json --example post02
//! Finished dev [unoptimized + debuginfo] target(s) in 0.00s
//! Running `bootimage runner target/x86_64-os/debug/examples/post02`
//! Building bootloader
//! Finished release [optimized + debuginfo] target(s) in 0.01s
//! Running: `qemu-system-x86_64 -drive format=raw,file=target/x86_64-os/debug/examples/bootimage-post02.bin`
//! VNC server running on ::1:5900
//! ```
#![no_std]
#![no_main]
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    // Hello world output to VGA buffer.
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb + i as u8; // color byte
        }
    }
    loop {}
}
