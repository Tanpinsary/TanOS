#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{exit, semaphore_create, semaphore_down, semaphore_up, thread_create, waittid};

const SEM_READY: usize = 0;

fn worker() -> ! {
    println!("[syncdemo] worker ready");
    semaphore_up(SEM_READY);
    exit(0)
}

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let sem_id = semaphore_create(0);
    if sem_id < 0 {
        println!("[syncdemo] demo failed: semaphore_create returned {}", sem_id);
        return 1;
    }
    if sem_id as usize != SEM_READY {
        println!("[syncdemo] demo failed: unexpected semaphore id {}", sem_id);
        return 1;
    }

    let tid = thread_create(worker as usize, 0);
    if tid < 0 {
        println!("[syncdemo] demo failed: thread_create returned {}", tid);
        return 1;
    }

    semaphore_down(SEM_READY);
    println!("[syncdemo] semaphore ok");

    let exit_code = waittid(tid as usize);
    if exit_code != 0 {
        println!("[syncdemo] demo failed: worker exited with {}", exit_code);
        return 1;
    }

    println!("[syncdemo] demo passed");
    0
}
