use evm::environment::Environment;
use evm::evm;
use evm::helpers::{hex_decode_with_prefix, Convert};
use evm::state::State;
use evm::{block::Block, helpers::add_padding};
use primitive_types::{H160, U256};
use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};

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
    pub fn account_data(&self) -> Vec<(H160, Vec<u8>, Vec<u8>)> {
        self.0
            .iter()
            .map(|(address, account_data)| {
                (
                    H160::from_str(address).unwrap(),
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
    // asm: Option<String>,
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
    logs: Option<Vec<Log>>,
    #[serde(rename = "return")]
    ret: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Log {
    address: Option<String>,
    data: Option<String>,
    topics: Option<Vec<String>>,
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
                Some(to) => to.to_h160(),
                None => H160::zero(),
            },
            None => H160::zero(),
        };
        let caller = match &test.tx {
            Some(tx) => match &tx.from {
                Some(from) => from.to_h160(),
                None => H160::zero(),
            },
            None => H160::zero(),
        };
        let origin: H160 = match &test.tx {
            Some(tx) => match &tx.origin {
                Some(origin) => origin.to_h160(),
                None => H160::zero(),
            },
            None => H160::zero(),
        };
        let gasprice = match &test.tx {
            Some(tx) => match &tx.gasprice {
                Some(gasprice) => gasprice.to_u256(),
                None => U256::zero(),
            },
            None => U256::zero(),
        };
        let value = match &test.tx {
            Some(tx) => match &tx.value {
                Some(value) => value.to_u256(),
                None => U256::zero(),
            },
            None => U256::zero(),
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

        let account_data = match &test.state {
            Some(state) => state.account_data(),
            None => StateData::new().account_data(),
        };

        let mut state: State = State::new();

        state.add_accounts(&account_data);

        let result = evm(
            &code,
            Environment::new(
                address,
                caller,
                origin,
                gasprice,
                value,
                &data,
                state,
                false,
            ),
            Block::new(
                &coinbase,
                &timestamp,
                &number,
                &difficulty,
                &gaslimit,
                &chainid,
                &basefee,
            ),
            None,
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

        let mut logs_match = true;

        match &test.expect.logs {
            Some(logs) => {
                for (i, log) in logs.iter().enumerate() {
                    match &log.address {
                        Some(address) => {
                            if i >= result.logs.len() {
                                logs_match = false;
                            } else {
                                logs_match = address == &result.logs[i].address;
                            }
                        }
                        None => (),
                    }
                    match &log.data {
                        Some(data) => {
                            if i >= result.logs.len() {
                                logs_match = false;
                            } else {
                                logs_match = data == &result.logs[i].data
                            }
                        }
                        None => (),
                    }
                    match &log.topics {
                        Some(data) => {
                            if i >= result.logs.len() {
                                logs_match = false;
                            } else {
                                for (j, topic) in data.iter().enumerate() {
                                    if j >= result.logs[i].topics.len() {
                                        logs_match = false
                                    } else {
                                        logs_match = topic == &result.logs[i].topics[j]
                                    }
                                }
                            }
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }

        let mut return_matches = true;

        match &test.expect.ret {
            Some(ret) => {
                let expected_ret = U256::from_str_radix(ret, 16).unwrap();
                let actual_ret = result.return_val;
                match actual_ret {
                    Some(actual_ret) => {
                        return_matches = actual_ret == expected_ret;
                    }
                    None => {
                        return_matches = false;
                    }
                }
            }
            None => (),
        }

        matching = matching
            && result.success == test.expect.success
            && (test.expect.success && result.error.is_none()
                || !test.expect.success && !result.error.is_none())
            && logs_match
            && return_matches;

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm);

            println!(
                "Expected error: {:?}",
                if test.expect.success { None } else { Some(()) }
            );
            println!("Expected success: {:?}", test.expect.success);
            match &test.expect.ret {
                Some(ret) => {
                    println!("Expected return: {:?}", ret)
                }
                None => println!("Expected return: None"),
            }
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]");
            match &test.expect.logs {
                Some(logs) => {
                    println!("Expected logs: [");
                    for log in logs {
                        match &log.address {
                            Some(addr) => println!("  address: {:?},", addr),
                            None => (),
                        }
                        match &log.data {
                            Some(data) => println!("  data: {:?},", data),
                            None => (),
                        }
                        match &log.topics {
                            Some(topics) => {
                                if topics.len() > 0 {
                                    println!("  topics: [");
                                    for topic in topics {
                                        println!("    {:?}", topic);
                                    }
                                    println!("  ]");
                                } else {
                                    println!("  topics: {:?},", topics);
                                }
                            }
                            None => (),
                        }
                    }
                    println!("]\n");
                }
                None => {
                    println!("\n");
                }
            }

            println!("Actual error: {:?}", result.error);
            println!("Actual success: {:?}", result.success);
            println!("Actual return: {:?}", result.return_val);
            println!("Actual stack: [");
            for v in result.stack {
                println!("  {:#X},", v);
            }
            println!("]");
            if result.logs.len() > 0 {
                println!("Actual logs: [");
                for log in result.logs {
                    println!("  address: {:?},", log.address);
                    println!("  data: {:?},", log.data);
                    if log.topics.len() > 0 {
                        println!("  topics: [");
                        for topic in log.topics {
                            println!("    {:?}", topic);
                        }
                        println!("  ]");
                    } else {
                        println!("  topics: []")
                    }
                }
                println!("]\n");
            } else {
                println!("\n")
            }

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            panic!("Test failed");
        }
        println!("PASS");
    }
    println!("Congratulations!");
}
