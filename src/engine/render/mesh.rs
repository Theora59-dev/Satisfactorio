use crate::world::{CHUNK_SIZE, CHUNK_SIZE_SQR, World, Chunk, Block};

const X: u8 = 1;
const Y: u8 = 2;
const Z: u8 = 4;

// Block's vertices positions (counter clockwise)
// Face à un cube, en partant du coin inférieur gauche
const BLOCK_VERTEX_000: u8 = 0;
const BLOCK_VERTEX_100: u8 = X;
const BLOCK_VERTEX_110: u8 = X | Y;
const BLOCK_VERTEX_010: u8 = Y;
// Face derrière la première, en partant du coin inférieur gauche
const BLOCK_VERTEX_001: u8 = Z;
const BLOCK_VERTEX_101: u8 = X | Z;
const BLOCK_VERTEX_111: u8 = X | Y | Z;
const BLOCK_VERTEX_011: u8 = Y | Z;

struct ChunkMesh {
    vertices: Vec<u16>
}

impl ChunkMesh {
    fn new() -> ChunkMesh {
        return ChunkMesh {
            vertices: vec![]
        };
    }

    pub fn make(chunk: &Chunk, world: &World) -> ChunkMesh {
        // C'est là que ça devient vraiment rigolo
        let mesh = ChunkMesh::new();

        let mut block: Block;
        for z in 0..CHUNK_SIZE {
            let iz = z * CHUNK_SIZE_SQR;
            for y in 0..CHUNK_SIZE {
                let iy = y * CHUNK_SIZE;
                for x in 0..CHUNK_SIZE {
                    let i = (x + iy + iz) as usize;
                    block = chunk.get_block_from_i(i);
                    if block.is_air() {
                        continue;
                    }

                    // Check des voisins intra-chunk
                    let left = world.get_block_from_xyz(x-1, y, z);
                    let right = world.get_block_from_xyz(x+1, y, z);
                    let below = world.get_block_from_xyz(x, y-1, z);
                    let above = world.get_block_from_xyz(x, y+1, z);
                    let behind = world.get_block_from_xyz(x, y, z-1);
                    let front = world.get_block_from_xyz(x, y, z+1);

                }
            }
        }

        return mesh;
    }
}