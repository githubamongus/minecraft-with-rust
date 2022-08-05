use super::block::Block;

pub struct Chunk<'a> {
    position: (i32, i32),
    blocks: Box<[[[Block<'a>; 20]; 256]; 20]>
}

impl<'a> Chunk<'a> {
    pub fn new() -> Chunk<'a> {
        let blocks: Box<[[[Block; 20]; 256]; 20]> = vec![[[Block::new("air"); 20]; 256]; 20].into_boxed_slice().try_into().unwrap();

        Chunk {
            position: (0, 0),
            blocks: blocks.clone()
        }
    }

    pub fn draw(&self) {
        println!("yes");
    }
}
