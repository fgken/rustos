//#![crate_type="lib"]
#![feature(lang_items, start, no_std, core, asm)]
#![no_std]

extern crate core;
use core::str::StrExt;

//use core::prelude::*;

extern {
    fn test_main();
}

fn console_init() {
    let base: u16 = 0x3f8;
    //let baud = 115200;

    let LCR = 3;    // Line Control
    let IER = 1;    // Interrupt Enable
    let FCR = 2;    // FIFO Control
    let MCR = 4;    // Modem Control

    let DLAB = 0x80; // Divisor Latch Access Bit
    let DLL = 0;    // Divisor Latch Low
    let DLH = 1;    // Divisor Latch High

    unsafe {
        // 8 bits, no parity, one stop bit
        asm!("out %0, %1" : : "a"(0x03), "dN"(base + LCR));
        // Disable all interrupts
        asm!("out %0, %1" : : "a"(0x00), "dN"(base + IER));
        // Disable FIFO
        //asm!("outb %0, %1" : : "a"(0x00), "dN"(base + FCR));
        asm!("out %0, %1" : : "a"(0xC7), "dN"(base + FCR));
        // DTR + RTS
        //asm!("outb %0, %1" : : "a"(0x03), "dN"(base + MCR));
        // Turn on DTR, RTS, and OUT2
        asm!("out %0, %1" : : "a"(0x0B), "dN"(base + MCR));

        //let divisor: u16 = 115200 / baud;
        let divisor: u16 = 1;
        let mut c: u16 = 0;
        asm!("in %0, %1" : "=a"(c) : "dN"(base + LCR));

        // Enable DLAB (set baud rate divisor)
        asm!("out %0, %1" : : "a"(c|DLAB), "dN"(base + LCR));

        // Set divisor to 3 (lo byte) 115200 baud
        asm!("out %0, %1" : : "a"(divisor & 0x00ff), "dN"(base + DLL));
        //                  (hi byte)
        asm!("out %0, %1" : : "a"((divisor >> 8) & 0x00ff), "dN"(base + DLH));

        // Enable FIFO, clear them, with 14-byte threshold
        //asm!("outb %0, %1" : : "a"(0xC7), "dN"(base + 2));
        // IRQs enabled, RTS/DSR set
        //asm!("outb %0, %1" : : "a"(0x0B), "dN"(base + 4));
        
        // Set divisor to 3 (lo byte) 38400 baud
        //asm!("outb %0, %1" : : "a"(0x03), "dN"(base + 0));
        //                  (hi byte)
        //asm!("outb %0, %1" : : "a"(0x00), "dN"(base + 1));
        // Enable FIFO, clear them, with 14-byte threshold
        //asm!("outb %0, %1" : : "a"(0xC7), "dN"(base + 2));
        // IRQs enabled, RTS/DSR set
        //asm!("outb %0, %1" : : "a"(0x0B), "dN"(base + 4));
        
        
        //asm!("outb %0, %1" : : "a"(0x80|0x3), "dN"(base));
        //asm!("outb %0, %1" : : "a"(1), "dN"(base+0));
        //asm!("outb %0, %1" : : "a"(0), "dN"(base + 1));
        //asm!("outb %0, %1" : : "a"(3), "dN"(base + 3));
        //asm!("outb %0, %1" : : "a"(8), "dN"(base + 4));
        //asm!("outb %0, %1" : : "a"(0x41), "dN"(base));
    }
}

//fn console_in() -> u8 {
//    let mut c: u8 = 0;
//
//    unsafe {
//        let mut stat: u8 = 0;
//        let mut done = false;
//
//        while !done {
//            asm!("in %0, %1" : "=a"(stat) : "dN"(0x3f8+5));    // REG_LSR
//
//            match stat {
//                0 => (),
//                1 => printk("1"),
//                2 => printk("2"),
//                3 => printk("3"),
//                4 => printk("4"),
//                5 => printk("5"),
//                6 => printk("6"),
//                7 => printk("7"),
//                8 => printk("8"),
//                9 => printk("9"),
//                _ => printk("_"),
//            }
//
//            if (stat & 0x01) == 1 {
//                printk("BBB");
//                asm!("in %0, %1" : "=a"(c) : "dN"(0x3f8));    // REG_DATA
//                done = true;
//            }
//        }
//    }
//
//    c
//}

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
    console_out(0x0a);  // '\n'
}

//#[start]
#[no_mangle]
pub extern fn kernel_main() {
    console_init();
    
    printk("--- start rust os ---");

//    printk("input");
//    let c = console_in();
//    printk("output");
//    console_out(c);
    
    printk("Goto C code");
    unsafe { test_main() };
    printk("Return from C code");
    
    // init IDT

    // init GDT

    printk("--- finish rust os ---");
    loop{}
}

#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop{} }

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

