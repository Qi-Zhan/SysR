//! ----------------------- VME: Virtual Memory -----------------------
//! bool     vme_init    (void *(*pgalloc)(int), void (*pgfree)(void *));
//! void     protect     (AddrSpace *as);
//! void     unprotect   (AddrSpace *as);
//! void     map         (AddrSpace *as, void *vaddr, void *paddr, int prot);
//! Context *ucontext    (AddrSpace *as, Area kstack, void *entry);
//! Not implemented: :)

pub struct AddrSpace {
    pub pgsize: usize,
    pub area: Area,
    pub ptr: *mut u8,
}

pub struct Area {
    pub start: usize,
    pub end: usize,
}

impl Area {
    pub fn new(start: usize, end: usize) -> Area {
        Area { start, end }
    }

    pub fn size(&self) -> usize {
        self.end - self.start
    }

    pub fn inrange(&self, addr: usize) -> bool {
        addr >= self.start && addr < self.end
    }
}
