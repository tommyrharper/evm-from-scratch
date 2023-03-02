use crate::helpers::*;
use crate::machine::Machine;
use crate::opcode::Opcode;
use primitive_types::U256;

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
        Opcode::SMOD => smodulus(machine),
        Opcode::ADDMOD => add_modulus(machine),
        Opcode::MULMOD => mul_modulus(machine),
        Opcode::EXP => exp(machine),
        Opcode::SIGNEXTEND => sign_extend(machine),
        Opcode::LT => lt(machine),
        Opcode::GT => gt(machine),
        Opcode::SLT => slt(machine),
        Opcode::SGT => sgt(machine),
        Opcode::EQ => eq(machine),
        Opcode::ISZERO => iszero(machine),
        Opcode::AND => and(machine),
        Opcode::OR => or(machine),
        Opcode::XOR => xor(machine),
        Opcode::NOT => not(machine),
        Opcode::POP => pop_from_stack(machine),
        Opcode::PUSH1..=Opcode::PUSH32 => push_on_to_stack(machine),
        _ => ControlFlow::Continue(1),
    }
}

fn stop(_machine: &mut Machine) -> ControlFlow {
    ControlFlow::Exit
}

fn add(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let b = machine.stack.pop().unwrap();
    let res = a.overflowing_add(b).0;
    machine.stack.push(res);

    ControlFlow::Continue(1)
}

fn mul(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let b = machine.stack.pop().unwrap();
    let res = a.overflowing_mul(b).0;
    machine.stack.push(res);

    ControlFlow::Continue(1)
}

fn sub(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let b = machine.stack.pop().unwrap();
    let res = a.overflowing_sub(b).0;
    machine.stack.push(res);

    ControlFlow::Continue(1)
}

fn div(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let b = machine.stack.pop().unwrap();
    let res = a.checked_div(b);
    match res {
        Some(result) => machine.stack.push(result),
        None => machine.stack.push(0.into()),
    }

    ControlFlow::Continue(1)
}

fn sdiv(machine: &mut Machine) -> ControlFlow {
    let mut a = machine.stack.pop().unwrap();
    let mut b = machine.stack.pop().unwrap();

    // If the first bit is 1, then the value is negative, according to the rules of two's compliment
    let a_is_negative = a.bit(255);
    let b_is_negative = b.bit(255);

    // If the value is negative, we need to switch it into a positive value
    if a_is_negative {
        a = convert_twos_compliment(a);
    }
    // We do this for either of the numbers if they are negative, to find their absolute value
    if b_is_negative {
        b = convert_twos_compliment(b);
    }

    // now res = |a| / |b|
    let res = a.checked_div(b);

    match res {
        Some(mut result) => match result {
            // if the result is 0, push 0 straight onto stack
            i if i == 0.into() => machine.stack.push(i),
            _ => {
                // If only one of the numbers is negative, the result will be negative
                if a_is_negative ^ b_is_negative {
                    // We need to perform two's compliment again to provide a negative result
                    result = convert_twos_compliment(result);
                }
                machine.stack.push(result);
            }
        },
        None => machine.stack.push(U256::zero()),
    }

    ControlFlow::Continue(1)
}

fn modulus(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let b = machine.stack.pop().unwrap();
    let res = a.checked_rem(b);
    match res {
        Some(result) => machine.stack.push(result),
        None => machine.stack.push(0.into()),
    }

    ControlFlow::Continue(1)
}

fn smodulus(machine: &mut Machine) -> ControlFlow {
    let mut a = machine.stack.pop().unwrap();
    let mut b = machine.stack.pop().unwrap();

    let a_is_negative = a.bit(255);
    let b_is_negative = b.bit(255);

    if a_is_negative {
        a = convert_twos_compliment(a);
    }
    if b_is_negative {
        b = convert_twos_compliment(b);
    }

    let res = a.checked_rem(b);

    match res {
        Some(mut result) => match result {
            i if i == 0.into() => machine.stack.push(i),
            _ => {
                if a_is_negative {
                    result = convert_twos_compliment(result);
                }
                machine.stack.push(result);
            }
        },
        None => machine.stack.push(0.into()),
    }

    ControlFlow::Continue(1)
}

fn add_modulus(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let b = machine.stack.pop().unwrap();
    let c = machine.stack.pop().unwrap();
    let res = a.overflowing_add(b).0.checked_rem(c);
    match res {
        Some(result) => machine.stack.push(result),
        None => machine.stack.push(0.into()),
    }

    ControlFlow::Continue(1)
}

fn mul_modulus(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let b = machine.stack.pop().unwrap();
    let c = machine.stack.pop().unwrap();
    let res_mul = a.full_mul(b);
    let res_modulo = res_mul.checked_rem(c.into());
    match res_modulo {
        Some(result) => machine
            .stack
            .push(result.try_into().expect(
                "c <= U256::MAX, result = res_mul % c, ∴ result <  U256::MAX, ∴ overflow impossible; qed"
            )),
        None => machine.stack.push(0.into()),
    }

    ControlFlow::Continue(1)
}

fn exp(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let b = machine.stack.pop().unwrap();
    let res = a.overflowing_pow(b).0;
    machine.stack.push(res);

    ControlFlow::Continue(1)
}

// extend a signed integer to 32 bytes
// a = num_bytes = the number of bytes of the integer to extend - 1
// b = int_to_extend = the integer to extend
// e.g.
// a = 0, b = 00000001, int_to_extend with bytes => 00000001
// a = 1, b = 00000001, int_to_extend with bytes => 0000000000000001
// a = 1, b = 11111111, int_to_extend with bytes => 0000000011111111
// Full example:
// a = 0, b = 11111110, int_to_extend with bytes => 11111110
// bit_index = (8 * 0) + 7 = 7
// bit = 1
// mask  = 0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000011111111
// !mask = 1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111100000000
// res = int_to_extend | !mask
// = 11111110 | 1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111100000000
// = 1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111110
// = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE
fn sign_extend(machine: &mut Machine) -> ControlFlow {
    let num_bytes = machine.stack.pop().unwrap();
    let int_to_extend = machine.stack.pop().unwrap();

    if num_bytes >= U256::from(32) {
        // int is already fully extended, EVM is max 256 bits, 32 bytes = 256 bits
        // ∴ push int_to_extend straight to stack
        machine.stack.push(int_to_extend);
    } else {
        // t is the index from left to right of the first bit of the int_to_extend in a 32-byte word
        // x = num_bytes
        // t = 256 - 8(x + 1)
        // rearrange t to find the index from left to right
        // s = 255 - t = 8(x + 1)
        // where s is the index from left to right of the first bit of the int_to_extend in a 32-byte word
        // `low_u32` works since num_bytes < 32
        let bit_index = (8 * num_bytes.low_u32() + 7) as usize;
        // find whether the bit at bit_index is 1 or 0
        let bit = int_to_extend.bit(bit_index);
        // create a mask of 0s up to bit_index and then 1s from then on
        let mask = (U256::one() << bit_index) - U256::one();
        if bit {
            // append 1s to int_to_extend
            machine.stack.push(int_to_extend | !mask);
        } else {
            // append 0s to int_to_extend
            machine.stack.push(int_to_extend & mask);
        }
    }
    ControlFlow::Continue(1)
}

fn lt(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let b = machine.stack.pop().unwrap();
    let res = (a < b) as u32;
    machine.stack.push(U256::from(res));

    ControlFlow::Continue(1)
}

fn gt(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let b = machine.stack.pop().unwrap();
    let res = (a > b) as u32;
    machine.stack.push(U256::from(res));

    ControlFlow::Continue(1)
}

fn slt(machine: &mut Machine) -> ControlFlow {
    let mut a = machine.stack.pop().unwrap();
    let mut b = machine.stack.pop().unwrap();

    if a == b {
        machine.stack.push(U256::zero());
        return ControlFlow::Continue(1);
    }

    let a_is_negative = a.bit(255);
    let b_is_negative = b.bit(255);

    if a_is_negative && !b_is_negative {
        machine.stack.push(U256::one());
        return ControlFlow::Continue(1);
    } else if !a_is_negative && b_is_negative {
        machine.stack.push(U256::zero());
        return ControlFlow::Continue(1);
    }

    if a_is_negative {
        a = convert_twos_compliment(a);
    }
    if b_is_negative {
        b = convert_twos_compliment(b);
    }

    let mut res = a < b;

    if a_is_negative && b_is_negative {
        res = !res;
    }

    machine.stack.push(U256::from(res as u32));

    ControlFlow::Continue(1)
}

fn sgt(machine: &mut Machine) -> ControlFlow {
    let mut a = machine.stack.pop().unwrap();
    let mut b = machine.stack.pop().unwrap();

    if a == b {
        machine.stack.push(U256::zero());
        return ControlFlow::Continue(1);
    }

    let a_is_negative = a.bit(255);
    let b_is_negative = b.bit(255);

    if a_is_negative && !b_is_negative {
        machine.stack.push(U256::zero());
        return ControlFlow::Continue(1);
    } else if !a_is_negative && b_is_negative {
        machine.stack.push(U256::one());
        return ControlFlow::Continue(1);
    }

    if a_is_negative {
        a = convert_twos_compliment(a);
    }
    if b_is_negative {
        b = convert_twos_compliment(b);
    }

    let mut res = a > b;

    if a_is_negative && b_is_negative {
        res = !res;
    }

    machine.stack.push(U256::from(res as u32));

    ControlFlow::Continue(1)
}

fn eq(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let b = machine.stack.pop().unwrap();

    if a == b {
        machine.stack.push(U256::one());
    } else {
        machine.stack.push(U256::zero());
    }

    ControlFlow::Continue(1)
}

fn iszero(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();

    if a == U256::zero() {
        machine.stack.push(U256::one());
    } else {
        machine.stack.push(U256::zero());
    }

    ControlFlow::Continue(1)
}

fn not(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();

    machine.stack.push(!a);

    ControlFlow::Continue(1)
}

fn and(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let b = machine.stack.pop().unwrap();

    machine.stack.push(a & b);

    ControlFlow::Continue(1)
}

fn or(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let b = machine.stack.pop().unwrap();

    machine.stack.push(a | b);

    ControlFlow::Continue(1)
}

fn xor(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let b = machine.stack.pop().unwrap();

    machine.stack.push(a ^ b);

    ControlFlow::Continue(1)
}

fn pop_from_stack(machine: &mut Machine) -> ControlFlow {
    machine.stack.pop();

    ControlFlow::Continue(1)
}

fn push_on_to_stack(machine: &mut Machine) -> ControlFlow {
    let n = usize::from(machine.opcode() - 0x5F);
    let start = machine.pc + 1;
    let end = start + n;
    let bytes = &machine.code[start..end];
    let val_to_push = concat_decimals(bytes);
    machine.stack.push(val_to_push);

    ControlFlow::Continue(n + 1)
}
