mod opcodes;
mod stack;

use crate::opcodes::Opcode;
use crate::stack::Stack;
use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

fn concat_decimals(arr: &[u8]) -> U256 {
    let hexadecimal_concat: String = arr
        .iter()
        .map(|x| format!("{:X}", x))
        .collect::<Vec<String>>()
        .join("");

    return U256::from_str_radix(&hexadecimal_concat, 16).unwrap();
}

struct Machine<'a> {
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

    fn push_on_to_stack(&mut self) {
        let n = usize::from(self.opcode() - 0x5F);
        let start = self.pc + 1;
        let end = start + n;
        let bytes = &self.code[start..end];
        let val_to_push = concat_decimals(bytes);
        self.stack.push(val_to_push);
        self.step(n);
    }

    fn pop_from_stack(&mut self) {
        self.stack.pop();
    }

    fn add(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let res = a.overflowing_add(b).0;
        self.stack.push(res);
    }

    fn mul(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let res = a.overflowing_mul(b).0;
        self.stack.push(res);
    }

    fn sub(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let res = a.overflowing_sub(b).0;
        self.stack.push(res);
    }

    fn div(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let res = a.checked_div(b);
        match res {
            Some(result) => self.stack.push(result),
            None => self.stack.push(0.into()),
        }
    }

    fn modulus(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let res = a.checked_rem(b);
        match res {
            Some(result) => self.stack.push(result),
            None => self.stack.push(0.into()),
        }
    }

    fn add_modulus(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let c = self.stack.pop().unwrap();
        // TODO: Update to use full_add ???
        let res = a.overflowing_add(b).0.checked_rem(c);
        match res {
            Some(result) => self.stack.push(result),
            None => self.stack.push(0.into()),
        }
    }

    fn mul_modulus(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let c = self.stack.pop().unwrap();
        let res_mul = a.full_mul(b);
        let res_modulo = res_mul.checked_rem(c.into());
        match res_modulo {
            Some(result) => self
                .stack
                .push(result.try_into().expect(
                    "c <= U256::MAX, result = res_mul % c, ∴ result <  U256::MAX, ∴ overflow impossible; qed"
                )),
            None => self.stack.push(0.into()),
        }
    }

    fn exp(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let res = a.overflowing_pow(b).0;
        self.stack.push(res);
    }

    fn sign_extend(&mut self) {
        let bytes_of_int_to_extend = self.stack.pop().unwrap();
        let int_to_extend = self.stack.pop().unwrap();

        if bytes_of_int_to_extend >= U256::from(32) {
            // int is already fully extended, EVM is max 256 bits, 32 bytes = 256 bits
            // ∴ push int_to_extend straight to stack
            self.stack.push(int_to_extend);
        } else {
            // t is the index from left to right of the first bit of the int_to_extend in a 32-byte word
            // x = bytes_of_int_to_extend
            // t = 256 - 8(x + 1)
            // rearrange t to find the index from left to right
            // s = 255 - t = 8(x + 1)
            // where s is the index from left to right of the first bit of the int_to_extend in a 32-byte word
            // `low_u32` works since bytes_of_int_to_extend < 32
            let bit_index = (8 * bytes_of_int_to_extend.low_u32() + 7) as usize;
            // find whether the bit at bit_index is 1 or 0
            let bit = int_to_extend.bit(bit_index);
            // create a mask of 0s up to bit_index and then 1s from then on
            let mask = (U256::one() << bit_index) - U256::one();
            if bit {
                // append 1s to int_to_extend
                self.stack.push(int_to_extend | !mask);
            } else {
                // append 0s to int_to_extend
                self.stack.push(int_to_extend & mask);
            }
        }
    }

    fn execute(&mut self) -> EvmResult {
        while self.pc < self.code.len() {
            match self.opcode() {
                Opcode::STOP => break,
                Opcode::ADD => self.add(),
                Opcode::MUL => self.mul(),
                Opcode::SUB => self.sub(),
                Opcode::DIV => self.div(),
                Opcode::MOD => self.modulus(),
                Opcode::ADDMOD => self.add_modulus(),
                Opcode::MULMOD => self.mul_modulus(),
                Opcode::EXP => self.exp(),
                Opcode::SIGNEXTEND => self.sign_extend(),
                Opcode::POP => self.pop_from_stack(),
                Opcode::PUSH1..=Opcode::PUSH32 => self.push_on_to_stack(),
                _ => {}
            }

            self.step(1);
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
