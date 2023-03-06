mod consts;
mod eval;
mod helpers;
mod jump_map;
mod machine;
mod memory;
mod opcode;
mod stack;
pub mod environment;
pub mod block;


use crate::block::Block;
use crate::environment::Environment;
use crate::machine::EvmResult;
use crate::machine::Machine;

pub fn evm(
    code: impl AsRef<[u8]>,
    address: impl AsRef<[u8]>,
    environment: Environment,
    block: Block,
) -> EvmResult {
    let mut machine: Machine = Machine::new(
        code.as_ref(),
        address.as_ref(),
        environment,
        block,
    );
    return machine.execute();
}
