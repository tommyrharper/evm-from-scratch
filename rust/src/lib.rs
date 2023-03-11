pub mod block;
mod consts;
pub mod context;
mod eval;
pub mod helpers;
mod jump_map;
mod machine;
mod memory;
mod opcode;
mod stack;
pub mod state;

use std::collections::HashMap;
use primitive_types::U256;
use crate::block::Block;
use crate::context::Context;
use crate::machine::EvmResult;
use crate::machine::Machine;

// TODO: refactor to create Runtime struct with Machine within it
pub fn evm(
    code: impl AsRef<[u8]>,
    context: Context,
    block: Block,
    storage: Option<&mut HashMap<U256, U256>>,
) -> EvmResult {
    let mut new_storage = HashMap::new();
    let mut machine: Machine = match storage {
        Some(store) => Machine::new(code.as_ref(), context, block, store),
        None => Machine::new(code.as_ref(), context, block, &mut new_storage)
    };
    return machine.execute();
}
