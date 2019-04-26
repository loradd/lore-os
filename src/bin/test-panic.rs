#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    lore_os::gdt::init();
    lore_os::interrupts::init_idt();
    unsafe { lore_os::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    panic!();

    lore_os::serial_println!("failed");
    lore_os::serial_println!("No panic occurred");
    unsafe { lore_os::exit_qemu(); }
    lore_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    lore_os::serial_println!("ok");
    lore_os::serial_println!("{}", info);
    unsafe { lore_os::exit_qemu(); }
    lore_os::hlt_loop();
}
