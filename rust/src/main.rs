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
    value: Option<String>,
    data: Option<String>,
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

    // TODO: clean up mess
    pub fn account_data(
        address_balances_vecs: &Vec<(String, Vec<u8>, Vec<u8>)>,
    ) -> Vec<(&String, &[u8], &[u8])> {
        address_balances_vecs
            .iter()
            .map(|(address, balance, code)| (address, balance.as_slice(), code.as_slice()))
            .collect()
    }

    // TODO: clean up mess
    pub fn account_data_vecs(&self) -> Vec<(String, Vec<u8>, Vec<u8>)> {
        self.0
            .iter()
            .map(|(address, account_data)| {
                (
                    address[2..].to_string().to_uppercase(),
                    account_data.hex_decode_balance(),
                    account_data.hex_decode_code(),
                )
            })
            .collect()
    }
}

#[derive(Debug, Deserialize, Clone)]
struct AccountData {
    balance: Option<String>,
    code: Option<CodeState>,
}

impl AccountData {
    pub fn hex_decode_balance(&self) -> Vec<u8> {
        match &self.balance {
            Some(balance) => hex_decode_with_prefix(&balance),
            None => vec![],
        }
    }
    pub fn hex_decode_code(&self) -> Vec<u8> {
        let default = String::new();

        match &self.code {
            Some(code_state) => hex::decode(match &code_state.bin {
                Some(bin) => &bin,
                None => &default,
            })
            .unwrap(),
            None => vec![],
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct CodeState {
    asm: Option<String>,
    bin: Option<String>,
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
    let slice = if data.contains('x') {
        &data[2..]
    } else {
        &data[..]
    };

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
        let value = match &test.tx {
            Some(tx) => match &tx.value {
                Some(value) => hex_decode_with_prefix(value),
                None => vec![],
            },
            None => vec![],
        };
        let data = match &test.tx {
            Some(tx) => match &tx.data {
                Some(data) => data.clone(),
                None => String::new(),
            },
            None => String::new(),
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

        let account_data_vecs = match &test.state {
            Some(state) => state.account_data_vecs(),
            None => StateData::new().account_data_vecs(),
        };

        let account_data = StateData::account_data(&account_data_vecs);

        let mut state: State = State::new();

        state.add_accounts(&account_data);

        let result = evm(
            &code,
            Environment::new(&address, &caller, &origin, &gasprice, &value, &data, state),
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
