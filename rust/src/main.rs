use std::collections::HashMap;

use evm::block::Block;
use evm::environment::{Environment, State};
/**
 * EVM From Scratch
 * Rust template
 *
 * To work on EVM From Scratch in Rust:
 *
 * - Install Rust: https://www.rust-lang.org/tools/install
 * - Edit `rust/lib.rs`
 * - Run `cd rust && cargo run` to run the tests
 *
 * Hint: most people who were trying to learn Rust and EVM at the same
 * gave up and switched to JavaScript, Python, or Go. If you are new
 * to Rust, implement EVM in another programming language first.
 */
use evm::evm;
use primitive_types::U256;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Evmtest {
    name: String,
    hint: String,
    code: Code,
    // TODO: check if Option can be removed here
    tx: Option<Tx>,
    block: Option<BlockData>,
    state: Option<StateData>,
    expect: Expect,
}

#[derive(Debug, Deserialize)]
struct Tx {
    to: Option<String>,
    from: Option<String>,
    origin: Option<String>,
    gasprice: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BlockData {
    basefee: Option<String>,
    coinbase: Option<String>,
    timestamp: Option<String>,
    number: Option<String>,
    difficulty: Option<String>,
    gaslimit: Option<String>,
    chainid: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StateData(HashMap<String, AccountData>);

impl StateData {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get_address_balance_vecs(&self) -> Vec<(Vec<u8>, Vec<u8>)> {
        let address_balances_vecs: Vec<(Vec<u8>, Vec<u8>)> = self
            .0
            .iter()
            .map(|(address, account_data)| {
                (
                    hex_decode_with_prefix(address),
                    account_data.hex_decode_balance(),
                )
            })
            .collect();
        address_balances_vecs
    }
}

#[derive(Debug, Deserialize, Clone)]
struct AccountData {
    balance: Option<String>,
}

impl AccountData {
    pub fn hex_decode_balance(&self) -> Vec<u8> {
        match &self.balance {
            Some(balance) => hex_decode_with_prefix(&balance),
            None => hex_decode_with_prefix(&String::new()),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Code {
    asm: String,
    bin: String,
}

#[derive(Debug, Deserialize)]
struct Expect {
    stack: Option<Vec<String>>,
    success: bool,
    // #[serde(rename = "return")]
    // ret: Option<String>,
}

pub fn hex_decode_with_prefix(data: &String) -> Vec<u8> {
    let slice = &data[2..data.len()];
    let mut res = String::new();
    if slice.len() % 2 == 1 {
        res.push('0');
    }
    res.push_str(slice);
    hex::decode(res).unwrap()
}

fn main() {
    let text = std::fs::read_to_string("../evm.json").unwrap();
    let data: Vec<Evmtest> = serde_json::from_str(&text).unwrap();

    let total = data.len();

    for (index, test) in data.iter().enumerate() {
        println!("Test {} of {}: {}", index + 1, total, test.name);

        let code: Vec<u8> = hex::decode(&test.code.bin).unwrap();

        // TODO: update to use macro here
        let address = match &test.tx {
            Some(tx) => match &tx.to {
                Some(to) => hex_decode_with_prefix(to),
                None => vec![],
            },
            None => vec![],
        };
        let caller = match &test.tx {
            Some(tx) => match &tx.from {
                Some(from) => hex_decode_with_prefix(from),
                None => vec![],
            },
            None => vec![],
        };
        let origin = match &test.tx {
            Some(tx) => match &tx.origin {
                Some(origin) => hex_decode_with_prefix(origin),
                None => vec![],
            },
            None => vec![],
        };
        let gasprice = match &test.tx {
            Some(tx) => match &tx.gasprice {
                Some(gasprice) => hex_decode_with_prefix(gasprice),
                None => vec![],
            },
            None => vec![],
        };
        let basefee = match &test.block {
            Some(tx) => match &tx.basefee {
                Some(basefee) => hex_decode_with_prefix(basefee),
                None => vec![],
            },
            None => vec![],
        };
        let coinbase = match &test.block {
            Some(tx) => match &tx.coinbase {
                Some(coinbase) => hex_decode_with_prefix(coinbase),
                None => vec![],
            },
            None => vec![],
        };
        let timestamp = match &test.block {
            Some(tx) => match &tx.timestamp {
                Some(timestamp) => hex_decode_with_prefix(timestamp),
                None => vec![],
            },
            None => vec![],
        };
        let number = match &test.block {
            Some(tx) => match &tx.number {
                Some(number) => hex_decode_with_prefix(number),
                None => vec![],
            },
            None => vec![],
        };
        let difficulty = match &test.block {
            Some(tx) => match &tx.difficulty {
                Some(difficulty) => hex_decode_with_prefix(difficulty),
                None => vec![],
            },
            None => vec![],
        };
        let gaslimit = match &test.block {
            Some(tx) => match &tx.gaslimit {
                Some(gaslimit) => hex_decode_with_prefix(gaslimit),
                None => vec![],
            },
            None => vec![],
        };
        let chainid = match &test.block {
            Some(tx) => match &tx.chainid {
                Some(chainid) => hex_decode_with_prefix(chainid),
                None => vec![],
            },
            None => vec![],
        };

        let address_balances_vecs: Vec<(Vec<u8>, Vec<u8>)> = match &test.state {
            Some(state) => state.get_address_balance_vecs(),
            None => StateData::new().get_address_balance_vecs(),
        };

        let address_balances_arrays: Vec<(&[u8], &[u8])> = address_balances_vecs
            .iter()
            .map(|(address, balance)| (address.as_slice(), balance.as_slice()))
            .collect();

        let mut state: State = State::new();

        state.add_accounts(&address_balances_arrays);

        let result = evm(
            &code,
            Environment::new(&address, &caller, &origin, &gasprice, state),
            Block::new(
                &coinbase,
                &timestamp,
                &number,
                &difficulty,
                &gaslimit,
                &chainid,
                &basefee,
            ),
        );

        let mut expected_stack: Vec<U256> = Vec::new();
        if let Some(ref stacks) = test.expect.stack {
            for value in stacks {
                expected_stack.push(U256::from_str_radix(value, 16).unwrap());
            }
        }

        let mut matching = result.stack.len() == expected_stack.len();
        if matching {
            for i in 0..result.stack.len() {
                if result.stack[i] != expected_stack[i] {
                    matching = false;
                    break;
                }
            }
        }

        matching = matching
            && result.success == test.expect.success
            && (test.expect.success && result.error.is_none()
                || !test.expect.success && !result.error.is_none());

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm);

            println!(
                "Expected error: {:?}",
                if test.expect.success { None } else { Some(()) }
            );
            println!("Expected success: {:?}", test.expect.success);
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("Actual error: {:?}", result.error);
            println!("Actual success: {:?}", result.success);
            println!("Actual stack: [");
            for v in result.stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            panic!("Test failed");
        }
        println!("PASS");
    }
    println!("Congratulations!");
}
