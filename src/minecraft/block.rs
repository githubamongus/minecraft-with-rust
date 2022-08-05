#[derive(Debug, Clone, Copy)]
pub struct Block<'a> {
    pub block_type: &'a str
}

impl<'a> Block<'a> {
    pub fn new(block_type: &'a str) -> Self {
        Block {
            block_type
        }
    }
}
