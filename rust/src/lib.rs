use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

fn concat(a: u8, b: u8) -> U256 {
    let mut owned_string: String = a.to_string();
    let borrowed_string: &str = &b.to_string();

    owned_string.push_str(borrowed_string);
    let my_int: u128 = owned_string.parse().unwrap();

    println!("------------ {}", my_int);
    // return my_int;
    return U256::from(my_int);
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
                let mut a = format!("{:X}", code[pc]);
                let b = format!("{:X}", code[pc + 1]);
                a.push_str(&b);

                let z = i64::from_str_radix(&a, 16).unwrap();
                stack.push(U256::from(z));
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
