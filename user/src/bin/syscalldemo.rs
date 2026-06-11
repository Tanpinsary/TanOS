#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{getpid, sleep, yield_};

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    println!("[syscalldemo] pid = {}", getpid());
    yield_();
    println!("[syscalldemo] yield ok");
    sleep(10);
    println!("[syscalldemo] sleep ok");
    println!("[syscalldemo] demo passed");
    0
}
