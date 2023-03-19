use crate::machine::{ControlFlow, EvmError, ExitReason, ExitSuccess};
use primitive_types::{H160, H256, U256};
use sha3::{Digest, Keccak256};
use std::ops::{Add, Div, Sub};

// TODO: follow this pattern elsewhere
// TODO: extract traits into own file
pub trait Convert {
    fn to_h160(&self) -> H160;
    fn to_u256(&self) -> U256;
}

// TODO: update to use into
impl Convert for U256 {
    fn to_h160(&self) -> H160 {
        let mut bytes: [u8; 32] = [0; 32];
        self.to_big_endian(&mut bytes);
        H160::from_slice(&remove_padding(&bytes))
    }
    fn to_u256(&self) -> U256 {
        *self
    }
}

impl Convert for H160 {
    fn to_h160(&self) -> H160 {
        *self
    }
    fn to_u256(&self) -> U256 {
        U256::from_big_endian(self.as_bytes())
    }
}

impl Convert for String {
    fn to_h160(&self) -> H160 {
        let hex_decoded = hex_decode_with_prefix(&self);
        let res = add_padding(&hex_decoded, 20);
        H160::from_slice(&res)
    }

    fn to_u256(&self) -> U256 {
        let hex_decoded = hex_decode_with_prefix(&self);
        U256::from_big_endian(&hex_decoded)
    }
}

pub fn hex_decode_with_prefix(data: &String) -> Vec<u8> {
    let slice = if data.contains('x') {
        &data[2..]
    } else {
        &data[..]
    };

    let mut res = String::new();
    if slice.len() % 2 == 1 {
        res.push('0');
    }
    res.push_str(slice);
    hex::decode(res).unwrap()
}

pub fn remove_padding(list: &[u8]) -> Vec<u8> {
    list.to_vec().into_iter().skip_while(|x| *x == 0).collect()
}

// TODO: tidy this up
pub fn u256_to_vec_u8_without_padding(value: &U256) -> Vec<u8> {
    let mut return_val_bytes: [u8; 32] = [0; 32];
    U256::to_big_endian(value, &mut return_val_bytes);
    let return_val_without_padding: Vec<u8> = remove_padding(&return_val_bytes);
    return_val_without_padding
}

pub fn create_address(caller: H160, nonce: U256) -> H160 {
    let mut stream = rlp::RlpStream::new_list(2);
    stream.append(&caller);
    stream.append(&nonce);
    H256::from_slice(Keccak256::digest(&stream.out()).as_slice()).into()
}

pub fn exit_error(err: EvmError) -> ControlFlow {
    ControlFlow::Exit(ExitReason::Error(err))
}
pub fn exit_success(success: ExitSuccess) -> ControlFlow {
    ControlFlow::Exit(ExitReason::Success(success))
}

pub fn add_padding(arr: &[u8], size: usize) -> Vec<u8> {
    if arr.len() >= size {
        return arr.to_vec();
    }
    let mut res = vec![0; size];
    let start_index = size - arr.len();
    for (arr_index, res_index) in (start_index..res.len()).into_iter().enumerate() {
        res[res_index] = arr[arr_index];
    }
    res
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
