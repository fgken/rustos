//#![crate_type="lib"]
#![feature(lang_items, start, no_std, core, asm, libc)]
#![no_std]

extern crate core;

use core::prelude::*;

use core::mem;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

#[no_mangle]
pub extern fn ppp() {
    let x = 1;
    match x {
        0 => loop{},
        _ => loop{},
    }
}

#[no_mangle]
pub extern fn kernel_main() {
    unsafe {
        asm!("outb %0, %1" : : "a"(0x80|0x3), "dN"(0x3f8)::);
        asm!("outb %0, %1" : : "a"(1), "dN"(0x3f8+0));
        asm!("outb %0, %1" : : "a"(0), "dN"(0x3f8 + 1));
        asm!("outb %0, %1" : : "a"(3), "dN"(0x3f8 + 3));
        asm!("outb %0, %1" : : "a"(8), "dN"(0x3f8 + 4));
        asm!("outb %0, %1" : : "a"(0x41), "dN"(0x3f8));
        for i in 0..3 {
            asm!("mov $$0x41, %al");
            asm!("mov $$0x3f8, %dx");
            asm!("out %al, %dx");
            //asm!("outb %0, %1" : : "a"(0x41), "dN"(0x3f8));
        }
    }

    loop{}
}

#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop{} }

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

