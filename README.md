# EVM From Scratch

This is an EVM implementation built from scratch using Rust.

It does not implement gas calculations yet.

Mainly it consumes EVM bytecode and blockchain state data (account with balances and code), and executes the EVM bytecode according to the Ethereum Yellow Paper spec.

This is still a work in progress, though 98% of functionality is implemented there is still some refactoring work that needs to be done.

## Credits

Used [w1nt3r-eth/evm-from-scratch](https://github.com/w1nt3r-eth/evm-from-scratch) as a starting point and for many of the tests.

Also used [rust-blockchain/evm](https://github.com/rust-blockchain/evm) as inspiration and a helpful reference implementation.
