<<<<<<< HEAD
#![no_std]
#![no_main]
#![allow(unused_imports)]
use os;
=======
#![no_std]
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info:&PanicInfo)->!{
loop{}
}
fn main() {
    // println!("Hello, world!");
}
>>>>>>> c9430f7a64a11bbb6250342dd8f7a1452675fd6a
