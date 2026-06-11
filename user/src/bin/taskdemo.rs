#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{fork, getpid, waitpid, yield_};

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    println!("[taskdemo] parent pid = {}", getpid());

    let child_pid = fork();
    if child_pid == 0 {
        println!("[taskdemo] child branch ok");
        yield_();
        return 0;
    }

    if child_pid < 0 {
        panic!("[taskdemo] fork failed");
    }

    let mut exit_code = -1;
    let waited_pid = waitpid(child_pid as usize, &mut exit_code);
    assert_eq!(waited_pid, child_pid);
    assert_eq!(exit_code, 0);

    println!("[taskdemo] wait ok");
    println!("[taskdemo] demo passed");
    0
}
