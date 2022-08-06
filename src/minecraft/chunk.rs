use crate::{Vertex, drawable::Drawable};

use super::block::{Block, BlockType};

#[derive(Debug, Clone)]
pub struct Chunk {
    position: (i32, i32),
    blocks: Vec<Vec<Vec<Block>>>,
    vertices: Vec<Vertex>,
    drawable: Drawable
}

const DRAW_CHUNK_SIDES: bool = false;
impl Chunk {
    pub fn new() -> Chunk {
        let mut blocks = vec![vec![vec![Block::new(super::block::BlockType::Air, [0.9, 0.9]); 20]; 256]; 20];

        for x in 0..20 {
            for y in 0..256 {
                for z in 0..20 {
                    if y <= 9 {
                        blocks[x][y][z] = Block::new(super::block::BlockType::Solid, [0.0, 0.0]).with_side_uv([0.1, 0.0]).with_bottom_uv([0.2, 0.0]);
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
        self.blocks[4][12][6] = Block::new(super::block::BlockType::Solid, [0.0, 0.0]).with_side_uv([0.1, 0.0]).with_bottom_uv([0.2, 0.0]);
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
                                        position: [crnt_position.0, y as f32 + 1.0, crnt_position.1],
                                        normal: [0.0, 1.0, 0.0],
                                        uv: block.top_uv
                                    },
                                    Vertex {
                                        position: [crnt_position.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                        normal: [0.0, 1.0, 0.0],
                                        uv: [block.top_uv[0], block.top_uv[1] + 0.1]
                                    },
                                    Vertex {
                                        position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                        normal: [0.0, 1.0, 0.0],
                                        uv: [block.top_uv[0] + 0.1, block.top_uv[1] + 0.1]
                                    },

                                    Vertex {
                                        position: [crnt_position.0, y as f32 + 1.0, crnt_position.1],
                                        normal: [0.0, 1.0, 0.0],
                                        uv: block.top_uv
                                    },
                                    Vertex {
                                        position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                        normal: [0.0, 1.0, 0.0],
                                        uv: [block.top_uv[0] + 0.1, block.top_uv[1] + 0.1]
                                    },
                                    Vertex {
                                        position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1],
                                        normal: [0.0, 1.0, 0.0],
                                        uv: [block.top_uv[0] + 0.1, block.top_uv[1]]
                                    }
                                ]);
                            }

                            else {
                                if block_above.unwrap().block_type == BlockType::Air {
                                    self.vertices.append(&mut vec![
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [0.0, 1.0, 0.0],
                                            uv: block.top_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [0.0, 1.0, 0.0],
                                            uv: [block.top_uv[0], block.top_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [0.0, 1.0, 0.0],
                                            uv: [block.top_uv[0] + 0.1, block.top_uv[1] + 0.1]
                                        },

                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [0.0, 1.0, 0.0],
                                            uv: block.top_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [0.0, 1.0, 0.0],
                                            uv: [block.top_uv[0] + 0.1, block.top_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [0.0, 1.0, 0.0],
                                            uv: [block.top_uv[0] + 0.1, block.top_uv[1]]
                                        }
                                    ]);
                                }
                            }
                        }
                        {   //BOTTOM FACE
                            let block = &self.blocks[x][y][z];
                            let block_below: Option<&Block> = {
                                if y as i32 - 1 < 0 {
                                    None
                                }
                                else {
                                    Some(&self.blocks[x][y - 1][z])
                                }
                            };

                            let crnt_position: (f32, f32) = ((self.position.0 + x as i32) as f32, (self.position.1 + z as i32) as f32);
                            
                            if block_below.is_none() {
                                self.vertices.append(&mut vec![
                                    Vertex {
                                        position: [crnt_position.0, y as f32, crnt_position.1],
                                        normal: [0.0, -1.0, 0.0],
                                        uv: block.bottom_uv
                                    },
                                    Vertex {
                                        position: [crnt_position.0 + 1.0, y as f32, crnt_position.1 + 1.0],
                                        normal: [0.0, -1.0, 0.0],
                                        uv: [block.bottom_uv[0] + 0.1, block.bottom_uv[1] + 0.1]
                                    },
                                    Vertex {
                                        position: [crnt_position.0, y as f32, crnt_position.1 + 1.0],
                                        normal: [0.0, -1.0, 0.0],
                                        uv: [block.bottom_uv[0], block.bottom_uv[1] + 0.1]
                                    },

                                    Vertex {
                                        position: [crnt_position.0, y as f32, crnt_position.1],
                                        normal: [0.0, -1.0, 0.0],
                                        uv: block.bottom_uv
                                    },
                                    Vertex {
                                        position: [crnt_position.0 + 1.0, y as f32, crnt_position.1],
                                        normal: [0.0, -1.0, 0.0],
                                        uv: [block.bottom_uv[0] + 0.1, block.bottom_uv[1]]
                                    },
                                    Vertex {
                                        position: [crnt_position.0 + 1.0, y as f32, crnt_position.1 + 1.0],
                                        normal: [0.0, -1.0, 0.0],
                                        uv: [block.bottom_uv[0] + 0.1, block.bottom_uv[1] + 0.1]
                                    }
                                ]);
                            }

                            else {
                                if block_below.unwrap().block_type == BlockType::Air {
                                    self.vertices.append(&mut vec![
                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1],
                                            normal: [0.0, -1.0, 0.0],
                                            uv: block.bottom_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [0.0, -1.0, 0.0],
                                            uv: [block.bottom_uv[0] + 0.1, block.bottom_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [0.0, -1.0, 0.0],
                                            uv: [block.bottom_uv[0], block.bottom_uv[1] + 0.1]
                                        },
    
                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1],
                                            normal: [0.0, -1.0, 0.0],
                                            uv: block.bottom_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1],
                                            normal: [0.0, -1.0, 0.0],
                                            uv: [block.bottom_uv[0] + 0.1, block.bottom_uv[1]]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [0.0, -1.0, 0.0],
                                            uv: [block.bottom_uv[0] + 0.1, block.bottom_uv[1] + 0.1]
                                        }
                                    ]);
                                }
                            }
                        }
                        {   //NORTH FACE
                            let block = &self.blocks[x][y][z];
                            let block_north: Option<&Block> = {
                                self.blocks.get(x).unwrap().get(y).unwrap().get(z + 1)
                            };

                            let crnt_position: (f32, f32) = ((self.position.0 + x as i32) as f32, (self.position.1 + z as i32) as f32);

                            if block_north.is_none() {
                                if DRAW_CHUNK_SIDES {
                                    self.vertices.append(&mut vec![
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [0.0, 0.0, -1.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1]]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [0.0, 0.0, -1.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [0.0, 0.0, -1.0],
                                            uv: [block.side_uv[0], block.side_uv[1] + 0.1]
                                        },

                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [0.0, 0.0, -1.0],
                                            uv: block.side_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [0.0, 0.0, -1.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1]]
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [0.0, 0.0, -1.0],
                                            uv: [block.side_uv[0], block.side_uv[1] + 0.1]
                                        }
                                    ]);
                                }
                            }

                            else {
                                if block_north.unwrap().block_type == BlockType::Air {
                                    self.vertices.append(&mut vec![
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [0.0, 0.0, -1.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1]]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [0.0, 0.0, -1.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [0.0, 0.0, -1.0],
                                            uv: [block.side_uv[0], block.side_uv[1] + 0.1]
                                        },
    
                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [0.0, 0.0, -1.0],
                                            uv: block.side_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [0.0, 0.0, -1.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1]]
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [0.0, 0.0, -1.0],
                                            uv: [block.side_uv[0], block.side_uv[1] + 0.1]
                                        }
                                    ]);
                                }
                            }
                        }
                        {   //SOUTH FACE
                            let block = &self.blocks[x][y][z];
                            let block_south: Option<&Block> = {
                                if z as i32 - 1 < 0 {
                                    None
                                }
                                else {
                                    Some(&self.blocks[x][y][z - 1])
                                }
                            };

                            let crnt_position: (f32, f32) = ((self.position.0 + x as i32) as f32, (self.position.1 + z as i32) as f32);

                            if block_south.is_none() {
                                if DRAW_CHUNK_SIDES {
                                    self.vertices.append(&mut vec![
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1],
                                            normal: [0.0, 0.0, 1.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1]]
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [0.0, 0.0, 1.0],
                                            uv: [block.side_uv[0], block.side_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [0.0, 0.0, 1.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1] + 0.1]
                                        },

                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1],
                                            normal: [0.0, 0.0, 1.0],
                                            uv: block.side_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [0.0, 0.0, 1.0],
                                            uv: [block.side_uv[0], block.side_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1],
                                            normal: [0.0, 0.0, 1.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1]]
                                        }
                                    ]);
                                }
                            }

                            else {
                                if block_south.unwrap().block_type == BlockType::Air {
                                    self.vertices.append(&mut vec![
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1],
                                            normal: [0.0, 0.0, 1.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1]]
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [0.0, 0.0, 1.0],
                                            uv: [block.side_uv[0], block.side_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [0.0, 0.0, 1.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1] + 0.1]
                                        },
    
                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1],
                                            normal: [0.0, 0.0, 1.0],
                                            uv: block.side_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [0.0, 0.0, 1.0],
                                            uv: [block.side_uv[0], block.side_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1],
                                            normal: [0.0, 0.0, 1.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1]]
                                        }
                                    ]);
                                }
                            }
                        }
                        {   //EAST FACE
                            let block = &self.blocks[x][y][z];
                            let block_east: Option<&Block> = {
                                let result = self.blocks.get(x + 1);
                                if result.is_none() {
                                    None
                                }

                                else {
                                    Some(result.unwrap().get(y).unwrap().get(z).unwrap())
                                }
                            };

                            let crnt_position: (f32, f32) = ((self.position.0 + x as i32) as f32, (self.position.1 + z as i32) as f32);

                            if block_east.is_none() {
                                if DRAW_CHUNK_SIDES {
                                    self.vertices.append(&mut vec![
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1],
                                            normal: [1.0, 0.0, 0.0],
                                            uv: block.side_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0], block.side_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1] + 0.1]
                                        },

                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1],
                                            normal: [1.0, 0.0, 0.0],
                                            uv: block.side_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1]]
                                        }
                                    ]);
                                }
                            }

                            else {
                                if block_east.unwrap().block_type == BlockType::Air {
                                    self.vertices.append(&mut vec![
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1],
                                            normal: [1.0, 0.0, 0.0],
                                            uv: block.side_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0], block.side_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1] + 0.1]
                                        },
                                    
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1],
                                            normal: [1.0, 0.0, 0.0],
                                            uv: block.side_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0 + 1.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1]]
                                        }
                                    ]);
                                }
                            }
                        }
                        {   //WEST FACE
                            let block = &self.blocks[x][y][z];
                            let block_west: Option<&Block> = {
                                if x as i32 - 1 < 0 {
                                    None
                                }
                                else {
                                    Some(&self.blocks[x - 1][y][z])
                                }
                            };

                            let crnt_position: (f32, f32) = ((self.position.0 + x as i32) as f32, (self.position.1 + z as i32) as f32);

                            if block_west.is_none() {
                                if DRAW_CHUNK_SIDES {
                                    self.vertices.append(&mut vec![
                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [-1.0, 0.0, 0.0],
                                            uv: block.side_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [-1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0], block.side_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [-1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1] + 0.1]
                                        },

                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [-1.0, 0.0, 0.0],
                                            uv: block.side_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [-1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1],
                                            normal: [-1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1]]
                                        }
                                    ]);
                                }
                            }

                            else {
                                if block_west.unwrap().block_type == BlockType::Air {
                                    self.vertices.append(&mut vec![
                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [-1.0, 0.0, 0.0],
                                            uv: block.side_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1 + 1.0],
                                            normal: [-1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0], block.side_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [-1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1] + 0.1]
                                        },
    
                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1 + 1.0],
                                            normal: [-1.0, 0.0, 0.0],
                                            uv: block.side_uv
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32 + 1.0, crnt_position.1],
                                            normal: [-1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1] + 0.1]
                                        },
                                        Vertex {
                                            position: [crnt_position.0, y as f32, crnt_position.1],
                                            normal: [-1.0, 0.0, 0.0],
                                            uv: [block.side_uv[0] + 0.1, block.side_uv[1]]
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
