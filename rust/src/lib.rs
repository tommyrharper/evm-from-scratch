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

struct Machine<'a> {
    stack: Stack,
    code: &'a [u8],
    pc: usize,
}

impl<'a> Machine<'a> {
    pub fn new(
        code: &'a [u8],
    ) -> Self {
        Self {
            stack: Stack::new(),
            code,
            pc: 0,
        }
    }

    pub fn stack(&self) -> &Vec<U256> {
        &self.stack.data
    }

    pub fn opcode(&self) -> u8 {
        self.code[self.pc]
    }

    pub fn step(&mut self, steps: usize) {
        self.pc += steps;
    }

    pub fn stackPush(&mut self, n: usize) {
        let start = self.pc;
        let end = start + n;
        let bytes = &self.code[start..end];
        let valToPush = concatDecimals(bytes);
        self.stack.push(valToPush);
        self.pc += n - 1;
    }
}

struct Stack {
    data: Vec<U256>
}


impl Stack {
    pub fn new() -> Self {
        Self {
            data: Vec::new()
        }
    }

	pub fn push(&mut self, value: U256) {
		self.data.push(value);
	}
}

fn push(code: &[u8], n: usize, position: usize) -> U256 {
    let start = position;
    let end = position + n;
    let bytes = &code[start..end];
    return concatDecimals(bytes);
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut machine: Machine = Machine::new(_code.as_ref());

    while machine.pc < machine.code.len() {
        let opcode = machine.opcode();
        machine.step(1);
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
                machine.stackPush(1);
            }
            0x61 => {
                // PUSH2
                machine.stackPush(2);
            }
            0x63 => {
                // PUSH4
                machine.stackPush(4);
            }
            _ => {
                continue;
            }
        }
    }

    return EvmResult {
        stack: machine.stack.data,
        success: true,
    };
}
