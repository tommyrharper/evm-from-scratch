mod opcode;
mod stack;
mod test;
mod machine;

use crate::machine::Machine;
use crate::machine::EvmResult;

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut machine: Machine = Machine::new(_code.as_ref());
    return machine.execute();
}
