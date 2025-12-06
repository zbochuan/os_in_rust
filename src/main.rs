#![no_std] // do not link std lib
#![no_main] // disable all rust level entry point
use core::panic::PanicInfo;
#[unsafe(no_mangle)] // do not mangle the name of the function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
