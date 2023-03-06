// TODO: rename to be environment data and add address to struct
pub struct Environment<'a> {
    pub caller: &'a [u8],
    pub origin: &'a [u8],
    pub gasprice: &'a [u8],
}

impl<'a> Environment<'a> {
    pub fn new(caller: &'a [u8], origin: &'a [u8], gasprice: &'a [u8]) -> Self {
        Self {
            caller,
            origin,
            gasprice,
        }
    }
}
