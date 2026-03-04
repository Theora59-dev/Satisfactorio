use std::{collections::HashMap, fmt::Error};

use crate::player::Player;

pub const CHUNK_SIZE: i32 = 32;
pub const CHUNK_SIZE_SQR: i32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_BLOCK_NUMBER: usize = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize;
pub const LAST_CHUNK_AXIS_INDEX: i32 = CHUNK_SIZE - 1;
pub const PADDED_CHUNK_SIZE: i32 = CHUNK_SIZE + 2;
pub const PADDED_CHUNK_SIZE_DOUBLE: usize = (PADDED_CHUNK_SIZE * 2) as usize;
pub const PADDED_CHUNK_SIZE_SQR: i32 = PADDED_CHUNK_SIZE * PADDED_CHUNK_SIZE;
pub const PADDED_CHUNK_BLOCK_NUMBER: usize = (PADDED_CHUNK_SIZE * PADDED_CHUNK_SIZE * PADDED_CHUNK_SIZE) as usize;
pub const FIRST_PADDED_CHUNK_CENTER_INDEX: i32 = 1;
pub const LAST_PADDED_CHUNK_CENTER_INDEX: i32 = PADDED_CHUNK_SIZE - 2;
pub const FIRST_PADDED_CHUNK_AXIS_INDEX: i32 = 0;
pub const LAST_PADDED_CHUNK_AXIS_INDEX: i32 = PADDED_CHUNK_SIZE - 1;

#[derive(Clone, Copy)]
pub struct Block {
    pub id: i32
}

pub struct Chunk {
    blocks: [Block; CHUNK_BLOCK_NUMBER],
    x: i32,
    y: i32,
    z: i32
}

pub struct PaddedChunk {
    blocks: [Block; PADDED_CHUNK_BLOCK_NUMBER],
}

pub struct World {
    chunks: HashMap<(i32, i32, i32), Chunk>,
}

impl Block {
    pub fn new(id: i32) -> Block {
        return Block {
            id: id
        };
    }

    pub fn air() -> Block {
        return Block {
            id: 0,
        };
    }

    pub fn is_air(&self) -> bool {
        return self.id == Block::air().id;
    }
}

impl Chunk {
    pub fn generate(cx: i32, cy: i32, cz: i32) -> Chunk {
        let mut chunk = Chunk { blocks: [Block::air(); CHUNK_BLOCK_NUMBER], x: cx, y: cy, z: cz };
        let block = Block::new(1);

        // On génère pour l'instant un flat sur la couche y: 0 avec un id bidon pour qu'on ne le mélange pas à l'air et qu'on crée des blocs solides et visibles
        // Pour l'instant on s'en fout des cos du chunk, ça servira plus tard pour des trucs style biomes, température, etc

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                // Todo: génération aléatoire
                chunk.set_block_from_xyz(x, 0, z, block.clone());
            }
        }
        
        return chunk;
    }

    /// Abstraction of `get_block_from_i` but with components.
    /// 
    /// Prefer using `get_block_from_i` whenever possible, as it saves computing power and time.
    #[inline(always)]
    pub fn get_block_from_xyz(&self, x: i32, y: i32, z: i32) -> Block {
        return self.get_block_from_i((x + y * CHUNK_SIZE + z * CHUNK_SIZE_SQR) as usize);
    }
    
    #[inline(always)]
    pub fn get_block_from_i(&self, i: usize) -> Block {
        return self.blocks[i];
    }

    /// Abstraction of `set_block_from_i` but with components.
    /// 
    /// Prefer using `set_block_from_i` whenever possible, as it saves computing power and time. 
    #[inline(always)]
    pub fn set_block_from_xyz(&mut self, x: i32, y: i32, z: i32, block: Block) {
        self.set_block_from_i((x + y * CHUNK_SIZE + z * CHUNK_SIZE_SQR) as usize, block);
    }

    #[inline(always)]
    pub fn set_block_from_i(&mut self, i: usize, block: Block) {
        self.blocks[i] = block;
    }
}

impl PaddedChunk {
    pub fn empty() -> PaddedChunk {
        return PaddedChunk {
            blocks: [Block::air(); PADDED_CHUNK_BLOCK_NUMBER],
        }
    }

    pub fn new(chunk: &Chunk, world: &World) -> PaddedChunk {
        let mut padded_chunk = PaddedChunk::empty();

        let mut src_i = 0usize;
        let mut dst_i = (1 + PADDED_CHUNK_SIZE + PADDED_CHUNK_SIZE_SQR) as usize; // (1,1,1)

        for _z in 0..CHUNK_SIZE {
            for _y in 0..CHUNK_SIZE {
                for _x in 0..CHUNK_SIZE {

                    padded_chunk.blocks[dst_i] = chunk.get_block_from_i(src_i);

                    src_i += 1;
                    dst_i += 1;
                }

                // fin ligne X → sauter bordure droite + gauche
                dst_i += 2;
            }

            // fin plan Y → sauter 2 lignes complètes (haut/bas)
            dst_i += PADDED_CHUNK_SIZE_DOUBLE;
        }

        padded_chunk.fill_edges(
            world.get_chunk(chunk.x-1, chunk.y, chunk.z),
            world.get_chunk(chunk.x+1, chunk.y, chunk.z),
            world.get_chunk(chunk.x, chunk.y-1, chunk.z),
            world.get_chunk(chunk.x, chunk.y+1, chunk.z),
            world.get_chunk(chunk.x, chunk.y, chunk.z-1),
            world.get_chunk(chunk.x, chunk.y, chunk.z+1),
        );

        return padded_chunk;
    }

    /// Abstraction of `get_block_from_i` but with components.
    /// 
    /// Prefer using `get_block_from_i` whenever possible, as it saves computing power and time.
    #[inline(always)]
    pub fn get_block_from_xyz(&self, x: i32, y: i32, z: i32) -> Block {
        return self.get_block_from_i((x + y * PADDED_CHUNK_SIZE + z * PADDED_CHUNK_SIZE_SQR) as usize);
    }
    
    #[inline(always)]
    pub fn get_block_from_i(&self, i: usize) -> Block {
        return self.blocks[i];
    }

    /// Abstraction of `set_block_from_i` but with components.
    /// 
    /// Prefer using `set_block_from_i` whenever possible, as it saves computing power and time. 
    #[inline(always)]
    fn set_block_from_xyz(&mut self, x: i32, y: i32, z: i32, block: Block) {
        self.set_block_from_i((x + y * PADDED_CHUNK_SIZE + z * PADDED_CHUNK_SIZE_SQR) as usize, block);
    }

    #[inline(always)]
    fn set_block_from_i(&mut self, i: usize, block: Block) {
        self.blocks[i] = block;
    }

    pub fn fill_neg_x(&mut self, chunk: &Chunk) {
        for y in FIRST_PADDED_CHUNK_CENTER_INDEX..=LAST_PADDED_CHUNK_CENTER_INDEX {
            for z in FIRST_PADDED_CHUNK_CENTER_INDEX..=LAST_PADDED_CHUNK_CENTER_INDEX {
                self.set_block_from_xyz(0, y, z, chunk.get_block_from_xyz(LAST_CHUNK_AXIS_INDEX, y-1, z-1));
            }
        }
    }

    pub fn fill_pos_x(&mut self, chunk: &Chunk) {
        for y in FIRST_PADDED_CHUNK_CENTER_INDEX..=LAST_PADDED_CHUNK_CENTER_INDEX {
            for z in FIRST_PADDED_CHUNK_CENTER_INDEX..=LAST_PADDED_CHUNK_CENTER_INDEX {
                self.set_block_from_xyz(LAST_PADDED_CHUNK_AXIS_INDEX, y, z, chunk.get_block_from_xyz(0, y-1, z-1));
            }
        }
    }

    pub fn fill_neg_y(&mut self, chunk: &Chunk) {
        for x in FIRST_PADDED_CHUNK_CENTER_INDEX..=LAST_PADDED_CHUNK_CENTER_INDEX {
            for z in FIRST_PADDED_CHUNK_CENTER_INDEX..=LAST_PADDED_CHUNK_CENTER_INDEX {
                self.set_block_from_xyz(x, 0, z, chunk.get_block_from_xyz(x-1, LAST_CHUNK_AXIS_INDEX, z-1));
            }
        }
    }

    pub fn fill_pos_y(&mut self, chunk: &Chunk) {
        for x in FIRST_PADDED_CHUNK_CENTER_INDEX..=LAST_PADDED_CHUNK_CENTER_INDEX {
            for z in FIRST_PADDED_CHUNK_CENTER_INDEX..=LAST_PADDED_CHUNK_CENTER_INDEX {
                self.set_block_from_xyz(x, LAST_PADDED_CHUNK_AXIS_INDEX, z, chunk.get_block_from_xyz(x-1, 0, z-1));
            }
        }
    }

    pub fn fill_neg_z(&mut self, chunk: &Chunk) {
        for x in FIRST_PADDED_CHUNK_CENTER_INDEX..=LAST_PADDED_CHUNK_CENTER_INDEX {
            for y in FIRST_PADDED_CHUNK_CENTER_INDEX..=LAST_PADDED_CHUNK_CENTER_INDEX {
                self.set_block_from_xyz(x, y, 0, chunk.get_block_from_xyz(x-1, y-1, LAST_CHUNK_AXIS_INDEX));
            }
        }
    }

    pub fn fill_pos_z(&mut self, chunk: &Chunk) {
        for x in FIRST_PADDED_CHUNK_CENTER_INDEX..=LAST_PADDED_CHUNK_CENTER_INDEX {
            for y in FIRST_PADDED_CHUNK_CENTER_INDEX..=LAST_PADDED_CHUNK_CENTER_INDEX {
                self.set_block_from_xyz(x, y, LAST_PADDED_CHUNK_AXIS_INDEX, chunk.get_block_from_xyz(x-1, y-1, 0));
            }
        }
    }

    pub fn fill_edges(&mut self, neg_x: Option<&Chunk>, pos_x: Option<&Chunk>, neg_y: Option<&Chunk>, pos_y: Option<&Chunk>, neg_z: Option<&Chunk>, pos_z: Option<&Chunk>) {
        if let Some(neg_x) = neg_x {
            self.fill_neg_x(neg_x);
        };
        if let Some(pos_x) = pos_x {
            self.fill_pos_x(pos_x);
        };
        if let Some(neg_y) = neg_y {
            self.fill_neg_y(neg_y);
        };
        if let Some(pos_y) = pos_y {
            self.fill_pos_y(pos_y);
        };
        if let Some(neg_z) = neg_z {
            self.fill_neg_z(neg_z);
        };
        if let Some(pos_z) = pos_z {
            self.fill_pos_z(pos_z);
        };
    }
}

impl World {
    pub fn new() -> World {
        return World {
            chunks: HashMap::new()
        };
    }

    #[inline(always)]
    pub fn get_chunk(&self, cx: i32, cy: i32, cz: i32) -> Option<&Chunk> {
        return self.chunks.get(&(cx, cy, cz));
    }

    #[inline(always)]
    pub fn set_chunk(&mut self, cx: i32, cy: i32, cz: i32, chunk: Chunk) {
        self.chunks.insert((cx, cy, cz), chunk);
    }

    pub fn get_player_rendered_chunks(&self, player: &Player) -> Vec<(&Chunk, i32, i32, i32)> {
        let [min_cx, max_cx, min_cy, max_cy, min_cz, max_cz] = player.get_rendered_chunk_range();
        let chunk_number = player.get_rendered_chunk_number();

        let mut chunks: Vec<(&Chunk, i32, i32, i32)> = Vec::new();
        chunks.reserve_exact(chunk_number as usize);

        for x in min_cx..=max_cx {
            for y in min_cy..=max_cy {
                for z in min_cz..=max_cz {
                    if let Some(chunk) = self.get_chunk(x, y, z) {
                        chunks.push((chunk, x, y, z));
                    }
                }
            }
        }

        return chunks;
    }

    pub fn get_block_from_xyz(&self, x: i32, y: i32, z: i32) -> Block {
        // Chunk coordinates
        let cx: i32 = x.div_euclid(CHUNK_SIZE);
        let cy: i32 = y.div_euclid(CHUNK_SIZE);
        let cz: i32 = z.div_euclid(CHUNK_SIZE);

        // Chunk block coordinates
        let cbx: i32 = x.rem_euclid(CHUNK_SIZE);
        let cby: i32 = y.rem_euclid(CHUNK_SIZE);
        let cbz: i32 = z.rem_euclid(CHUNK_SIZE);

        if let Some(chunk) = self.get_chunk(cx, cy, cz) {
            return chunk.get_block_from_xyz(cbx, cby, cbz);
        }
        else {
            // If the chunk does not exist / is not found, return air (useful for rendering purpose mainly)
            return Block::air();
        }
    }

    pub fn get_local_block_from_xyz(&self, lx: i32, ly: i32, lz: i32, cx: i32, cy: i32, cz: i32) -> Block {
        if !(0..CHUNK_SIZE).contains(&lx)
            || !(0..CHUNK_SIZE).contains(&ly)
            || !(0..CHUNK_SIZE).contains(&lz) {
            return self.get_block_from_xyz(
                lx + cx * CHUNK_SIZE,
                ly + cy * CHUNK_SIZE,
                lz + cz * CHUNK_SIZE
            );   
        }
        
        if let Some(chunk) = self.get_chunk(cx, cy, cz) {
            return chunk.get_block_from_xyz(lx, ly, lz);
        }
        else {
            return Block::air();
        }
    }
}