mod opcode;
mod stack;
mod test;

use crate::stack::Stack;
use crate::test::ControlFlow;
use crate::test::eval_instruction;
use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub struct Machine<'a> {
    stack: Stack,
    code: &'a [u8],
    pc: usize,
}

impl<'a> Machine<'a> {
    fn new(code: &'a [u8]) -> Self {
        Self {
            stack: Stack::new(),
            code,
            pc: 0,
        }
    }

    fn stack(&self) -> Vec<U256> {
        self.stack.data()
    }

    fn opcode(&self) -> u8 {
        self.code[self.pc]
    }

    fn step(&mut self, steps: usize) {
        self.pc += steps;
    }

    fn execute(&mut self) -> EvmResult {
        while self.pc < self.code.len() {
            match eval_instruction(self) {
                ControlFlow::Continue(n) => {
                    self.step(n);
                    continue;
                }
                ControlFlow::Exit => break,
            }
        }

        return EvmResult {
            stack: self.stack(),
            success: true,
        };
    }
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut machine: Machine = Machine::new(_code.as_ref());
    return machine.execute();
}
