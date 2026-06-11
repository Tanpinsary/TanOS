#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    println!("[TanOS] Feature Showcase Index");
    println!("Memory management        -> meminfo");
    println!("Filesystem and file API  -> fsdemo");
    println!("System calls             -> syscalldemo");
    println!("Task/process/thread      -> taskdemo");
    println!("Synchronization          -> syncdemo");
    println!("Optional networking      -> udp, tcp_simplehttp");
    println!("Recommended run order: features, meminfo, fsdemo, syscalldemo, taskdemo, syncdemo");
    println!("[features] showcase index ready");
    0
}
