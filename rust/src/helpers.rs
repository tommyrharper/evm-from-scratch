use std::ops::{Add, Div, Sub};
use primitive_types::{U256, H160, H256};
use sha3::{Keccak256, Digest};
use crate::machine::{ControlFlow, EvmError, ExitReason, ExitSuccess};

pub fn u256_to_vec_u8_without_padding(value: &U256) -> Vec<u8> {
    let mut return_val_bytes: [u8; 32] = [0; 32];
    U256::to_big_endian(value, &mut return_val_bytes);
    let return_val_without_padding: Vec<u8> = return_val_bytes
        .to_vec()
        .into_iter()
        .skip_while(|x| *x == 0)
        .collect();
    return_val_without_padding
}

pub fn create_address(caller: &[u8], nonce: U256) -> H160 {
    let caller_address_hex = H160::from_slice(caller);
	let mut stream = rlp::RlpStream::new_list(2);
	stream.append(&caller_address_hex);
	stream.append(&nonce);
	H256::from_slice(Keccak256::digest(&stream.out()).as_slice()).into()
}

pub fn exit_error(err: EvmError) -> ControlFlow {
    ControlFlow::Exit(ExitReason::Error(err))
}
pub fn exit_success(success: ExitSuccess) -> ControlFlow {
    ControlFlow::Exit(ExitReason::Success(success))
}

pub fn arr_slice_extend(arr: &[u8], offset: usize, size: usize) -> U256 {
    let mut res = vec![0; size];
    for i in 0..size {
        let code_index = i + offset;
        if code_index < arr.len() {
            let data = arr[code_index];
            res[i] = data;
        }
    }
    U256::from_big_endian(&res)
}

pub fn convert_twos_compliment(x: U256) -> U256 {
    // Note, normally the twos compliment of 0 is 0
    // However according to the EVM spec it seems to want this behaviour
    // I am uncertain if this is a misunderstanding by me/edge case for the SAR opcode
    // TODO: research this behaviour
    if x == U256::zero() {
        return !x;
    }
    // We do this by first doing a bitwise negation then adding one
    !x + U256::one()
}

pub fn is_negative(x: U256) -> bool {
    // check the first bit, if it is 1, it is negative
    // according to the rules of twos_compliment
    x.bit(255)
}

pub fn ceil_divide<T: Int>(a: T, b: T) -> T {
    (a + b - T::one()) / b
}

pub trait Int:
    Add<Output = Self> + Sub<Output = Self> + Div<Output = Self> + PartialEq + Copy
{
    fn zero() -> Self;
    fn one() -> Self;
}

impl Int for U256 {
    fn zero() -> Self {
        U256::zero()
    }
    fn one() -> Self {
        U256::one()
    }
}

impl Int for usize {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Int for u32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
