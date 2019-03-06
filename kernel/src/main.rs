#![no_std]
#![no_main]

extern crate common;
use common::*;

mod console;
use console::*;

use core::fmt::Write;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn kmain(framebuffer: &'static Framebuffer) {
    let mut framebuffer = framebuffer.clone();
    framebuffer.fill(0, 0, 0);
    let mut console = Console::new(framebuffer);

    writeln!(console, "Hello, Kernel!").unwrap();

    loop {}
}
