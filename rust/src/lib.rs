mod opcode;
mod stack;
mod eval;
mod machine;
mod helpers;
mod jump_map;
mod memory;
mod consts;

use crate::machine::Machine;
use crate::machine::EvmResult;

pub fn evm(code: impl AsRef<[u8]>, address: impl AsRef<[u8]>) -> EvmResult {
    let mut machine: Machine = Machine::new(code.as_ref(), address.as_ref());
    return machine.execute();
}
