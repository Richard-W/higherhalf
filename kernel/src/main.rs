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

    for i in 0..99 {
        let bottles = 99 - i;
        writeln!(console, "{} bottles of beer on the wall, {} bottles of beer, take one down, pass it around, {} bottles of beer on the wall", bottles, bottles, bottles - 1).unwrap();
    }

    loop {}
}
