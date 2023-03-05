use std::{
    cmp::max,
    ops::{Add, Div, Sub},
};

use primitive_types::U256;

pub struct Memory {
    data: Vec<u8>,
    // len: U256,
    len: usize,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len: 0,
        }
    }

    // fn resize() {}

    // num_words_in_mem′≡max(num_words_in_mem,⌈(offset+32)÷32⌉)

    pub fn set(&mut self, byte_offset: usize, value: U256) -> Result<(), ()> {
        // memory′[offset . . . (offset + 31)] ≡ value
        // num_words_in_mem′≡max(num_words_in_mem, ceil( (offset+32)÷32 ) )

        self.data.resize(byte_offset + 32, 0);

        for i in 0..32 {
            let byte = value.byte(31 - i);
            self.data[byte_offset + i] = byte;
        }

        // (a + b - 1) / b

        // self.len = max(self.len, ceil_divide(byte_offset + 32, 32.into())).into();
        self.len = max(self.len, ceil_divide(byte_offset + 32, 32)).into();

        Ok(())
    }

    pub fn get(&self, byte_offset: usize) -> Result<&[u8], ()> {
        let slice = &self.data[byte_offset..byte_offset + 32];
        Ok(slice)
    }
}

// TODO: clean up this mess 🤦‍♂️
fn ceil_divide<T: Int>(a: T, b: T) -> T {
    (a + b - T::one()) / b
}

trait Int: Add<Output = Self> + Sub<Output = Self> + Div<Output = Self> + PartialEq + Copy {
    fn zero() -> Self;
    fn one() -> Self;
}

impl Int for U256 {
    fn zero() -> Self {
        U256::zero()
    }
    fn one() -> Self {
        U256::one()
    }
}

impl Int for usize {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Int for u32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
