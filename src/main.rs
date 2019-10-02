// Disable the link to the standart library
#![no_std]

/* 
 * Tell Rust we dont want to have the normal entry point
 * because we dont can initialize the stack and so on without
 * the standart library that do it normaly for us
 */
#![no_main]
              //Static Byte String
mod vga_buffer;

/*
 * We overwrite the operating system entry point with _start
 * its the most common entry point name
 * this function needs to use the c calling conventions
 * @return{!} this function dont returns so return-type is never
 * @[no_mangle] tells the compiler to use the name _start and not
 * any cryptic name like: _ZN37hb173fedf945531caE
 */
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    loop{}
}

use core::panic::PanicInfo;
/* 
 * Handles a Panic when something in code happend that is a 
 * Unrecoverable Error this function get catched
 * @param{&PanicInfo} contains the file and line where the panic
 * is happened
 * @return{!} this function dont returns so return-type is never
 */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}