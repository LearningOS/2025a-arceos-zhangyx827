#![no_std]

use core::ptr::NonNull;
// use core::alloc::AllocError;
use allocator::AllocError;

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const SIZE: usize> {
    start: usize,
    b_pos: usize,
    b_end: usize,
    p_pos: usize,
    end: usize,
    total_bytes: usize,
    used_bytes: usize,
    total_pages: usize,
    used_pages: usize,
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            b_pos: 0,
            end: 0,
            p_pos: 0,
            b_end: 0,
            used_bytes: 0,
            total_bytes: 0,
            total_pages: 0,
            used_pages: 0,
        }
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.b_pos = start;
        self.end = start + size;
        self.p_pos = start + size;
        self.used_bytes = 0;
        self.total_bytes = size;
        self.total_pages = size / SIZE;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        todo!()
    }
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        if self.b_pos + layout.size() > self.p_pos {
            return Err(AllocError::NoMemory);
        }
        unsafe {
            let ptr = NonNull::new_unchecked(self.b_pos as *mut u8);
            self.used_bytes += layout.size();
            self.total_bytes -= layout.size();
            self.total_pages -= layout.size() / SIZE;
            Ok(ptr)
        }
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        // todo!()
        
    }

    fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    fn used_bytes(&self) -> usize {
        self.used_bytes
    }

    fn available_bytes(&self) -> usize {
        self.total_bytes - self.used_bytes
    }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        if align_pow2 % Self::PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
        let align_pow2 = align_pow2 / SIZE;
        if !align_pow2.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }
        if self.p_pos - num_pages * SIZE < self.b_pos {
            return Err(AllocError::NoMemory);
        }


        self.p_pos -= num_pages * SIZE;
        self.total_pages += num_pages;
        self.used_pages += num_pages;
        self.total_bytes -= num_pages * SIZE;
        self.total_pages -= num_pages;
        Ok(self.p_pos)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        todo!()
    }

    fn total_pages(&self) -> usize {
        self.total_pages
    }

    fn used_pages(&self) -> usize {
        self.used_pages
    }

    fn available_pages(&self) -> usize {
        self.total_pages - self.used_pages
    }
}