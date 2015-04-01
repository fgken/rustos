#![feature(no_std)]
#![feature(int_uint)]
#![feature(core)]
#![feature(lang_items)]
#![feature(asm)]

#![no_std]

extern crate core;

use core::prelude::*;

use core::mem;

fn outb(val: u8, port: u16) {
    unsafe {
        asm!("outb %0, %1" : : "a"(val), "dN"(port)::);
    }
}

//fn inb(port: u16) -> u8 {
//    let mut val: u8;
//    unsafe {
////        asm!("intb %1, %0" : "=a"(val) : "dN"(port)::);
//    }
//    val = 0u8;
//    val
//}

enum CONSOLE {
    DEFAULT_SERIAL_PORT = 0x3f8,
    LCR_DLAB = 0x80,
    LCR_LEN8BIT = 0x03,
    MCR_AUX_OUTPUT_2 = 0x8,
}

enum CONSOLE_REG {
    DLL = 0x00,
    IER = 0x01,
    LCR = 0x03,
    MCR = 0x04,
}

fn serial_early_init() {
    //let port = CONSOLE::DEFAULT_SERIAL_PORT;
    let port = 0x3f8;
    
    //outb(CONSOLE::LCR_LEN8BIT + CONSOLE::LCR_DLAB, port);
    //outb(1, port + CONSOLE_REG::DLL);
    //outb(0, port + CONSOLE_REG::IER);
    //outb(CONSOLE::LCR_LEN8BIT, port + CONSOLE_REG::LCR);
    //outb(CONSOLE::MCR_AUX_OUTPUT_2, port + CONSOLE_REG::MCR);
    outb(0x80 | 0x3, port);
    outb(1, port + 0);
    outb(0, port + 1);
    outb(3, port + 3);
    outb(8, port + 4);
}

fn serial_puts(c: &str) {
    outb(0x41, 0x3f8);
    //outb(c, CONSOLE::DEFAULT_SERIAL_PORT);
//    for b in c.bytes() {
//        outb(b, 0x3f8);
//    }
}

fn serial_out(c: u8) {
    unsafe {
        asm!("nop");
        asm!("mov %al, 0x41");
        asm!("mov %dx, 0x3f8");
        asm!("outb %al, %dx");
        asm!("outb $0, $1" : : "a"(c), "Nd"(0x3f8));
        //asm!("outb $1, $0" :: "{dx}"(0x3f8), "{al}"(c) ::)
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
        loop {
            asm!("mov $$0x41, %al");
            asm!("mov $$0x3f8, %dx");
            asm!("out %al, %dx");
            //asm!("outb %0, %1" : : "a"(0x41), "dN"(0x3f8));
        }
    }
    serial_early_init();
    loop{serial_puts("A");}
    serial_out(0x41);
    loop{}
}

#[no_mangle]
pub extern fn dot_product(a: *const u32, a_len: u32,
                          b: *const u32, b_len: u32) -> u32 {
    use core::raw::Slice;

    // Convert the provided arrays into Rust slices.
    // The core::raw module guarantees that the Slice
    // structure has the same memory layout as a &[T]
    // slice.
    //
    // This is an unsafe operation because the compiler
    // cannot tell the pointers are valid.
    let (a_slice, b_slice): (&[u32], &[u32]) = unsafe {
        mem::transmute((
            Slice { data: a, len: a_len as usize },
            Slice { data: b, len: b_len as usize },
        ))
    };

    // Iterate over the slices, collecting the result
    let mut ret = 0;
    for (i, j) in a_slice.iter().zip(b_slice.iter()) {
        ret += (*i) * (*j);
    }
    return ret;
}

#[lang = "panic_fmt"]
extern fn panic_fmt(args: &core::fmt::Arguments,
                    file: &str,
                    line: u32) -> ! {
    loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
