// TODO: rename to be environment data and add address to struct
pub struct Environment<'a> {
    pub address: &'a [u8],
    pub caller: &'a [u8],
    pub origin: &'a [u8],
    pub gasprice: &'a [u8],
}

impl<'a> Environment<'a> {
    pub fn new(address: &'a [u8], caller: &'a [u8], origin: &'a [u8], gasprice: &'a [u8]) -> Self {
        Self {
            address,
            caller,
            origin,
            gasprice,
        }
    }
}
