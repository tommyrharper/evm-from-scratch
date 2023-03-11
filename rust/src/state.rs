use std::collections::HashMap;
use primitive_types::{U256, H160};

#[derive(Debug, Clone)]
pub struct Account {
    pub balance: U256,
    pub code: Vec<u8>,
}

impl Account {
    pub fn new(balance: U256, code: Vec<u8>) -> Self {
        Self { balance, code }
    }
}

// TODO: Update to use BTreeMap
// TODO: update to_string to to_owned across codebase where possible
// TODO: Update String to H160
#[derive(Clone)]
pub struct State(pub HashMap<H160, Account>);

impl State {
    pub fn new() -> Self {
        let map = HashMap::<H160, Account>::new();
        Self(map)
    }

    pub fn add_accounts(&mut self, address_balances: &Vec<(H160, Vec<u8>, Vec<u8>)>) {
        for (address, balance, code) in address_balances {
            self.add_account(
                *address,
                U256::from_big_endian(balance),
                code.clone(),
            );
        }
    }

    pub fn add_account(&mut self, address: H160, balance: U256, code: Vec<u8>) {
        self.0.insert(address, Account::new(balance, code));
    }

    pub fn destruct_account(&mut self, address: H160) -> U256 {
        // TODO: remove double query to account balance
        let balance = self.get_account_balance(address);
        self.0.remove(&address);
        balance
    }

    pub fn add_or_update_account(&mut self, address: H160, balance: U256, code: Vec<u8>) {
        let prev_balance = self.get_account_balance(address);
        let new_balance = balance + prev_balance;

        self.0.insert(address, Account::new(new_balance, code));
    }

    pub fn get_account_code(&self, address: H160) -> Vec<u8> {
        let balance = self.0.get(&address);

        match balance {
            Some(account) => account.code.clone(),
            None => vec![],
        }
    }

    pub fn get_account_balance(&self, address: H160) -> U256 {
        let account = self.0.get(&address);

        match account {
            Some(account) => account.balance,
            None => U256::zero(),
        }
    }

    pub fn get_account(&self, address: H160) -> Option<&Account> {
        let account = self.0.get(&address);

        account
    }

    pub fn increment_balance(&mut self, address: H160, extra: U256) {
        let account = self.get_account(address);
        match account {
            Some(account) => {
                let new_balance = account.balance + extra;
                self.add_account(address, new_balance, account.code.clone())
            }
            None => {
                self.add_account(address, extra, Vec::new())
            }
        }
    }
}
