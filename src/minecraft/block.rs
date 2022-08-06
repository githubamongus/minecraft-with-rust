use crate::{Vertex, drawable::Drawable};

#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    Air,
    GrassDirt
}

#[derive(Debug, Clone)]
pub struct Block {
    pub block_type: BlockType,
    pub top_uv: [f32; 2],
    pub side_uv: [f32; 2],
    pub bottom_uv: [f32; 2]
}

impl Block {
    pub fn new(block_type: BlockType, uv: [f32; 2]) -> Self {
        Block {
            block_type,
            top_uv: uv,
            side_uv: uv,
            bottom_uv: uv
        }
    }

    pub fn with_side_uv(mut self, side_uv: [f32; 2]) -> Self {
        self.side_uv = side_uv;
        self
    }

    pub fn with_bottom_uv(mut self, bottom_uv: [f32; 2]) -> Self {
        self.bottom_uv = bottom_uv;
        self
    }
}
