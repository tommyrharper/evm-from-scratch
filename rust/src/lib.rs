use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

// fn concatDecimals(a: u8, b: u8) -> String {
//     let mut hexa_decimal_a = format!("{:X}", a);
//     let hexaDecimalB = format!("{:X}", b);
//     hexa_decimal_a.push_str(&hexaDecimalB);
//     return hexa_decimal_a;
// }

fn concat(a: u8, b: u8) -> U256 {
    let hexadecimal_a_b = format!("{:X}{:X}", a, b);
    let decimal = i64::from_str_radix(&hexadecimal_a_b, 16).unwrap();
    return U256::from(decimal);
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut stack: Vec<U256> = Vec::new();
    let mut pc = 0;

    let code = _code.as_ref();

    while pc < code.len() {
        let opcode = code[pc];
        pc += 1;
        // println!("-------pc {:?}", pc);
        // println!("-------code {:?}", code);
        // println!("-------opcode {:?}", opcode);
        // println!("-------stack {:?}", stack);

        match opcode {
            0x00 => {
                // STOP
                break;
            }
            0x60 => {
                // PUSH1
                stack.push(U256::from(code[pc]));
            }
            0x61 => {
                // PUSH2
                stack.push(concat(code[pc], code[pc + 1]));
            }
            _ => {
                continue;
            }
        }
    }

    return EvmResult {
        stack: stack,
        success: true,
    };
}
