pub struct Block<'a> {
    pub basefee: &'a [u8],
    pub coinbase: &'a [u8],
    pub timestamp: &'a [u8],
    pub number: &'a [u8],
    pub difficulty: &'a [u8],
    pub gaslimit: &'a [u8],
    pub chainid: &'a [u8],
}

impl<'a> Block<'a> {
    pub fn new(
        coinbase: &'a [u8],
        coinbase: &'a [u8],
        timestamp: &'a [u8],
        number: &'a [u8],
        difficulty: &'a [u8],
        gaslimit: &'a [u8],
        chainid: &'a [u8],
    ) -> Self {
        Self {
            coinbase,
            coinbase,
            timestamp,
            number,
            difficulty,
            gaslimit,
            chainid,
        }
    }
}
