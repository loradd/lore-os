#![cfg_attr(not(test), no_std)] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(unused_imports))] // allow unused imports on test

// This function is the entry point, since the linker
// looks for a function named `_start` by default
#[cfg(not(test))] // only compile when the test flag is not set
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    lore_os::gdt::init();
    lore_os::interrupts::init_idt();
    unsafe { lore_os::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    lore_os::hlt_loop();
}

#[cfg(not(test))] // only compile when the test flag is not set
#[panic_handler] // this function is called on panic
fn panic(info: &core::panic::PanicInfo) -> ! {
    lore_os::println!("{}", info);
    lore_os::hlt_loop();
}
