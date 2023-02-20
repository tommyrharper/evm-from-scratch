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

struct Machine<'a> {
    stack: Stack,
    code: &'a [u8],
    pc: usize,
}

impl<'a> Machine<'a> {
    fn new(code: &'a [u8]) -> Self {
        Self {
            stack: Stack::new(),
            code,
            pc: 0,
        }
    }

    fn stack(&self) -> &Vec<U256> {
        &self.stack.data
    }

    fn opcode(&self) -> u8 {
        self.code[self.pc]
    }

    fn step(&mut self, steps: usize) {
        self.pc += steps;
    }

    fn stackPush(&mut self, n: usize) {
        let start = self.pc;
        let end = start + n;
        let bytes = &self.code[start..end];
        let valToPush = concatDecimals(bytes);
        self.stack.push(valToPush);
        self.pc += n - 1;
    }

    fn execute(&mut self) -> EvmResult {
        while self.pc < self.code.len() {
            let opcode = self.opcode();
            self.step(1);

            match opcode {
                0x00 => {
                    // STOP
                    break;
                }
                0x60 => {
                    // PUSH1
                    self.stackPush(1);
                }
                0x61 => {
                    // PUSH2
                    self.stackPush(2);
                }
                0x63 => {
                    // PUSH4
                    self.stackPush(4);
                }
                _ => {
                    continue;
                }
            }
        }

        return EvmResult {
            stack: self.stack().to_vec(),
            success: true,
        };
    }
}

struct Stack {
    data: Vec<U256>,
}

impl Stack {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, value: U256) {
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
    return machine.execute();
}
