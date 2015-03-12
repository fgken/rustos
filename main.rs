#![feature(core)]

extern crate core;

extern {
    fn serial_out(charactor: u8);
}

fn puts(s: &str) {
    for c in s.bytes() {
        unsafe { serial_out(c) };
    }
}

#[no_mangle]
#[no_stack_check]
pub fn main() {
    let string = "Hello My Rust OS!!!";
    puts(string);
    loop{}
}

