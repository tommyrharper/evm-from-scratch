use std::{
    cmp::max,
    ops::{Add, Div, Sub},
};

use primitive_types::U256;

pub struct Memory {
    data: Vec<u8>,
    len: U256,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len: U256::zero(),
        }
    }

    fn resize() {
        
    }

    // num_words_in_mem′≡max(num_words_in_mem,⌈(offset+32)÷32⌉)

    pub fn set(&mut self, offset: U256, value: U256) -> Result<(), ()> {
        // memory′[offset . . . (offset + 31)] ≡ value
        // num_words_in_mem′≡max(num_words_in_mem, ceil( (offset+32)÷32 ) )

        for i in 0..32 {
            let byte = value.byte(0);
            self.data[offset.as_usize() + i] = byte;
        }

        // (a + b - 1) / b

        // let thing = (offset + 32) / 32;
        // let thing = ceil_divide(offset + 32, 32.into());

        // self.len = cmp::max(3, 4);
        self.len = max(self.len, ceil_divide(offset + 32, 32.into())).into();

        Ok(())
    }
}

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

impl Int for u32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
