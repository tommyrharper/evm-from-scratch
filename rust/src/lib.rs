mod consts;
mod eval;
mod helpers;
mod jump_map;
mod machine;
mod memory;
mod opcode;
mod stack;
mod transaction;

use crate::machine::EvmResult;
use crate::machine::Machine;

pub fn evm(
    code: impl AsRef<[u8]>,
    address: impl AsRef<[u8]>,
    caller: impl AsRef<[u8]>,
    origin: impl AsRef<[u8]>,
    gasprice: impl AsRef<[u8]>,
) -> EvmResult {
    let mut machine: Machine = Machine::new(
        code.as_ref(),
        address.as_ref(),
        caller.as_ref(),
        origin.as_ref(),
        gasprice.as_ref(),
    );
    return machine.execute();
}
