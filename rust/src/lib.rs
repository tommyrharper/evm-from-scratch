use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

fn concatDecimals(arr: &[u8]) -> U256 {
    let hexadecimal_concat: String = arr
        .iter()
        .map(|x| format!("{:X}", x))
        .collect::<Vec<String>>()
        .join("");

    return U256::from_str_radix(&hexadecimal_concat, 16).unwrap();
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

    fn stack(&self) -> Vec<U256> {
        self.stack.data()
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
        let val_to_push = concatDecimals(bytes);
        self.stack.push(val_to_push);
        self.pc += n;
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
                0x50 => {
                    // POP
                    self.stack.pop();
                }
                0x60..=0x7F => {
                    // PUSH1 => PUSH32
                    let push_size = opcode - 0x5F;
                    self.stackPush(usize::from(push_size));
                }
                _ => {
                    continue;
                }
            }
        }

        return EvmResult {
            stack: self.stack(),
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

    fn pop(&mut self) -> Option<U256> {
        self.data.pop()
    }

    fn data(&self) -> Vec<U256> {
        self.data.to_vec().into_iter().rev().collect()
    }
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut machine: Machine = Machine::new(_code.as_ref());
    return machine.execute();
}
