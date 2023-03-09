pub mod block;
mod consts;
pub mod environment;
mod eval;
mod helpers;
mod jump_map;
mod machine;
mod memory;
mod opcode;
mod stack;

use std::collections::HashMap;
use primitive_types::U256;
use crate::block::Block;
use crate::environment::Environment;
use crate::machine::EvmResult;
use crate::machine::Machine;

// TODO: refactor to create Runtime struct with Machine within it
pub fn evm(
    code: impl AsRef<[u8]>,
    environment: Environment,
    block: Block,
    storage: Option<&mut HashMap<U256, U256>>,
) -> EvmResult {
    let mut new_storage = HashMap::new();
    let mut machine: Machine = match storage {
        Some(store) => Machine::new(code.as_ref(), environment, block, store),
        None => Machine::new(code.as_ref(), environment, block, &mut new_storage)
    };
    return machine.execute();
}
