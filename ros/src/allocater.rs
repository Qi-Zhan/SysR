use core::alloc::{GlobalAlloc, Layout};
use ram::{println, print};
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

const HEAP_START: usize = 0x84000000;
const HEAP_END: usize = 0xffffffff;
static mut INDEX : usize = HEAP_START;
struct Mutex<T> {
    locked: bool,
    data: T,
}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Mutex {
            locked: false,
            data,
        }
    }
    pub fn lock(&mut self) -> MutexGuard<T> {
        while self.locked {
            // spin
        }
        self.locked = true;
        MutexGuard {
            lock: self,
        }
    }
}

struct MutexGuard<'a, T> {
    lock: &'a Mutex<T>,
}

pub struct Locked<A> {
    inner: Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: Mutex::new(inner),
        }
    }
    pub fn lock(&mut self) -> MutexGuard<A> {
        self.inner.lock()
    }
}

pub struct MyAllocator {
    start: usize,
    end: usize,
    next: usize,
}

#[global_allocator]
static ALLOCATOR: Locked<MyAllocator> = Locked::new(MyAllocator {
    start: HEAP_START,
    end: HEAP_END,
    next: HEAP_START,
});

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

unsafe impl GlobalAlloc for Locked<MyAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        println!("allocating {} bytes with alignment {} from {:#x} to {:#x}", layout.size(), layout.align(), INDEX, INDEX + layout.size());
        let alloc_start = align_up(INDEX, layout.align());        
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(alloc_end) => alloc_end,
            None => return core::ptr::null_mut(),
        };
        INDEX = alloc_end;
        alloc_start as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    }
}
