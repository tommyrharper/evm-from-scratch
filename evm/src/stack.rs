use crate::machine::EvmError;
use primitive_types::U256;

pub struct Stack {
    data: Vec<U256>,
}

impl Stack {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, value: U256) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<U256> {
        self.data.pop()
    }

    pub fn set(&mut self, value: U256, i: usize) -> Result<(), EvmError> {
        let len = self.data.len();
        if len > i {
            self.data[len - i - 1] = value;
            Ok(())
        } else {
            Err(EvmError::StackUnderflow)
        }
    }
    pub fn peek(&self, i: usize) -> Result<U256, EvmError> {
        if self.data.len() > i {
            Ok(self.data[self.data.len() - i - 1])
        } else {
            Err(EvmError::StackUnderflow)
        }
    }

    pub fn data(&self) -> Vec<U256> {
        self.data.to_vec().into_iter().rev().collect()
    }
}
