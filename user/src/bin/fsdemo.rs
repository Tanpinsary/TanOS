#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{OpenFlags, close, open, read, write};

const PATH: &str = "tanos_fsdemo.txt\0";
const CONTENT: &[u8] = b"TanOS fsdemo OK\n";
const PREFIX: &[u8] = b"TanOS fsdemo OK";

fn fail(reason: &str) -> i32 {
    println!("[fsdemo] demo failed: {}", reason);
    -1
}

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let fd = open(PATH, OpenFlags::CREATE | OpenFlags::TRUNC | OpenFlags::WRONLY);
    if fd < 0 {
        return fail("open for write");
    }
    let fd = fd as usize;

    let written = write(fd, CONTENT);
    if written != CONTENT.len() as isize {
        let _ = close(fd);
        return fail("write content");
    }

    if close(fd) < 0 {
        return fail("close after write");
    }
    println!("[fsdemo] create/write ok");

    let fd = open(PATH, OpenFlags::RDONLY);
    if fd < 0 {
        return fail("open for read");
    }
    let fd = fd as usize;

    let mut buffer = [0u8; 64];
    let read_len = read(fd, &mut buffer);
    if read_len < 0 {
        let _ = close(fd);
        return fail("read content");
    }

    if close(fd) < 0 {
        return fail("close after read");
    }

    let read_len = read_len as usize;
    if read_len < PREFIX.len() || &buffer[..PREFIX.len()] != PREFIX {
        return fail("readback mismatch");
    }
    println!("[fsdemo] readback ok");
    println!("[fsdemo] demo passed");
    0
}
