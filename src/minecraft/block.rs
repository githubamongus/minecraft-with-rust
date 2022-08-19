use crate::{Vertex, drawable::Drawable};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockType {
    Air,
    Solid
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub name: String,
    pub block_type: BlockType,
    pub top_uv: [f32; 2],
    pub side_uv: [f32; 2],
    pub bottom_uv: [f32; 2]
}

impl Block {
    pub fn new(name: String, block_type: BlockType, uv: [f32; 2]) -> Self {
        Block {
            name: name,
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
