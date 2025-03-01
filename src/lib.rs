#![no_std]

use core::{
    panic::PanicInfo,
    ptr::{self, addr_of, addr_of_mut},
};

/**
    This is the entry point for the firmware, whose address is stored as the second entry in the vector table,
    after the initial stack pointer. This functions job is to provide a suitable runtime environment for the program.
    This includes exposing symbols to the linker so it can place them where the bootloader expects them, zeroing out
    the .bss section of RAM, and populating the .data section of RAM with the initial values store in flash.
*/
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;

        static _sidata: u8;
    }

    // zero out .bss
    let count = addr_of!(_ebss) as usize - addr_of!(_sbss) as usize;
    ptr::write_bytes(addr_of_mut!(_sbss), 0, count);

    // copy from .sidata into .data
    let count = addr_of!(_edata) as usize - addr_of!(_sdata) as usize;
    ptr::copy_nonoverlapping(addr_of!(_sidata), addr_of_mut!(_sdata), count);

    extern "Rust" {
        fn main() -> !;
    }

    main();
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}

/**
   Wrapper to specify a main function. This is called immediately after `Reset()`.
*/
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            let f: fn() -> ! = $path;

            f()
        }
    };
}
