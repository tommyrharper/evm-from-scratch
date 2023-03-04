use primitive_types::U256;

use crate::opcode::Opcode;

pub struct JumpMap {
    vals: Vec<bool>,
}

impl JumpMap {
    pub fn new(code: &[u8]) -> Self {
        Self {
            vals: Self::generate_map(code),
        }
    }

    fn generate_map(code: &[u8]) -> Vec<bool> {
        let mut map: Vec<bool> = vec![];
        let mut block_count = 0;
        for i in 0..code.len() {
            if block_count > 0 {
                block_count -= 1;
            }

            let opcode = code[i];
            // TODO: optimize to use map instead of list
            if (Opcode::PUSH1..=Opcode::PUSH32).contains(&opcode) {
                block_count = usize::from(opcode) - usize::from(Opcode::PUSH1) + 2
            }

            if block_count == 0 && opcode == Opcode::JUMPDEST {
                map.push(true);
            } else {
                map.push(false);
            }
        }

        map
    }

    pub fn is_valid(&self, index: U256) -> bool {
        if index < self.vals.len().into() {
            self.vals[index.as_usize()]
        } else {
            false
        }
    }
}
