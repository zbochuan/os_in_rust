#![no_std] // do not link std lib
#![no_main] // disable all rust level entry point

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)] // do not mangle the name of the function
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    loop {}
}
