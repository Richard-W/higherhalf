#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]

extern crate common;
use common::*;

extern crate efw;
use efw::*;
use efw::efi::Protocol;

extern crate mmu;

extern crate xmas_elf as elf;

/// ELF image of the kernel.
const KERNEL_BYTES: &[u8] = include_bytes!("../../target/higherhalf/debug/kernel");

/// Page table to use when boot services are terminated.
static mut PT4: mmu::amd64::PageTable = mmu::amd64::PageTable::new();

unsafe fn allocate_pages(num_pages: usize) -> efi::Result<u64> {
    efi::SystemTable::get()
        .boot_services()
        .allocate_pages(
            efi::bits::AllocateType::AllocateAnyPages,
            efi::bits::MemoryType::RuntimeServicesData,
            num_pages,
        )
        .map(|x| x as _)
}

#[no_mangle]
unsafe fn efw_main() {
    println!("Boot starting");

    // Create the mapper for the future page table.
    let mut mapper = mmu::amd64::RecursiveMapper::new(&mut PT4, || allocate_pages(1).map_err(|_| mmu::Error::NoMemory), |phys_addr| phys_addr);

    // Get the maximum physical memory address.
    let max_phys_addr: usize = {
        let uefi_mem_map = efi::SystemTable::get()
            .boot_services()
            .get_memory_map()
            .unwrap()
            ;

        uefi_mem_map.iter().fold(0, |max_addr, desc| {
            let desc_upper_addr = desc.physical_start as usize + (0x1000 * desc.number_of_pages as usize);
            core::cmp::max(max_addr, desc_upper_addr)
        })
    };
    println!("Available physical memory: {}MiB", max_phys_addr / 0x10_0000);

    // Map physical memory to 0xffff_c000_0000_0000
    for addr in (0..core::cmp::max(0x1_0000_0000, max_phys_addr)).step_by(0x4000_0000) {
        mapper.entry(0xffff_c000_0000_0000 + addr as u64, 3).unwrap()
            .set_address(addr as u64)
            .set_bit(mmu::amd64::Bit::Present)
            .set_bit(mmu::amd64::Bit::Writable)
            .set_bit(mmu::amd64::Bit::Huge)
            ;
    }

    // Identity map the first GiB
    mapper.entry(0x0, 3).unwrap()
        .set_address(0x0)
        .set_bit(mmu::amd64::Bit::Present)
        .set_bit(mmu::amd64::Bit::Writable)
        .set_bit(mmu::amd64::Bit::Huge)
        ;

    // Map the kernel
    let kernel = elf::ElfFile::new(KERNEL_BYTES).unwrap();
    for program_header in kernel.program_iter() {
        match program_header.get_type().unwrap() {
            elf::program::Type::Load => {
                println!("Loading program header: {:x?}", program_header);
                assert!(program_header.virtual_addr() % 0x1000 == 0);

                // Allocate physical space for the section.
                let pages = (program_header.mem_size() as usize + 0xfff) / 0x1000;
                let phys_start = allocate_pages(pages).unwrap() as *mut u8;

                // Zero the memory.
                for offset in 0..(pages*0x1000) {
                    phys_start.offset(offset as _).write(0);
                }

                // Copy the section.
                for offset in 0..program_header.file_size() {
                    phys_start.offset(offset as _).write(KERNEL_BYTES[(program_header.offset() + offset) as usize])
                }

                // Map the section to virtual memory.
                let virt_start = program_header.virtual_addr();
                for page in 0..pages {
                    let offset = (page * 0x1000) as u64;
                    let virt_addr = virt_start + offset;
                    let phys_addr = phys_start as u64 + offset;
                    mapper.entry(virt_addr, 1).unwrap()
                        .set_address(phys_addr)
                        .set_bit(mmu::amd64::Bit::Present)
                        ;
                }
            },
            _ => {},
        }
    }
    let entry_point = kernel.header.pt2.entry_point();

    // Get the framebuffer.
    let framebuffer = {
        let protocol = &mut efi::protocols::GraphicsOutput::find_instances().unwrap()[0];
        let mode = protocol.mode();
        let info = &mut *mode.info;
        let addr = 0xffff_c000_0000_0000 + mode.frame_buffer_base;
        Framebuffer::new(addr as _, info.horizontal_resolution as _, info.vertical_resolution as _, info.pixels_per_scan_line as _)
    };

    // Map a single page of stack space for the kernel
    let stack_addr = allocate_pages(1).unwrap();
    mapper.entry(0xffff_8180_0000_0000 - 0x1000, 1).unwrap()
        .set_address(stack_addr)
        .set_bit(mmu::amd64::Bit::Present)
        .set_bit(mmu::amd64::Bit::Writable)
        ;

    // Get UEFI memory map.
    let mut mem_map = efi::SystemTable::get()
        .boot_services()
        .get_memory_map()
        .unwrap()
        ;

    // Exit boot services.
    efi::SystemTable::get()
        .boot_services()
        .exit_boot_services(mem_map.key())
        .unwrap()
        ;

    // Use the new page table.
    asm!("movq $0, %cr3"
         :
         : "r"(&mut PT4 as *mut _)
         :
    );

    // Modify memory map so runtime services still work.
    for desc in mem_map.iter_mut() {
        desc.virtual_start = desc.physical_start + 0xffff_c000_0000_0000;
    }
    efi::SystemTable::get()
        .runtime_services()
        .set_virtual_address_map(&mut mem_map)
        .unwrap()
        ;

    call_kernel(entry_point, &framebuffer as _);

    loop {}
}

extern {
    #[no_mangle]
    fn call_kernel(entry_point: u64, framebuffer: *const Framebuffer);
}

global_asm!("
    .section .text
    .global call_kernel
    call_kernel:
        movq %rdx, %rdi
        movq $0xffff818000000000, %rsp
        jmpq *%rcx
");
