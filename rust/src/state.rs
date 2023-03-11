use std::collections::HashMap;
use primitive_types::U256;

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
pub struct State(pub HashMap<String, Account>);

impl State {
    pub fn new() -> Self {
        let map = HashMap::<String, Account>::new();
        Self(map)
    }

    pub fn add_accounts(&mut self, address_balances: &Vec<(String, Vec<u8>, Vec<u8>)>) {
        for (address, balance, code) in address_balances {
            self.add_account(
                address.to_owned(),
                U256::from_big_endian(balance),
                code.clone(),
            );
        }
    }

    pub fn add_account(&mut self, address: String, balance: U256, code: Vec<u8>) {
        self.0.insert(address, Account::new(balance, code));
    }

    pub fn destruct_account(&mut self, address: U256) -> U256 {
        // TODO: remove double query to account balance
        let balance = self.get_account_balance(address);
        let address_string = format! {"{:X}", address};
        self.0.remove(&address_string);
        balance
    }

    pub fn add_or_update_account(&mut self, address: U256, balance: U256, code: Vec<u8>) {
        let prev_balance = self.get_account_balance(address);
        let new_balance = balance + prev_balance;
        let address_string = format! {"{:X}", address};

        self.0.insert(address_string, Account::new(new_balance, code));
    }

    pub fn get_account_code(&self, address: U256) -> Vec<u8> {
        let address_string = format! {"{:X}", address};
        let balance = self.0.get(&address_string);

        match balance {
            Some(account) => account.code.clone(),
            None => vec![],
        }
    }

    pub fn get_account_balance(&self, address: U256) -> U256 {
        let address_string = format! {"{:X}", address};
        let account = self.0.get(&address_string);

        match account {
            Some(account) => account.balance,
            None => U256::zero(),
        }
    }

    pub fn get_account(&self, address: U256) -> Option<&Account> {
        let address_string = format! {"{:X}", address};
        let account = self.0.get(&address_string);

        account
    }

    pub fn increment_balance(&mut self, address: U256, extra: U256) {
        let account = self.get_account(address);
        let address_string = format! {"{:X}", address};
        match account {
            Some(account) => {
                let new_balance = account.balance + extra;
                self.add_account(address_string, new_balance, account.code.clone())
            }
            None => {
                self.add_account(address_string, extra, Vec::new())
            }
        }
    }
}