pub struct Transaction<'a> {
    pub caller: &'a [u8],
    pub origin: &'a [u8],
}

impl<'a> Transaction<'a> {
    pub fn new(caller: &'a [u8], origin: &'a [u8]) -> Self {
        Self { caller, origin }
    }
}
