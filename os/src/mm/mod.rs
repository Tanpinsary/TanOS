mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

use crate::config::PAGE_SIZE;
pub use address::VPNRange;
pub use address::{PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
pub use frame_allocator::{
    FrameTracker, frame_alloc, frame_alloc_more, frame_allocator_stats, frame_dealloc,
};
pub use heap_allocator::heap_allocator_stats;
pub use memory_set::{KERNEL_SPACE, MapArea, MapPermission, MapType, MemorySet, kernel_token};
use page_table::PTEFlags;
pub use page_table::{
    PageTable, PageTableEntry, UserBuffer, translated_byte_buffer, translated_ref,
    translated_refmut, translated_str,
};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryInfo {
    pub managed_start: usize,
    pub managed_end: usize,
    pub page_size: usize,
    pub total_frames: usize,
    pub used_frames: usize,
    pub free_frames: usize,
    pub recycled_frames: usize,
    pub heap_initialized: usize,
    pub heap_total_bytes: usize,
    pub heap_used_bytes: usize,
    pub heap_free_bytes: usize,
    pub paging_mode: usize,
}

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}

pub fn memory_info() -> MemoryInfo {
    let frame = frame_allocator_stats();
    let heap = heap_allocator_stats();
    MemoryInfo {
        managed_start: frame.start_ppn * PAGE_SIZE,
        managed_end: frame.end_ppn * PAGE_SIZE,
        page_size: PAGE_SIZE,
        total_frames: frame.total_frames,
        used_frames: frame.used_frames,
        free_frames: frame.free_frames,
        recycled_frames: frame.recycled_frames,
        heap_initialized: heap.initialized as usize,
        heap_total_bytes: heap.total_bytes,
        heap_used_bytes: heap.used_bytes,
        heap_free_bytes: heap.free_bytes,
        paging_mode: 8,
    }
}
