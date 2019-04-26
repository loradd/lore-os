#![feature(abi_x86_interrupt)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[cfg(not(test))]
#[no_mangle]
#[allow(unconditional_recursion)]
pub extern "C" fn _start() -> ! {
    lore_os::gdt::init();
    lore_os::interrupts::init_idt();
    unsafe { lore_os::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    init_test_idt();
    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }
    // trigger a stack overflow
    stack_overflow();

    lore_os::serial_println!("failed");
    lore_os::serial_println!("No exception occurred");
    unsafe { lore_os::exit_qemu(); }
    lore_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    lore_os::serial_println!("failed");
    lore_os::serial_println!("{}", info);
    unsafe { lore_os::exit_qemu(); }
    lore_os::hlt_loop();
}

lazy_static::lazy_static! {
    static ref TEST_IDT: x86_64::structures::idt::InterruptDescriptorTable = {
        let mut idt = x86_64::structures::idt::InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(lore_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: &mut x86_64::structures::idt::InterruptStackFrame,
    _error_code: u64,
) {
    lore_os::serial_println!("ok");
    unsafe { lore_os::exit_qemu(); }
    lore_os::hlt_loop();
}
