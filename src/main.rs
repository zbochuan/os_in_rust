#![no_std] // do not link std lib
#![no_main] // disable all rust level entry point

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
    }
}

#[unsafe(no_mangle)] // do not mangle the name of the function
pub extern "C" fn _start() -> ! {
    print!("Hello world{}", "!");
    loop {}
}
