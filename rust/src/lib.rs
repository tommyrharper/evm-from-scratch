mod consts;
mod eval;
mod helpers;
mod jump_map;
mod machine;
mod memory;
mod opcode;
mod stack;
pub mod transaction;
pub mod block;


use crate::transaction::Transaction;
use crate::machine::EvmResult;
use crate::machine::Machine;

pub fn evm(
    code: impl AsRef<[u8]>,
    address: impl AsRef<[u8]>,
    transaction: Transaction,
) -> EvmResult {
    let mut machine: Machine = Machine::new(
        code.as_ref(),
        address.as_ref(),
        transaction,
    );
    return machine.execute();
}
