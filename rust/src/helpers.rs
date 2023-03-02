use primitive_types::U256;

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
    let mut y = x;
    // We do this by first doing a bitwise negation
    y = !x;
    // Then adding one
    y += U256::one();
    y
}

pub fn is_negative(x: U256) -> bool {
    // check the first bit, if it is 1, it is negative
    // according to the rules of twos_compliment
    x.bit(255)
}
