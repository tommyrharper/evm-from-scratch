use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut stack: Vec<U256> = Vec::new();
    let mut pc = 0;

    let code = _code.as_ref();

    while pc < code.len() {
        let opcode = code[pc];
        pc += 1;
        println!("-------pc {:?}", pc);
        println!("-------code {:?}", code);
        println!("-------opcode {:?}", opcode);
        println!("-------stack {:?}", stack);
        if opcode == 0x00 {
            // STOP
            break;
        } else if opcode == 0x60 {
            // PUSH
            // stack.push(U256::from(0x1));
            stack.push(U256::from(code[pc]));
            return EvmResult {
                stack: stack,
                success: true,
            };
        }
    }

    // TODO: Implement me

    return EvmResult {
        stack: stack,
        success: true,
    };
}
