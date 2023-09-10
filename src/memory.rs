
pub type Memory = Vec<u8>;

pub fn create_memory(memory_size: usize) -> Memory {
    return vec![0; memory_size];
}
