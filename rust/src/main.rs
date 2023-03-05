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
use evm::transaction::Transaction;
use evm::block::Block;
use primitive_types::U256;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Evmtest {
    name: String,
    hint: String,
    code: Code,
    tx: Option<Tx>,
    block_data: Option<BlockData>,
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
                Some(to) => hex::decode(to[2..to.len()].to_string()).unwrap(),
                None => vec![],
            },
            None => vec![],
        };
        let caller = match &test.tx {
            Some(tx) => match &tx.from {
                Some(from) => hex::decode(from[2..from.len()].to_string()).unwrap(),
                None => vec![],
            },
            None => vec![],
        };
        let origin = match &test.tx {
            Some(tx) => match &tx.origin {
                Some(origin) => hex::decode(origin[2..origin.len()].to_string()).unwrap(),
                None => vec![],
            },
            None => vec![],
        };
        let gasprice = match &test.tx {
            Some(tx) => match &tx.gasprice {
                Some(gasprice) => hex::decode(gasprice[2..gasprice.len()].to_string()).unwrap(),
                None => vec![],
            },
            None => vec![],
        };
        let basefee = match &test.block_data {
            Some(tx) => match &tx.basefee {
                Some(basefee) => hex::decode(basefee[2..basefee.len()].to_string()).unwrap(),
                None => vec![],
            },
            None => vec![],
        };
        let coinbase = match &test.block_data {
            Some(tx) => match &tx.coinbase {
                Some(coinbase) => hex::decode(coinbase[2..coinbase.len()].to_string()).unwrap(),
                None => vec![],
            },
            None => vec![],
        };
        let timestamp = match &test.block_data {
            Some(tx) => match &tx.timestamp {
                Some(timestamp) => hex::decode(timestamp[2..timestamp.len()].to_string()).unwrap(),
                None => vec![],
            },
            None => vec![],
        };
        let number = match &test.block_data {
            Some(tx) => match &tx.number {
                Some(number) => hex::decode(number[2..number.len()].to_string()).unwrap(),
                None => vec![],
            },
            None => vec![],
        };
        let difficulty = match &test.block_data {
            Some(tx) => match &tx.difficulty {
                Some(difficulty) => {
                    hex::decode(difficulty[2..difficulty.len()].to_string()).unwrap()
                }
                None => vec![],
            },
            None => vec![],
        };
        let gaslimit = match &test.block_data {
            Some(tx) => match &tx.gaslimit {
                Some(gaslimit) => hex::decode(gaslimit[2..gaslimit.len()].to_string()).unwrap(),
                None => vec![],
            },
            None => vec![],
        };
        let chainid = match &test.block_data {
            Some(tx) => match &tx.chainid {
                Some(chainid) => hex::decode(chainid[2..chainid.len()].to_string()).unwrap(),
                None => vec![],
            },
            None => vec![],
        };

        let result = evm(
            &code,
            &address,
            Transaction::new(&caller, &origin, &gasprice),
            Block::new(
                &basefee,
                &coinbase,
                &timestamp,
                &number,
                &difficulty,
                &gaslimit,
                &chainid,
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
