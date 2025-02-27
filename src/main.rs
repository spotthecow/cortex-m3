#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}
