//! [A Freestanding Rust Binary]
//!
//! [A Freestanding Rust Binary]: https://os.phil-opp.com/freestanding-rust-binary/
//!
//! # Examples
//!
//! ```sh
//! $ cargo rustc --example post01 -- -C link-args=-nostartfiles
//! Compiling os-blog v0.1.0 (/home/kei/git/books-rs/os)
//! Finished dev [unoptimized + debuginfo] target(s) in 0.10s
//! ```
//! ```sh
//! $ file ../target/debug/examples/post01
//! ../target/debug/examples/post01: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=b8592aa5e5cd64dfee255685b730d14380b73167, with debug_info, not stripped
//! ```
#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
