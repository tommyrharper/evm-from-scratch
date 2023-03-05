use crate::eval::eval;
use crate::eval::ControlFlow;
use crate::jump_map::JumpMap;
use crate::memory::Memory;
use crate::stack::Stack;
use crate::transaction::Transaction;
use primitive_types::U256;

#[derive(Debug)]
pub enum EvmError {
    StackUnderflow,
    InvalidInstruction,
    InvalidJump,
    Unknown // TODO: update to something more meaningful
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
    pub address: &'a [u8],
    pub transaction: Transaction<'a>,
    pub pc: usize,
}

impl<'a> Machine<'a> {
    pub fn new(code: &'a [u8], address: &'a [u8], caller: &'a [u8], origin: &'a [u8]) -> Self {
        Self {
            stack: Stack::new(),
            memory: Memory::new(),
            jump_map: JumpMap::new(code),
            transaction: Transaction::new(caller, origin),
            address,
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

    fn step(&mut self) -> Result<EvmStatus, EvmError> {
        match eval(self) {
            ControlFlow::Continue(steps) => {
                self.pc += steps;
                Ok(EvmStatus::Running)
            },
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
