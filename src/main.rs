#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use core::panic;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let val = 56;
    println!("Ahoj světe {}", val);
    loop {}
}

#[panic_handler]
fn panic(info: &panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
