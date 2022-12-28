#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::string::String;
use blog_os::serial_println;
//use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}


#[test]
fn test_println() -> Result<(), String> {
    serial_println!("test_println output");
    Ok(())
}

// #[test]
// fn test_failure() -> Result<(), String> {
//     serial_println!("test errored");
//     // We can't actually alloc, so this panics at runtime.
//     let s = String::from("some failure reason");
//     serial_println!("created string");
//     Err(s)
// }