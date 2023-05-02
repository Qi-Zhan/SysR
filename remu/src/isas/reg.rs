use std::ops::{Index, IndexMut};



pub trait RegisterModel: Index<u32, Output = u32> + IndexMut<u32> {
    // fn read_register(&self, index: u32) -> Option<u32>;
    fn read_register_by_name(&self, name: &str) -> Option<u32>;
    
    // fn write_register(&mut self, index: u32, value: u32);
    fn write_register_by_name(&mut self, name: &str, value: u32);
    fn name_to_index(&self, name: &str) -> Option<u32>;
    /// a iterator of all register names and values
    fn iter(&self) -> Box<dyn Iterator<Item = (String, u32)>>;

    fn read_register_previlege(&self, index: u32) -> Option<u32>;
    fn write_register_previlege(&mut self, index: u32, value: u32);
    
}

