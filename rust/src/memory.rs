use crate::{consts::WORD_BYTES, helpers::ceil_divide};
use primitive_types::U256;

pub struct Memory {
    data: Vec<u8>,
    len_words: usize,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len_words: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.len_words * WORD_BYTES
    }

    fn resize(&mut self, length: usize) {
        if length >= self.len_words * WORD_BYTES {
            self.data.resize(length, 0);
            self.len_words = ceil_divide(length, WORD_BYTES);
        }
    }

    // memory′[offset . . . (offset + 31)] ≡ value
    // num_words_in_mem′≡max(num_words_in_mem, ceil( (offset+32)÷32 ) )
    pub fn set(&mut self, byte_offset: usize, value: U256, target_size: usize) -> Result<(), ()> {
        self.resize(byte_offset + target_size);

        for i in 0..target_size {
            let byte = value.byte(target_size - 1 - i);
            self.data[byte_offset + i] = byte;
        }

        Ok(())
    }

    pub fn get(&mut self, byte_offset: usize, target_size: usize) -> &[u8] {
        let end_index = byte_offset + target_size;
        self.resize(end_index);
        &self.data[byte_offset..end_index]
    }
}
