use std::sync::{Arc, RwLock};

use noise::NoiseFn;

use crate::world::data::block::{BlockInstance, BlockManager};
use crate::world::data::chunk::{Chunk, CHUNK_BLOCK_NUMBER, CHUNK_SIZE};
use crate::world::generation::chunk_generator::ChunkGenContext;

pub struct ChunkWithChecksum {
    pub chunk_data: crate::world::data::chunk::ChunkData,
    pub checksum: [u8; 2],
}

impl Chunk {
    #[inline]
    pub fn generate(block_manager: Arc<RwLock<BlockManager>>, cx: i32, cy: i32, cz: i32, seed: u32) -> Self {
        let ctx = ChunkGenContext::new(seed, block_manager);
        Self::generate_with_context(cx, cy, cz, &ctx)
    }

    #[inline]
    pub fn generate_with_context(cx: i32, cy: i32, cz: i32, ctx: &ChunkGenContext) -> Self {
        let cwx = cx * CHUNK_SIZE;
        let cwy = cy * CHUNK_SIZE;
        let cwz = cz * CHUNK_SIZE;

        let blocks = ctx.block_manager.read().unwrap();

        let grass_id = blocks
            .get_block_by_string(String::from("grass"))
            .expect("Did not find block 'grass' in block manager")
            .get_id();
        let dirt_id = blocks
            .get_block_by_string(String::from("dirt"))
            .expect("Did not find block 'dirt' in block manager")
            .get_id();
        let stone_id = blocks
            .get_block_by_string(String::from("stone"))
            .expect("Did not find block 'stone' in block manager")
            .get_id();
        let sand_id = blocks
            .get_block_by_string(String::from("sand"))
            .expect("Did not find block 'sand' in block manager")
            .get_id();
        let snow_id = blocks
            .get_block_by_string(String::from("snow"))
            .expect("Did not find block 'snow' in block manager")
            .get_id();

        let mut ore_ids: Vec<Option<u32>> = Vec::with_capacity(ctx.get_ore_count());
        for i in 0..ctx.get_ore_count() {
            if let Some(config) = ctx.get_ore_config(i) {
                ore_ids.push(blocks.get_block_by_string(config.block_id.clone()).map(|b| b.get_id()));
            } else {
                ore_ids.push(None);
            }
        }

        drop(blocks);

        let blocks = vec![BlockInstance::air(); CHUNK_BLOCK_NUMBER];

        let mut chunk = Self {
            blocks,
            x: cx,
            y: cy,
            z: cz,
        };

        for x in 0..CHUNK_SIZE {
            let wx = (x + cwx) as f64;

            for z in 0..CHUNK_SIZE {
                let wz = (z + cwz) as f64;

                let biome_idx = ctx.get_biome_index(wx, wz);
                let biome = &ctx.biome_registry.biomes[biome_idx];

                let nx = wx * biome.terrain.scale;
                let nz = wz * biome.terrain.scale;
                let valeur = ctx.surface.get([nx, nz]);
                let terrain_y = valeur.mul_add(biome.terrain.amplitude, biome.terrain.base_height) as i32;

                let surface_id = match biome.layers.surface_block.as_str() {
                    "sand" => sand_id,
                    "snow" => snow_id,
                    _ => grass_id,
                };
                let subsurface_id = match biome.layers.subsurface_block.as_str() {
                    "sand" => sand_id,
                    "snow" => snow_id,
                    "dirt" => dirt_id,
                    _ => dirt_id,
                };

                for y in 0..CHUNK_SIZE {
                    let wy = y + cwy;
                    if wy >= terrain_y {
                        continue;
                    }

                    let depth = terrain_y - wy;

                    let is_cave = ctx.is_cave_block(wx, wy as f64, wz, depth);

                    if !is_cave {
                        let block_id = match wy {
                            y if y == terrain_y - 1 => surface_id,
                            y if y >= terrain_y - biome.layers.subsurface_depth => subsurface_id,
                            _ => {
                                let mut placed_id = stone_id;
                                for i in 0..ctx.get_ore_count() {
                                    if let Some(config) = ctx.get_ore_config(i) {
                                        if wy >= config.depth_min && wy <= config.depth_max {
                                            if ctx.should_place_ore(i, wx, wy as f64, wz) {
                                                if let Some(ore_id) = ore_ids[i] {
                                                    placed_id = ore_id;
                                                }
                                                break;
                                            }
                                        }
                                    }
                                }
                                placed_id
                            }
                        };
                        chunk.set_block_from_xyz(x, y, z, BlockInstance::new(block_id));
                    }
                }
            }
        }

        chunk
    }
}
