use primitive_types::U256;

// TODO: remove manual impl to use :x?
// https://stackoverflow.com/questions/27650312/show-u8-slice-in-hex-representation
pub fn concat_decimals(arr: &[u8]) -> U256 {
    let hexadecimal_concat: String = arr
        .iter()
        .map(|x| {
            if x < &16 {
                format!("0{:X}", x)
            } else {
                format!("{:X}", x)
            }
        })
        .collect::<Vec<String>>()
        .join("");

    return U256::from_str_radix(&hexadecimal_concat, 16).unwrap();
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
