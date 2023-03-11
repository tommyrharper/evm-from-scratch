use crate::consts::WORD_BYTES;
use crate::context::Context;
use crate::machine::{ControlFlow, EvmError, ExitSuccess, Log, Machine};
use crate::opcode::Opcode;
use crate::{evm, helpers::*};
use primitive_types::{U256};
use sha3::{Digest, Keccak256};

pub fn eval(machine: &mut Machine) -> ControlFlow {
    let opcode = machine.opcode();
    if machine.context.is_static && !Opcode::is_static(opcode) {
        return exit_error(EvmError::OpcodeNotStatic(opcode));
    }
    match opcode {
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
        Opcode::BYTE => byte(machine),
        Opcode::SHL => shl(machine),
        Opcode::SHR => shr(machine),
        Opcode::SAR => sar(machine),
        Opcode::KECCAK256 => keccak256(machine),
        Opcode::ADDRESS => address(machine),
        Opcode::BALANCE => balance(machine),
        Opcode::ORIGIN => origin(machine),
        Opcode::CALLER => caller(machine),
        Opcode::CALLVALUE => callvalue(machine),
        Opcode::CALLDATALOAD => calldataload(machine),
        Opcode::CALLDATASIZE => calldatasize(machine),
        Opcode::CALLDATACOPY => calldatacopy(machine),
        Opcode::CODESIZE => codesize(machine),
        Opcode::CODECOPY => codecopy(machine),
        Opcode::BLOCKHASH => blockhash(machine),
        Opcode::GASPRICE => gasprice(machine),
        Opcode::EXTCODESIZE => extcodesize(machine),
        Opcode::EXTCODECOPY => extcodecopy(machine),
        Opcode::EXTCODEHASH => extcodehash(machine),
        Opcode::RETURNDATASIZE => returndatasize(machine),
        Opcode::RETURNDATACOPY => returndatacopy(machine),
        Opcode::COINBASE => coinbase(machine),
        Opcode::TIMESTAMP => timestamp(machine),
        Opcode::NUMBER => number(machine),
        Opcode::DIFFICULTY => difficulty(machine),
        Opcode::GASLIMIT => gaslimit(machine),
        Opcode::CHAINID => chainid(machine),
        Opcode::SELFBALANCE => selfbalance(machine),
        Opcode::BASEFEE => basefee(machine),
        Opcode::POP => eval_pop(machine),
        Opcode::MLOAD => mload(machine),
        Opcode::MSTORE => mstore(machine),
        Opcode::SLOAD => sload(machine),
        Opcode::SSTORE => sstore(machine),
        Opcode::MSTORE8 => mstore8(machine),
        Opcode::JUMP => jump(machine),
        Opcode::JUMPI => jumpi(machine),
        Opcode::PC => pc(machine),
        Opcode::MSIZE => msize(machine),
        Opcode::GAS => gas(machine),
        Opcode::JUMPDEST => jumpdest(machine),
        Opcode::PUSH1..=Opcode::PUSH32 => eval_push(machine),
        Opcode::DUP1..=Opcode::DUP16 => dup(machine),
        Opcode::SWAP1..=Opcode::SWAP16 => swap(machine),
        Opcode::LOG0..=Opcode::LOG4 => log(machine),
        Opcode::CREATE => create(machine),
        Opcode::CALL => call(machine),
        Opcode::RETURN => eval_return(machine),
        Opcode::DELEGATECALL => delegatecall(machine),
        Opcode::STATICCALL => staticcall(machine),
        Opcode::REVERT => revert(machine),
        Opcode::INVALID => invalid(machine),
        Opcode::SELFDESTRUCT => selfdestruct(machine),
        _ => exit_error(EvmError::InvalidInstruction),
    }
}

// TODO: remove unwraps and handle failed stack pops
// TODO: remove unnecessary mut references for machine
// TODO: add and handle as_usize or fail
// TODO: add 1024 stack limit

fn stop(_machine: &mut Machine) -> ControlFlow {
    exit_success(ExitSuccess::Stop)
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
    let a_is_negative = is_negative(a);
    let b_is_negative = is_negative(b);

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

    let a_is_negative = is_negative(a);
    let b_is_negative = is_negative(b);

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

    let a_is_negative = is_negative(a);
    let b_is_negative = is_negative(b);

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

    let a_is_negative = is_negative(a);
    let b_is_negative = is_negative(b);

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

fn byte(machine: &mut Machine) -> ControlFlow {
    let byte_offset = machine.stack.pop().unwrap();
    let value = machine.stack.pop().unwrap();

    if byte_offset >= 32.into() {
        machine.stack.push(U256::zero());
        return ControlFlow::Continue(1);
    }

    let byte_index = U256::from(31) - byte_offset;

    let res = value.byte(byte_index.as_usize());

    machine.stack.push(res.into());

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

fn shl(machine: &mut Machine) -> ControlFlow {
    let shift = machine.stack.pop().unwrap();
    let value = machine.stack.pop().unwrap();

    let shifted = value << shift;
    machine.stack.push(shifted);

    ControlFlow::Continue(1)
}

fn shr(machine: &mut Machine) -> ControlFlow {
    let shift = machine.stack.pop().unwrap();
    let value = machine.stack.pop().unwrap();

    let shifted = value >> shift;
    machine.stack.push(shifted);

    ControlFlow::Continue(1)
}

fn sar(machine: &mut Machine) -> ControlFlow {
    // shift value is unsigned
    let shift = machine.stack.pop().unwrap();
    // value is signed
    let mut value = machine.stack.pop().unwrap();

    let value_is_negative = is_negative(value);

    if value_is_negative {
        value = convert_twos_compliment(value);
    }

    let mut shifted = value >> shift;

    if value_is_negative {
        shifted = convert_twos_compliment(shifted);
    }

    machine.stack.push(shifted);

    ControlFlow::Continue(1)
}

fn keccak256(machine: &mut Machine) -> ControlFlow {
    let offset = machine.stack.pop().unwrap();
    let size = machine.stack.pop().unwrap();

    let data_to_hash = machine.memory.get(offset.as_usize(), size.as_usize());
    let hashed_data = Keccak256::digest(data_to_hash);

    machine.stack.push(U256::from_big_endian(&hashed_data));

    ControlFlow::Continue(1)
}

fn address(machine: &mut Machine) -> ControlFlow {
    machine
        .stack
        .push(machine.context.address.to_u256());

    ControlFlow::Continue(1)
}

fn balance(machine: &mut Machine) -> ControlFlow {
    let address = machine.stack.pop().unwrap().to_h160();
    let balance = machine.context.state.get_account_balance(address);

    machine.stack.push(balance);

    ControlFlow::Continue(1)
}

fn origin(machine: &mut Machine) -> ControlFlow {
    machine
        .stack
        .push(machine.context.origin.to_u256());

    ControlFlow::Continue(1)
}

fn caller(machine: &mut Machine) -> ControlFlow {
    machine
        .stack
        .push(machine.context.caller.to_u256());

    ControlFlow::Continue(1)
}

fn callvalue(machine: &mut Machine) -> ControlFlow {
    machine.stack.push(machine.context.value);

    ControlFlow::Continue(1)
}

fn calldataload(machine: &mut Machine) -> ControlFlow {
    let byte_offset = machine.stack.pop().unwrap();

    machine.stack.push(
        machine
            .context
            .load_calldata(byte_offset.as_usize(), WORD_BYTES),
    );

    ControlFlow::Continue(1)
}

fn calldatasize(machine: &mut Machine) -> ControlFlow {
    machine.stack.push(machine.context.calldata_size());

    ControlFlow::Continue(1)
}

// TODO: move all possible .as_usize()'s to the initial values
fn calldatacopy(machine: &mut Machine) -> ControlFlow {
    let dest_offset = machine.stack.pop().unwrap();
    let offset = machine.stack.pop().unwrap();
    let size = machine.stack.pop().unwrap();

    let calldata = machine
        .context
        .load_calldata(offset.as_usize(), size.as_usize());

    machine
        .memory
        .set(dest_offset.as_usize(), calldata, size.as_usize());

    ControlFlow::Continue(1)
}

fn codesize(machine: &mut Machine) -> ControlFlow {
    machine.stack.push(machine.code.len().into());

    ControlFlow::Continue(1)
}

fn codecopy(machine: &mut Machine) -> ControlFlow {
    let dest_offset = machine.stack.pop().unwrap().as_usize();
    let offset = machine.stack.pop().unwrap().as_usize();
    let size = machine.stack.pop().unwrap().as_usize();

    let code = arr_slice_extend(machine.code, offset, size);

    machine.memory.set(dest_offset, code, size);

    ControlFlow::Continue(1)
}

fn blockhash(_machine: &mut Machine) -> ControlFlow {
    // TODO: implement blockhash properly
    ControlFlow::Continue(1)
}

fn gasprice(machine: &mut Machine) -> ControlFlow {
    // TODO: implement gas price properly
    machine
        .stack
        .push(machine.context.gasprice);

    ControlFlow::Continue(1)
}

fn extcodesize(machine: &mut Machine) -> ControlFlow {
    let address = machine.stack.pop().unwrap().to_h160();

    let code = machine.context.state.get_account_code(address);

    machine.stack.push(code.len().into());

    ControlFlow::Continue(1)
}

fn extcodecopy(machine: &mut Machine) -> ControlFlow {
    let address = machine.stack.pop().unwrap().to_h160();
    let dest_offset = machine.stack.pop().unwrap().as_usize();
    let offset = machine.stack.pop().unwrap().as_usize();
    let size = machine.stack.pop().unwrap().as_usize();

    let account_code = machine.context.state.get_account_code(address);
    let code = arr_slice_extend(&account_code[..], offset, size);

    // TODO: set vec<u8> instead of U256 => code could be longer. update in other place as well
    machine.memory.set(dest_offset, code, size);
    ControlFlow::Continue(1)
}

fn extcodehash(machine: &mut Machine) -> ControlFlow {
    let address = machine.stack.pop().unwrap().to_h160();

    let account_code = machine.context.state.get_account_code(address);
    if account_code.len() == 0 {
        machine.stack.push(0.into());
    } else {
        let hashed_code = Keccak256::digest(account_code);
        machine.stack.push(U256::from_big_endian(&hashed_code));
    }

    ControlFlow::Continue(1)
}

fn returndatasize(machine: &mut Machine) -> ControlFlow {
    machine.stack.push(machine.return_data_buffer.len().into());

    ControlFlow::Continue(1)
}

fn returndatacopy(machine: &mut Machine) -> ControlFlow {
    let dest_offset = machine.stack.pop().unwrap().as_usize();
    let offset = machine.stack.pop().unwrap().as_usize();
    let size = machine.stack.pop().unwrap().as_usize();

    let return_data = U256::from_big_endian(&machine.return_data_buffer[offset..]);

    machine.memory.set(dest_offset, return_data, size);

    ControlFlow::Continue(1)
}

fn coinbase(machine: &mut Machine) -> ControlFlow {
    machine
        .stack
        .push(U256::from_big_endian(machine.block.coinbase));

    ControlFlow::Continue(1)
}

fn timestamp(machine: &mut Machine) -> ControlFlow {
    machine
        .stack
        .push(U256::from_big_endian(machine.block.timestamp));

    ControlFlow::Continue(1)
}

fn number(machine: &mut Machine) -> ControlFlow {
    machine
        .stack
        .push(U256::from_big_endian(machine.block.number));

    ControlFlow::Continue(1)
}

fn difficulty(machine: &mut Machine) -> ControlFlow {
    machine
        .stack
        .push(U256::from_big_endian(machine.block.difficulty));

    ControlFlow::Continue(1)
}

fn gaslimit(machine: &mut Machine) -> ControlFlow {
    machine
        .stack
        .push(U256::from_big_endian(machine.block.gaslimit));

    ControlFlow::Continue(1)
}

fn chainid(machine: &mut Machine) -> ControlFlow {
    machine
        .stack
        .push(U256::from_big_endian(machine.block.chainid));

    ControlFlow::Continue(1)
}

fn selfbalance(machine: &mut Machine) -> ControlFlow {
    let address = machine.context.address;
    let balance = machine
        .context
        .state
        .get_account_balance(address);

    machine.stack.push(balance);

    ControlFlow::Continue(1)
}

fn basefee(machine: &mut Machine) -> ControlFlow {
    machine
        .stack
        .push(U256::from_big_endian(machine.block.basefee));

    ControlFlow::Continue(1)
}

fn eval_pop(machine: &mut Machine) -> ControlFlow {
    machine.stack.pop();

    ControlFlow::Continue(1)
}

fn mload(machine: &mut Machine) -> ControlFlow {
    let byte_offset = machine.stack.pop().unwrap();

    let res = machine.memory.get(byte_offset.as_usize(), WORD_BYTES);
    let res_word = U256::from_big_endian(res);

    machine.stack.push(res_word);
    ControlFlow::Continue(1)
}

fn mstore(machine: &mut Machine) -> ControlFlow {
    let byte_offset = machine.stack.pop().unwrap();
    let value = machine.stack.pop().unwrap();

    machine
        .memory
        .set(byte_offset.as_usize(), value, WORD_BYTES);

    ControlFlow::Continue(1)
}

fn mstore8(machine: &mut Machine) -> ControlFlow {
    let byte_offset = machine.stack.pop().unwrap();
    let value = machine.stack.pop().unwrap();

    machine.memory.set(byte_offset.as_usize(), value, 1);

    ControlFlow::Continue(1)
}

fn sload(machine: &mut Machine) -> ControlFlow {
    let key = machine.stack.pop().unwrap();

    let res = machine.storage.get(&key);

    match res {
        Some(result) => machine.stack.push(*result),
        None => machine.stack.push(0.into()),
    }

    ControlFlow::Continue(1)
}

fn sstore(machine: &mut Machine) -> ControlFlow {
    let key = machine.stack.pop().unwrap();
    let value = machine.stack.pop().unwrap();

    machine.storage.insert(key, value);

    ControlFlow::Continue(1)
}

fn jump(machine: &mut Machine) -> ControlFlow {
    let a = machine.stack.pop().unwrap();
    let is_valid = machine.jump_map.is_valid(a);

    if is_valid {
        ControlFlow::Jump(a.as_usize())
    } else {
        exit_error(EvmError::InvalidJump)
    }
}

fn jumpi(machine: &mut Machine) -> ControlFlow {
    let jump_to = machine.stack.pop().unwrap();
    let should_jump = machine.stack.pop().unwrap();

    if should_jump.is_zero() {
        return ControlFlow::Continue(1);
    }

    let is_valid = machine.jump_map.is_valid(jump_to);

    if is_valid {
        ControlFlow::Jump(jump_to.as_usize())
    } else {
        exit_error(EvmError::InvalidJump)
    }
}

fn pc(machine: &mut Machine) -> ControlFlow {
    machine.stack.push(machine.pc.into());

    ControlFlow::Continue(1)
}

fn msize(machine: &mut Machine) -> ControlFlow {
    let res = machine.memory.size();
    machine.stack.push(res.into());

    ControlFlow::Continue(1)
}

fn gas(machine: &mut Machine) -> ControlFlow {
    // TODO: update to calculate gas properly (update tests first)
    machine.stack.push(U256::MAX);

    ControlFlow::Continue(1)
}
fn jumpdest(_machine: &mut Machine) -> ControlFlow {
    ControlFlow::Continue(1)
}

fn eval_push(machine: &mut Machine) -> ControlFlow {
    let n = usize::from(machine.opcode() - (Opcode::PUSH1 - 1));
    let start = machine.pc + 1;
    let end = start + n;
    let bytes = &machine.code[start..end];
    let val_to_push = U256::from_big_endian(bytes);
    machine.stack.push(val_to_push);

    ControlFlow::Continue(n + 1)
}

fn dup(machine: &mut Machine) -> ControlFlow {
    let n = usize::from(machine.opcode() - Opcode::DUP1);

    let a = machine.stack.peek(n);

    match a {
        Ok(val) => machine.stack.push(val),
        Err(error) => return exit_error(error),
    }

    ControlFlow::Continue(1)
}

fn swap(machine: &mut Machine) -> ControlFlow {
    let n = usize::from(machine.opcode() - (Opcode::SWAP1 - 1));

    let a = match machine.stack.peek(0) {
        Ok(val) => val,
        Err(err) => return exit_error(err),
    };
    let b = match machine.stack.peek(n) {
        Ok(val) => val,
        Err(err) => return exit_error(err),
    };

    match machine.stack.set(a, n) {
        Ok(()) => (),
        Err(err) => return exit_error(err),
    }
    match machine.stack.set(b, 0) {
        Ok(()) => (),
        Err(err) => return exit_error(err),
    }

    ControlFlow::Continue(1)
}

fn log(machine: &mut Machine) -> ControlFlow {
    let n = usize::from(machine.opcode() - Opcode::LOG0);

    let offset = machine.stack.pop().unwrap().as_usize();
    let size = machine.stack.pop().unwrap().as_usize();

    let data = machine.memory.get(offset, size);

    let mut new_log = Log::new(machine.context.address, data);

    for _i in 0..n {
        let topic = machine.stack.pop().unwrap();
        new_log.add_topic(topic);
    }

    machine.logs.push(new_log);

    ControlFlow::Continue(1)
}

// TODO: sender_nonce => starts as 0 but increments each CREATE: todo: increment & test incrementation of this value
fn create(machine: &mut Machine) -> ControlFlow {
    let value = machine.stack.pop().unwrap();
    let offset = machine.stack.pop().unwrap().as_usize();
    let size = machine.stack.pop().unwrap().as_usize();

    let initialisation_code = machine.memory.get(offset, size);

    let address = create_address(machine.context.address, 0.into());

    let mut value_bytes: [u8; 32] = [0; 32];
    U256::to_big_endian(&value, &mut value_bytes);

    let res = evm(
        initialisation_code,
        Context::new(
            address,
            machine.context.address,
            machine.context.origin,
            machine.context.gasprice,
            // QUESTION??? should value be 0 at this point? And then just sent after?
            U256::from_big_endian(&value_bytes),
            &String::new(),
            machine.context.state.clone(),
            false,
        ),
        machine.block,
        None,
    );

    if !res.success {
        machine.stack.push(0.into());
        return ControlFlow::Continue(1);
    }

    machine.context.state = res.state;

    // code = return value of initialisation code
    match &res.return_val {
        // TODO: deal with code over 32_bytes long => update return val type to Vec<u8>
        Some(code) => {
            // TODO: what if code is more than 32 bytes????
            let mut code_bytes: [u8; 32] = [0; 32];
            U256::to_big_endian(code, &mut code_bytes);

            // This could cut off an (unlikely) initial stop opcode. Update return_val to be a Vec<u8>
            let code_vec = u256_to_vec_u8_without_padding(code);

            machine
                .context
                .state
                .add_or_update_account(address, value, code_vec);
        }
        None => {
            machine
                .context
                .state
                .add_or_update_account(address, value, Vec::new());
        }
    }

    // UPDATE STATE

    machine.stack.push(address.to_u256());

    ControlFlow::Continue(1)
}

fn call(machine: &mut Machine) -> ControlFlow {
    // TODO: handle gas
    let _gas = machine.stack.pop().unwrap();
    let address = machine.stack.pop().unwrap().to_h160();
    let value = machine.stack.pop().unwrap();
    let args_offset = machine.stack.pop().unwrap().as_usize();
    let args_size = machine.stack.pop().unwrap().as_usize();
    let ret_offset = machine.stack.pop().unwrap().as_usize();
    let ret_size = machine.stack.pop().unwrap().as_usize();

    let data = machine.memory.get(args_offset, args_size);

    let code = machine.context.state.get_account_code(address);


    // TODO: use trait for this
    let mut value_bytes: [u8; 32] = [0; 32];
    U256::to_big_endian(&value, &mut value_bytes);

    let data_string = hex::encode(data);

    let res = evm(
        code,
        Context::new(
            address,
            machine.context.address,
            machine.context.origin,
            machine.context.gasprice,
            U256::from_big_endian(&value_bytes),
            &data_string,
            machine.context.state.clone(),
            false,
        ),
        machine.block,
        None,
    );

    match &res.return_val {
        Some(value) => {
            let return_val_without_padding = u256_to_vec_u8_without_padding(value);

            machine.memory.set(ret_offset, *value, ret_size);
            machine.return_data_buffer = return_val_without_padding;
        }
        None => {
            machine.return_data_buffer = Vec::new();
        }
    };

    if res.success {
        machine.context.state = res.state;
        machine.stack.push(1.into());
    } else {
        machine.stack.push(0.into());
    }

    ControlFlow::Continue(1)
}

fn eval_return(machine: &mut Machine) -> ControlFlow {
    let offset = machine.stack.pop().unwrap().as_usize();
    let size = machine.stack.pop().unwrap().as_usize();

    let res = machine.memory.get(offset, size);

    exit_success(ExitSuccess::Return(U256::from_big_endian(res)))
}

fn delegatecall(machine: &mut Machine) -> ControlFlow {
    // TODO: handle gas
    let _gas = machine.stack.pop().unwrap();
    let address = machine.stack.pop().unwrap().to_h160();
    let args_offset = machine.stack.pop().unwrap().as_usize();
    let args_size = machine.stack.pop().unwrap().as_usize();
    let ret_offset = machine.stack.pop().unwrap().as_usize();
    let ret_size = machine.stack.pop().unwrap().as_usize();

    let data = machine.memory.get(args_offset, args_size);

    let code = machine.context.state.get_account_code(address);


    let data_string = hex::encode(data);

    let res = evm(
        code,
        Context::new(
            machine.context.address,
            machine.context.caller,
            machine.context.origin,
            machine.context.gasprice,
            machine.context.value,
            &data_string,
            machine.context.state.clone(),
            false,
        ),
        machine.block,
        Some(&mut machine.storage),
    );

    match &res.return_val {
        Some(value) => {
            let return_val_without_padding = u256_to_vec_u8_without_padding(value);

            machine.memory.set(ret_offset, *value, ret_size);
            machine.return_data_buffer = return_val_without_padding;
        }
        None => {
            machine.return_data_buffer = Vec::new();
        }
    };

    if res.success {
        machine.context.state = res.state;
        machine.stack.push(1.into());
    } else {
        machine.stack.push(0.into());
    }

    ControlFlow::Continue(1)
}

// TODO: merge call opcode shared logic into a single call function with a type enum passed in
fn staticcall(machine: &mut Machine) -> ControlFlow {
    // TODO: handle gas
    let _gas = machine.stack.pop().unwrap();
    let address = machine.stack.pop().unwrap().to_h160();
    let args_offset = machine.stack.pop().unwrap().as_usize();
    let args_size = machine.stack.pop().unwrap().as_usize();
    let ret_offset = machine.stack.pop().unwrap().as_usize();
    let ret_size = machine.stack.pop().unwrap().as_usize();

    let data = machine.memory.get(args_offset, args_size);

    let code = machine.context.state.get_account_code(address);

    let data_string = hex::encode(data);

    let res = evm(
        code,
        Context::new(
            address,
            machine.context.address,
            machine.context.origin,
            machine.context.gasprice,
            0.into(),
            &data_string,
            machine.context.state.clone(),
            true,
        ),
        machine.block,
        None,
    );

    match &res.return_val {
        Some(value) => {
            let return_val_without_padding = u256_to_vec_u8_without_padding(value);

            machine.memory.set(ret_offset, *value, ret_size);
            machine.return_data_buffer = return_val_without_padding;
        }
        None => {
            machine.return_data_buffer = Vec::new();
        }
    };

    if res.success {
        machine.context.state = res.state;
        machine.stack.push(1.into());
    } else {
        machine.stack.push(0.into());
    }

    ControlFlow::Continue(1)
}

fn revert(machine: &mut Machine) -> ControlFlow {
    let offset = machine.stack.pop().unwrap().as_usize();
    let size = machine.stack.pop().unwrap().as_usize();

    let res = machine.memory.get(offset, size);

    exit_error(EvmError::Revert(U256::from_big_endian(res)))
}

fn invalid(_machine: &mut Machine) -> ControlFlow {
    exit_error(EvmError::InvalidInstruction)
}

fn selfdestruct(machine: &mut Machine) -> ControlFlow {
    let address = machine.stack.pop().unwrap();

    let balance = machine
        .context
        .state
        .destruct_account(machine.context.address);

    machine
        .context
        .state
        .increment_balance(address.to_h160(), balance + machine.context.value);

    ControlFlow::Continue(1)
}
