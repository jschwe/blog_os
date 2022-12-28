#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(test)]

use core::panic::PanicInfo;
extern crate test;

pub mod serial;

use test::{TestDescAndFn, TestFn};

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&TestDescAndFn]) {
    serial_println!("Running {} tests", tests.len());
    let mut success = true;
    for test in tests {
        serial_println!("Running test `{}`:", test.desc.name.as_slice());
        match test.testfn {
            TestFn::StaticTestFn(test_fn) => match test_fn() {
                Ok(()) => {
                    serial_println!("test `{}` .. ok", test.desc.name.as_slice())
                }
                Err(msg) => {
                    serial_println!("test `{}` .. failed:\n {}", test.desc.name.as_slice(), msg);
                    success = false;
                }
            },
            TestFn::StaticBenchFn(_) => unreachable!(),
            TestFn::DynTestFn(_) => unreachable!(),
            TestFn::DynBenchFn(_) => unreachable!(),
        }
    }
    match success {
        true => exit_qemu(QemuExitCode::Success),
        false => exit_qemu(QemuExitCode::Failed),
    }
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}
