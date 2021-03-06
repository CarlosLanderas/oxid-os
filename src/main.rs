#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oxid_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use oxid_os::{memory, println};

entry_point!(kernel_main);


fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use oxid_os::memory;
    use x86_64::{structures::paging::Page, VirtAddr}; 
    println!("Hello World{}", "!");
    
    oxid_os::init();

    println!("Physical memory offset: 0x{:x}", boot_info.physical_memory_offset);
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset)};
    let mut frame_allocator = memory::EmptyFrameAllocator;

    //map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    #[cfg(test)]
    test_main();

    oxid_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    oxid_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) ->! {
    oxid_os::test_panic_handler(info)
}
