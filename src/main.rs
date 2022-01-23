#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};

#[repr(C)]
pub struct PortGroup {
    pub data_direction: u32,
    pub data_direction_clear: u32,
    pub data_direction_set: u32,
    pub data_direction_toggle: u32,
    pub data_output_value: u32,
    pub data_output_value_clear: u32,
    pub data_output_value_set: u32,
    pub data_output_value_toggle: u32,
    pub data_input_value: u32,
    pub control: u32,
    pub write_configuration: u32,
    pub reserved_0: [u8; 4],
    pub peripheral_multiplexing: [u8; 16],
    pub pin_configuration: [u8; 32],
    pub reserved_1: [u8; 32],
}

#[repr(C)]
pub struct Port {
    pub groups: [PortGroup; 2],
}

const PORT_ADDRESS: u32 = 0x41004400;

unsafe fn enable_led() {
    let port = &mut *(PORT_ADDRESS as *mut Port);
    write_volatile(&mut port.groups[1].pin_configuration[8], 0b00000010);
    write_volatile(&mut port.groups[1].data_direction_set, 1 << 8);
}

unsafe fn turn_led_on() {
    let port = &mut *(PORT_ADDRESS as *mut Port);
    write_volatile(
        &mut port.groups[1].data_output_value_clear as *mut u32,
        1 << 8,
    )
}

unsafe fn turn_led_off() {
    let port = &mut *(PORT_ADDRESS as *mut Port);
    write_volatile(
        &mut port.groups[1].data_output_value_set as *mut u32,
        1 << 8,
    )
}

unsafe fn delay() {
    for i in 0..100000 {
        read_volatile(&i);
    }
}

unsafe fn run() -> ! {
    enable_led();
    loop {
        turn_led_on();
        delay();
        turn_led_off();
        delay();
    }
}

pub type Handler = unsafe extern "C" fn();

#[repr(C, packed)]
pub struct ExceptionTable {
    pub initial_stack: *const u32,
    pub reset: unsafe extern "C" fn() -> !,
    pub nmi: Handler,
    pub hard_fault: unsafe extern "C" fn() -> !,
    pub reserved_0: [Option<Handler>; 7],
    pub sv_call: Handler,
    pub reserved_1: [Option<Handler>; 2],
    pub pend_sv: Handler,
    pub sys_tick: Option<Handler>,
    pub external: [Option<Handler>; 32],
}

unsafe impl Sync for ExceptionTable {}

unsafe extern "C" fn reset_handler() -> ! {
    run()
}

unsafe extern "C" fn nop_handler() {}

unsafe extern "C" fn trap() -> ! {
    loop {}
}

#[link_section = ".isr_vector"]
pub static ISR_VECTORS: ExceptionTable = ExceptionTable {
    initial_stack: 0x20008000 as _,
    reset: reset_handler,
    nmi: nop_handler,
    hard_fault: trap,
    reserved_0: [None; 7],
    sv_call: nop_handler,
    reserved_1: [None; 2],
    pend_sv: nop_handler,
    sys_tick: None,
    external: [None; 32],
};

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
