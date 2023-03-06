use std::collections::HashMap;

use primitive_types::U256;

use crate::consts::WORD_BYTES;

#[derive(Debug)]
pub struct Account<'a> {
    pub balance: &'a [u8],
}

impl<'a> Account<'a> {
    pub fn new(balance: &'a [u8]) -> Self {
        Self { balance }
    }
}

pub struct State<'a>(pub HashMap<&'a String, Account<'a>>);

impl<'a> State<'a> {
    pub fn new() -> Self {
        let map = HashMap::<&String, Account>::new();
        Self(map)
    }

    pub fn add_accounts(&mut self, address_balances: &Vec<(&'a String, &'a [u8])>) {
        for (address, balance) in address_balances {
            self.add_account(address, balance);
        }
    }

    pub fn add_account(&mut self, address: &'a String, balance: &'a [u8]) {
        self.0.insert(address, Account::new(balance));
    }

    pub fn get_account_balance(&self, address: U256) -> U256 {
        let address_string = format! {"{:X}", address};
        let balance = self.0.get(&address_string);

        let balance_uint = match balance {
            Some(account) => account.balance,
            None => &[0],
        };

        U256::from_big_endian(balance_uint)
    }
}

pub struct Environment<'a> {
    pub address: &'a [u8],
    pub caller: &'a [u8],
    pub origin: &'a [u8],
    pub gasprice: &'a [u8],
    pub value: &'a [u8],
    pub data: &'a String,
    pub state: State<'a>,
}

impl<'a> Environment<'a> {
    pub fn new(
        address: &'a [u8],
        caller: &'a [u8],
        origin: &'a [u8],
        gasprice: &'a [u8],
        value: &'a [u8],
        data: &'a String,
        state: State<'a>,
    ) -> Self {
        Self {
            address,
            caller,
            origin,
            gasprice,
            value,
            data,
            state,
        }
    }

    pub fn calldata_size(&self) -> U256 {
        let data = hex::decode(&self.data).unwrap().len();
        data.into()
    }

    pub fn load_calldata(&self, byte_offset: usize) -> U256 {
        let data = hex::decode(&self.data).unwrap();
        let mut res: [u8; WORD_BYTES] = [0; WORD_BYTES];

        for i in 0..WORD_BYTES {
            let data_index = i + byte_offset;
            if data_index < data.len() {
                let val = data[data_index];
                res[i] = val;
            }
        }

        U256::from_big_endian(&res)
    }
}
