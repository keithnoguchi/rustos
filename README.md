# rustos

[![drone]](https://cloud.drone.io/keithnoguchi/rustos)
[![crate]](https://lib.rs/rustos)
[![docs]](https://docs.rs/rustos)

Philipp Oppermann's *awesome* [Writing an OS in Rust]

- Bare bones
  - [A Freestanding Rust Binary] : [post01.rs](examples/post01.rs)
  - [A Minimal Rust Kernel] : [post02.rs](examples/post02.rs)
  - [VGA Text Mode] : [post03.rs](examples/post03.rs)
  - [Testing] : [post04.rs](examples/post04.rs)
    - [tests/basic_boot.rs](tests/basic_boot.rs)
    - [tests/should_panic.rs](tests/should_panic.rs)
- Interrupts
  - [CPU Exceptions] : [post05.rs](examples/post05.rs)
  - [Double Faults] : [post06.rs](examples/post06.rs)
    - [tests/page_fault.rs](tests/page_fault.rs)
    - [tests/stack_overflow.rs](tests/stack_overflow.rs)
  - [Hardware Interrupts] : [post07.rs](examples/post07.rs)
- Memroy Management
  - [Introduction to Paging] : [post08.rs](examples/post08.rs)
  - [Paging Implementation] : [post09.rs](examples/post09.rs)
  - [Heap Allocation] : [post10.rs](examples/post10.rs)
    - [tests/heap_allocation.rs](tests/heap_allocation.rs)
  - [Allocator Designs] : [post11.rs](examples/post11.rs)
- Multitasking
  - [Async/Await] : [post12.rs](examples/post12.rs)

## main.rs

Current [main.rs](src/main.rs):

```rust
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate bootloader;
extern crate rustos;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rustos::{println, task};

entry_point!(start_kernel);

fn start_kernel(boot_info: &'static BootInfo) -> ! {
    println!("Welcome to the real world!");

    // Initialize the kernel.
    rustos::init();
    rustos::memory::init(boot_info);

    // Spawn async task(s).
    let mut executor = task::Executor::new();
    executor.spawn(task::Task::new(example_task()));

    #[cfg(test)]
    test_main();
    println!("It did not crash!!!");

    // Run forever.
    executor.run()
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

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

async fn async_number() -> u32 {
    42
}
```

## Execution

You can run the current [main.rs] with `make run`:

```sh
make run
```

or the previous posts, e.g. [post01.rs] with `make run-post_name` as:

```sh
make run-post01
```

## Tests

You can run all the integration test with `make test`:

```sh
make test
```

or specific tests with `make tsst-test_name as:

```sh
make test-heap_allocation
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
[heap allocation]: https://os.phil-opp.com/heap-allocation/
[allocator designs]: https://os.phil-opp.com/allocator-designs/
[async/await]: https://os.phil-opp.com/async-await/
