use primitive_types::U256;

use crate::opcode::Opcode;

pub struct JumpMap(Vec<bool>);

impl JumpMap {
    pub fn new(code: &[u8]) -> Self {
        Self(Self::generate_map(code))
    }

    fn generate_map(code: &[u8]) -> Vec<bool> {
        let mut map: Vec<bool> = vec![];
        let mut steps_to_block = 0;
        for i in 0..code.len() {
            if steps_to_block > 0 {
                steps_to_block -= 1;
            }

            let opcode = code[i];

            match opcode {
                Opcode::PUSH1..=Opcode::PUSH32 => {
                    steps_to_block = usize::from(opcode) - usize::from(Opcode::PUSH1) + 2;
                }
                _ => (),
            }

            if steps_to_block == 0 && opcode == Opcode::JUMPDEST {
                map.push(true);
            } else {
                map.push(false);
            }
        }

        map
    }

    pub fn is_valid(&self, index: U256) -> bool {
        if index < self.0.len().into() {
            self.0[index.as_usize()]
        } else {
            false
        }
    }
}
