use crate::opcode::Opcode;
use crate::Machine;
use crate::opcode::*;

pub enum ControlFlow {
    Continue(usize),
    Exit,
}

pub fn eval(machine: &mut Machine) -> ControlFlow {
    match machine.opcode() {
        Opcode::STOP => stop(machine),
        Opcode::ADD => add(machine),
        Opcode::MUL => mul(machine),
        Opcode::SUB => sub(machine),
        Opcode::DIV => div(machine),
        Opcode::SDIV => sdiv(machine),
        Opcode::MOD => modulus(machine),
        Opcode::ADDMOD => add_modulus(machine),
        Opcode::MULMOD => mul_modulus(machine),
        Opcode::EXP => exp(machine),
        Opcode::SIGNEXTEND => sign_extend(machine),
        Opcode::POP => pop_from_stack(machine),
        Opcode::PUSH1..=Opcode::PUSH32 => push_on_to_stack(machine),
        _ => ControlFlow::Continue(1),
    }
}
