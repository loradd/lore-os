#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    lore_os::gdt::init();
    lore_os::interrupts::init_idt();
    unsafe { lore_os::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    x86_64::instructions::interrupts::int3();

    lore_os::serial_println!("ok");
    unsafe { lore_os::exit_qemu(); }
    lore_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    lore_os::serial_println!("failed");
    lore_os::serial_println!("{}", info);
    unsafe { lore_os::exit_qemu(); }
    lore_os::hlt_loop();
}
