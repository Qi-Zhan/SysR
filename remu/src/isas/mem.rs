pub trait MemoryModel {
    fn load_mem(&mut self, index: u32, bytes: u8) -> Option<u32>;
    fn store_mem(&mut self, index: u32, bytes: u8, value: u32);
    fn store_mems(&mut self, index: u32, value: &[u32]) {
        for (i, item) in value.iter().enumerate() {
            self.store_mem(index + i as u32, 1, *item);
        }
    }
}
