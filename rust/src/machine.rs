use crate::block::Block;
use crate::environment::Environment;
use crate::eval::eval;
use crate::eval::ControlFlow;
use crate::jump_map::JumpMap;
use crate::memory::Memory;
use crate::stack::Stack;
use primitive_types::U256;

#[derive(Debug)]
pub enum EvmError {
    StackUnderflow,
    InvalidInstruction,
    InvalidJump,
}

enum EvmStatus {
    Running,
    Exited,
}

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
    pub error: Option<EvmError>,
}

pub struct Machine<'a> {
    pub stack: Stack,
    pub memory: Memory,
    pub jump_map: JumpMap,
    pub code: &'a [u8],
    pub environment: Environment<'a>,
    pub block: Block<'a>,
    pub pc: usize,
}

impl<'a> Machine<'a> {
    pub fn new(code: &'a [u8], environment: Environment<'a>, block: Block<'a>) -> Self {
        Self {
            stack: Stack::new(),
            memory: Memory::new(),
            jump_map: JumpMap::new(code),
            environment,
            block,
            code,
            pc: 0,
        }
    }

    pub fn code_slice(&self, offset: usize, size: usize) -> U256 {
        let mut res = vec![0; size];
        for i in 0..size {
            let code_index = i + offset;
            if code_index < self.code.len() {
                let data = self.code[code_index];
                res[i] = data;
            }
        }
        U256::from_big_endian(&res)
    }

    fn stack(&self) -> Vec<U256> {
        self.stack.data()
    }

    pub fn opcode(&self) -> u8 {
        self.code[self.pc]
    }

    fn step(&mut self) -> Result<EvmStatus, EvmError> {
        match eval(self) {
            ControlFlow::Continue(steps) => {
                self.pc += steps;
                Ok(EvmStatus::Running)
            }
            ControlFlow::Jump(position) => {
                self.pc = position;
                Ok(EvmStatus::Running)
            }
            ControlFlow::Exit => Ok(EvmStatus::Exited),
            ControlFlow::ExitError(err) => Err(err),
        }
    }

    pub fn execute(&mut self) -> EvmResult {
        while self.pc < self.code.len() {
            match self.step() {
                Ok(status) => match status {
                    EvmStatus::Running => continue,
                    EvmStatus::Exited => break,
                },
                Err(error) => {
                    return EvmResult {
                        stack: self.stack(),
                        success: false,
                        error: Some(error),
                    }
                }
            }
        }

        return EvmResult {
            stack: self.stack(),
            success: true,
            error: None,
        };
    }
}
