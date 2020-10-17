#![no_std]
#![no_main]
#![feature(llvm_asm)]

extern crate common;
use common::*;

extern crate mmu;

mod console;
use console::*;

use core::fmt::Write;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn kmain(framebuffer: *const Framebuffer) {
    // Save a copy of the framebuffer.
    let mut framebuffer = unsafe { (*framebuffer).clone() };

    // Unmap the first GiB.
    unsafe {
        // Get the page table.
        let mut pt4_ptr: *mut mmu::amd64::PageTable;
        llvm_asm!("movq %cr3, $0"
             : "=r"(pt4_ptr)
             :
             :
        );
        let pt4 = &mut *pt4_ptr;

        // Clear the first entry. Leaks the level 3 table but we don't care at this point.
        pt4[0].clear();

        // Flush the TLB.
        llvm_asm!("movq %cr3, %rax; movq %rax, %cr3" : : : "rax");
    }

    // Initialize the console.
    framebuffer.fill(0, 0, 0);
    let mut console = Console::new(framebuffer);

    writeln!(console, "Hello, Kernel!").unwrap();

    for i in 0..99 {
        let bottles = 99 - i;
        writeln!(console, "{} bottles of beer on the wall, {} bottles of beer, take one down, pass it around, {} bottles of beer on the wall", bottles, bottles, bottles - 1).unwrap();
    }

    loop {}
}
