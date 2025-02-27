#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    let _x = 66;

    loop {}
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}
