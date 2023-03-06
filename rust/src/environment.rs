use std::collections::HashMap;

pub struct Account<'a> {
    balance: &'a [u8],
}

impl<'a> Account<'a> {
    pub fn new(balance: &'a [u8]) -> Self {
        Self { balance }
    }
}

pub struct State<'a>(HashMap<&'a [u8], Account<'a>>);

impl<'a> State<'a> {
    pub fn new() -> Self {
        let map = HashMap::<&'a [u8], Account>::new();
        Self(map)
    }

    pub fn add_accounts(&mut self, address_balances: &Vec<(&'a [u8], &'a [u8])>) {
        for (address, balance) in address_balances {
            self.add_account(address, balance);
        }
    }

    pub fn add_account(&mut self, address: &'a [u8], balance: &'a [u8]) {
        self.0.insert(address, Account::new(balance));
    }
}

pub struct Environment<'a> {
    pub address: &'a [u8],
    pub caller: &'a [u8],
    pub origin: &'a [u8],
    pub gasprice: &'a [u8],
    pub state: State<'a>,
}

impl<'a> Environment<'a> {
    pub fn new(
        address: &'a [u8],
        caller: &'a [u8],
        origin: &'a [u8],
        gasprice: &'a [u8],
        state: State<'a>,
    ) -> Self {
        Self {
            address,
            caller,
            origin,
            gasprice,
            state,
        }
    }
}
