//#![crate_type="lib"]
#![feature(lang_items, start, no_std, core, asm)]
#![no_std]

extern crate core;
use core::str::StrExt;

//use core::prelude::*;

fn console_out(c: u8) {
    unsafe {
        asm!("mov %0, %al" : : "a"(c));
        asm!("mov $$0x3f8, %dx");
        asm!("out %al, %dx");
    }
}

fn printk(s: &str) {
    for c in s.bytes() {
        console_out(c);
    }
}

//#[start]
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

    printk("ABCDEFG");

    loop{}
}

#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop{} }

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

