use crate::stack::Stack;
use crate::test::ControlFlow;
use crate::test::eval_instruction;
use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub struct Machine<'a> {
    pub stack: Stack,
    pub code: &'a [u8],
    pub pc: usize,
}

impl<'a> Machine<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        Self {
            stack: Stack::new(),
            code,
            pc: 0,
        }
    }

    fn stack(&self) -> Vec<U256> {
        self.stack.data()
    }

    pub fn opcode(&self) -> u8 {
        self.code[self.pc]
    }

    fn step(&mut self, steps: usize) {
        self.pc += steps;
    }

    pub fn execute(&mut self) -> EvmResult {
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
