use primitive_types::{U256, H160};
use crate::state::State;

pub struct Context<'a> {
    pub address: H160,
    pub caller: H160,
    pub origin: H160,
    pub gasprice: U256,
    // TODO: update to U256, handle overlap with State.Account.balance -> maybe not
    pub value: U256,
    // TODO: update this to be call_data for clarity
    pub data: &'a String,
    pub state: State,
    pub is_static: bool,
}

// TODO: remove lifetime parameter where possible
impl<'a> Context<'a> {
    pub fn new(
        address: H160,
        caller: H160,
        origin: H160,
        gasprice: U256,
        value: U256,
        // TODO: update data to call_data and store as hex not string
        data: &'a String,
        state: State,
        is_static: bool,
    ) -> Self {
        Self {
            address,
            caller,
            origin,
            gasprice,
            value,
            data,
            state,
            is_static,
        }
    }

    pub fn calldata_size(&self) -> U256 {
        let data = hex::decode(&self.data).unwrap().len();
        data.into()
    }

    pub fn load_calldata(&self, byte_offset: usize, target_size: usize) -> U256 {
        let data = hex::decode(&self.data).unwrap();
        let mut res: Vec<u8> = vec![0; target_size];

        for i in 0..target_size {
            let data_index = i + byte_offset;
            if data_index < data.len() {
                let val = data[data_index];
                res[i] = val;
            }
        }

        U256::from_big_endian(&res)
    }
}
