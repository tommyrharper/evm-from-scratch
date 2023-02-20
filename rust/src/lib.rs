use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

// fn concat(a: u8, b: u8) -> U256 {
//     let hexadecimal_a_b = format!("{:X}{:X}", a, b);
//     let decimal = i64::from_str_radix(&hexadecimal_a_b, 16).unwrap();
//     return U256::from(decimal);
// }

fn concatDecimals(arr: &[u8]) -> U256 {
    let hexadecimal_concat: String = arr
        .iter()
        .map(|x| format!("{:X}", x))
        .collect::<Vec<String>>()
        .join("");

    let decimal = i64::from_str_radix(&hexadecimal_concat, 16).unwrap();

    return U256::from(decimal);
}

// TODO: impl Stack, impl Machine

fn push(code: &[u8], n: usize, position: usize) -> U256 {
    let start = position;
    let end = position + n;
    let bytes = &code[start..end];
    return concatDecimals(bytes);
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
                stack.push(push(code, 1, pc));
            }
            0x61 => {
                // PUSH2
                stack.push(push(code, 2, pc));
                pc += 1;
            }
            0x63 => {
                // PUSH4
                stack.push(push(code, 4, pc));
                pc += 3;
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
