use crate::{Vertex, drawable::Drawable};

use super::block::{Block, BlockType};

#[derive(Debug, Clone)]
pub struct Chunk {
    position: (i32, i32),
    blocks: Vec<Vec<Vec<Block>>>,
    vertices: Vec<Vertex>,
    drawable: Drawable
}

impl Chunk {
    pub fn new() -> Chunk {
        let mut blocks = vec![vec![vec![Block::new(super::block::BlockType::Air, [0.9, 0.9]); 20]; 256]; 20];

        for x in 0..20 {
            for y in 0..256 {
                for z in 0..20 {
                    if y <= 10 {
                        blocks[x][y][z] = Block::new(super::block::BlockType::GrassDirt, [0.0, 0.0]).with_side_uv([0.1, 0.0]).with_bottom_uv([0.2, 0.0]);
                    }
                }
            }
        }

        Chunk {
            position: (0, 0),
            blocks: blocks,
            vertices: Vec::new(),
            drawable: Drawable::new(&Vec::new(), "textures/atlas.png")
        }
    }

    pub fn check_faces(&mut self) {
        for x in 0..20 {
            for y in 0..256 {
                for z in 0..20 {
                    if self.blocks[x][y][z].block_type != BlockType::Air {
                        {   //ABOVE FACE
                            let block = &self.blocks[x][y][z];
                            let block_above: Option<&Block> = {
                                let result = self.blocks.get(x).unwrap().get(y + 1);
                                if result.is_none() {
                                    None
                                }

                                else {
                                    result.unwrap().get(z)
                                }
                            };

                            let crnt_position: (f32, f32) = ((self.position.0 + x as i32) as f32, (self.position.1 + z as i32) as f32);

                            if block_above.is_none() {
                                self.vertices.append(&mut vec![
                                    Vertex {
                                        position: [crnt_position.0, y as f32, crnt_position.1],
                                        normal: [0.0, 1.0, 0.0],
                                        uv: block.top_uv
                                    },
                                    Vertex {
                                        position: [crnt_position.0, y as f32, crnt_position.1 + 1.0],
                                        normal: [0.0, 1.0, 0.0],
                                        uv: [block.top_uv[0], block.top_uv[1] + 0.1]
                                    },
                                    Vertex {
                                        position: [crnt_position.0 + 1.0, y as f32, crnt_position.1 + 1.0],
                                        normal: [0.0, 1.0, 0.0],
                                        uv: [block.top_uv[0] + 0.1, block.top_uv[1] + 0.1]
                                    },

                                    Vertex {
                                        position: [crnt_position.0, y as f32, crnt_position.1],
                                        normal: [0.0, 1.0, 0.0],
                                        uv: block.top_uv
                                    },
                                    Vertex {
                                        position: [crnt_position.0 + 1.0, y as f32, crnt_position.1 + 1.0],
                                        normal: [0.0, 1.0, 0.0],
                                        uv: [block.top_uv[0] + 0.1, block.top_uv[1] + 0.1]
                                    },
                                    Vertex {
                                        position: [crnt_position.0 + 1.0, y as f32, crnt_position.1],
                                        normal: [0.0, 1.0, 0.0],
                                        uv: [block.top_uv[0] + 0.1, block.top_uv[1]]
                                    }
                                ]);
                            }

                            else {
                                if block_above.unwrap().block_type == BlockType::Air {
                                    self.vertices.append(&mut vec![
                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1],
                                            normal: [0.0, 1.0, 0.0],
                                            uv: block.top_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [0.0, 1.0, 0.0],
                                            uv: [block.top_uv[0], block.top_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [0.0, 1.0, 0.0],
                                            uv: [block.top_uv[0] + 0.1, block.top_uv[1] + 0.1]
                                        },

                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1],
                                            normal: [0.0, 1.0, 0.0],
                                            uv: block.top_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [0.0, 1.0, 0.0],
                                            uv: [block.top_uv[0] + 0.1, block.top_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1],
                                            normal: [0.0, 1.0, 0.0],
                                            uv: [block.top_uv[0] + 0.1, block.top_uv[1]]
                                        }
                                    ]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&mut self) {
        self.drawable.change_vertices(&self.vertices);
        self.drawable.draw();
    }
}
