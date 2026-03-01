pub const CHUNK_SIZE: i32 = 16;
pub const CHUNK_SIZE_SQR: i32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_BLOCK_NUMBER: usize = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize;

pub struct Block {
    id: i32
}

pub struct Chunk {
    blocks: [usize; CHUNK_BLOCK_NUMBER],
}

pub struct World {
    chunks: Vec<Chunk>,
}

impl Block {
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
    pub fn get_block_from_xyz(&self, x: i32, y: i32, z: i32) -> Block {
        return Block {
            id: self.blocks[(x + y * CHUNK_SIZE + z * CHUNK_SIZE_SQR) as usize] as i32,
        };
    }

    pub fn get_block_from_i(&self, i: usize) -> Block {
        return Block {
            id: self.blocks[i] as i32,
        };
    }
}

impl World {
    pub fn get_chunk(&self, cx: i32, cy: i32, cz: i32) -> Option<&Chunk> {
        // todo: remanier ça car je suis pas sûr que ça fonctionne en vrai
        return Some(&self.chunks[(cx + cy * CHUNK_SIZE + cz * CHUNK_SIZE_SQR) as usize]);
    }

    pub fn get_block_from_xyz(&self, x: i32, y: i32, z: i32) -> Block {
        let cx: i32 = x.div_euclid(CHUNK_SIZE);
        let cy: i32 = y.div_euclid(CHUNK_SIZE);
        let cz: i32 = z.div_euclid(CHUNK_SIZE);
        let cbx: i32 = x.rem_euclid(CHUNK_SIZE);
        let cby: i32 = y.rem_euclid(CHUNK_SIZE);
        let cbz: i32 = z.rem_euclid(CHUNK_SIZE);
        if let Some(chunk) = self.get_chunk(cx, cy, cz) {
            return chunk.get_block_from_xyz(cbx, cby, cbz);
        }
        else {
            return Block::air();
        }
    }
}