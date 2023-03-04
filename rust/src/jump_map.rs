pub struct JumpMap {
    vals: Vec<bool>,
}

impl JumpMap {
    pub fn new(code: &[u8]) -> Self {
        Self {
            vals: vec![]
        }
    }
}