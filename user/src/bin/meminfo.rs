#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{MemoryInfo, meminfo};

fn kib(bytes: usize) -> usize {
    bytes / 1024
}

fn mib(bytes: usize) -> usize {
    bytes / 1024 / 1024
}

fn paging_mode_name(mode: usize) -> &'static str {
    match mode {
        8 => "Sv39",
        _ => "unknown",
    }
}

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let mut info = MemoryInfo::default();
    if meminfo(&mut info) != 0 {
        println!("meminfo: syscall failed");
        return -1;
    }

    println!("[TanOS] Memory Information");
    println!(
        "managed range: {:#x} - {:#x}",
        info.managed_start, info.managed_end
    );
    println!("page size: {} bytes", info.page_size);
    println!(
        "frames: total={} used={} free={} recycled={}",
        info.total_frames, info.used_frames, info.free_frames, info.recycled_frames
    );
    println!(
        "managed memory: total={} MiB used={} KiB free={} MiB",
        mib(info.total_frames * info.page_size),
        kib(info.used_frames * info.page_size),
        mib(info.free_frames * info.page_size)
    );
    println!(
        "kernel heap: initialized={} total={} KiB used={} KiB free={} KiB",
        info.heap_initialized != 0,
        kib(info.heap_total_bytes),
        kib(info.heap_used_bytes),
        kib(info.heap_free_bytes)
    );
    println!(
        "paging: mode={} ({})",
        info.paging_mode,
        paging_mode_name(info.paging_mode)
    );
    0
}
