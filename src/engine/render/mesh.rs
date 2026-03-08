use std::{collections::HashMap, sync::Arc};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use wgpu::{util::DeviceExt, Buffer, Device};

use crate::engine::render::geometry::Vertex;
use crate::engine::render::geometry::{Direction, FaceMask};
use crate::{
    player::Player,
    world::{BlockInstance, Chunk, PaddedChunk, World, CHUNK_SIZE},
};

pub struct ChunkMesh {
    pub vertices: Vec<Vertex>,
    pub vertex_buffer: Buffer,
    pub vertices_count: u32,
    dirty: bool,
}

impl ChunkMesh {
    pub fn make_greedy(
        chunk: &Chunk,
        world: &World,
        device: &Device,
        cx: i32,
        cy: i32,
        cz: i32,
    ) -> ChunkMesh {
        let mut vertices: Vec<Vertex> = vec![];

        let offset_x = cx * CHUNK_SIZE;
        let offset_y = cy * CHUNK_SIZE;
        let offset_z = cz * CHUNK_SIZE;

        // We allocate once to avoid memory reallocation/destruction.
        let mut mask: [[FaceMask; CHUNK_SIZE as usize]; CHUNK_SIZE as usize] =
            [[FaceMask::empty(); CHUNK_SIZE as usize]; CHUNK_SIZE as usize];

        let mut previous: BlockInstance;
        let mut current: BlockInstance;

        let padded_chunk = PaddedChunk::new(chunk, world);

        // X axis
        for x in 0..=CHUNK_SIZE {
            // Mask
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    previous = padded_chunk.get_block_from_xyz(x, y + 1, z + 1);
                    current = padded_chunk.get_block_from_xyz(x + 1, y + 1, z + 1);

                    match (previous.is_air(), current.is_air()) {
                        (true, true) | (false, false) => {
                            continue;
                        }
                        (true, false) => {
                            mask[y as usize][z as usize] =
                                FaceMask::from(false, current.id, Direction::Left);
                        }
                        (false, true) => {
                            mask[y as usize][z as usize] =
                                FaceMask::from(false, previous.id, Direction::Right);
                        }
                    }
                }
            }

            // Mesh quads
            for y in 0..CHUNK_SIZE {
                let mut z = 0;
                while z < CHUNK_SIZE {
                    let face = mask[y as usize][z as usize];
                    if face.get_visited() {
                        z += 1;
                        continue;
                    }

                    mask[y as usize][z as usize].set_visited(true);

                    let mut quad_y = 1;
                    let mut quad_z = 1;

                    // We grow the quad in the y-axis
                    'outer: for iy in (y as usize + 1)..(CHUNK_SIZE as usize) as usize {
                        if mask[iy][z as usize].get_visited()
                            || mask[iy][z as usize].data != face.data
                        {
                            break 'outer;
                        }
                        quad_y += 1;
                        // Clear from the mask
                        mask[iy][z as usize].set_visited(true);
                    }

                    // We grow the quad in the z-axis
                    'outer: for iz in (z + 1)..CHUNK_SIZE {
                        // We check if every face in the y is compatible with our expansion, and if not, we stop it
                        for iy in y..(y + quad_y) {
                            if mask[iy as usize][iz as usize].get_visited()
                                || mask[iy as usize][iz as usize].data != face.data
                            {
                                break 'outer;
                            }
                        }
                        quad_z += 1;
                        // Clear this space from the mask since we expand
                        for iy in (y as usize)..(y + quad_y) as usize {
                            mask[iy][iz as usize].set_visited(true);
                        }
                    }

                    // Add the quad to the mesh
                    let is_left_face = face.get_face() == Direction::Left;

                    let x = (x + offset_x) as f32;
                    let y0 = (y + offset_y) as f32;
                    let y1 = (y + quad_y + offset_y) as f32;
                    let z0 = (z + offset_z) as f32;
                    let z1 = (z + quad_z + offset_z) as f32;

                    let v1 = Vertex::new(x, y0, z0, 0);
                    let v2 = Vertex::new(x, y1, z1, 0);
                    let v3 = Vertex::new(x, y1, z0, 0);
                    let v4 = Vertex::new(x, y0, z1, 0);

                    if is_left_face {
                        vertices.extend_from_slice(&[v1, v2, v3, v1, v4, v2]);
                    } else {
                        vertices.extend_from_slice(&[v1, v3, v2, v1, v2, v4]);
                    }

                    // We can at least skip that part, knowing itering over this small part of the quad won't result in anything
                    // Skipping quad_y will probably makes us lose vertex in the process, this is why we just skip z.
                    z += quad_z;
                }
            }
        }

        // Y axis
        for y in 0..=CHUNK_SIZE {
            // Mask
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    previous = padded_chunk.get_block_from_xyz(x + 1, y, z + 1);
                    current = padded_chunk.get_block_from_xyz(x + 1, y + 1, z + 1);

                    match (previous.is_air(), current.is_air()) {
                        (true, true) | (false, false) => {
                            continue;
                        }
                        (true, false) => {
                            mask[x as usize][z as usize] =
                                FaceMask::from(false, current.id, Direction::Below);
                        }
                        (false, true) => {
                            mask[x as usize][z as usize] =
                                FaceMask::from(false, previous.id, Direction::Above);
                        }
                    }
                }
            }

            // Mesh quads
            for x in 0..CHUNK_SIZE {
                let mut z = 0;
                while z < CHUNK_SIZE {
                    let face = mask[x as usize][z as usize];
                    if face.get_visited() {
                        z += 1;
                        continue;
                    }

                    mask[x as usize][z as usize].set_visited(true);

                    let mut quad_x = 1;
                    let mut quad_z = 1;

                    // We grow the quad in the x-axis
                    'outer: for ix in (x as usize + 1)..(CHUNK_SIZE as usize) as usize {
                        if mask[ix][z as usize].get_visited()
                            || mask[ix][z as usize].data != face.data
                        {
                            break 'outer;
                        }
                        quad_x += 1;
                        // Clear from the mask
                        mask[ix][z as usize].set_visited(true);
                    }

                    // We grow the quad in the z-axis
                    'outer: for iz in (z + 1)..CHUNK_SIZE {
                        // We check if every face in the x is compatible with our expansion, and if not, we stop it
                        for ix in x..(x + quad_x) {
                            if mask[ix as usize][iz as usize].get_visited()
                                || mask[ix as usize][iz as usize].data != face.data
                            {
                                break 'outer;
                            }
                        }
                        quad_z += 1;
                        // Clear this space from the mask since we expand
                        for iy in (x as usize)..(x + quad_x) as usize {
                            mask[iy][iz as usize].set_visited(true);
                        }
                    }

                    // Add the quad to the mesh
                    let is_above_face = face.get_face() == Direction::Above;

                    let y = (y + offset_y) as f32;
                    let x0 = (x + offset_x) as f32;
                    let x1 = (x + quad_x + offset_x) as f32;
                    let z0 = (z + offset_z) as f32;
                    let z1 = (z + quad_z + offset_z) as f32;

                    let v1 = Vertex::new(x0, y, z0, 0);
                    let v2 = Vertex::new(x1, y, z1, 0);
                    let v3 = Vertex::new(x1, y, z0, 0);
                    let v4 = Vertex::new(x0, y, z1, 0);

                    if is_above_face {
                        vertices.extend_from_slice(&[v1, v2, v3, v1, v4, v2]);
                    } else {
                        vertices.extend_from_slice(&[v1, v3, v2, v1, v2, v4]);
                    }

                    // We can at least skip that part, knowing itering over this small part of the quad won't result in anything
                    // Skipping quad_x will probably makes us lose vertex in the process, this is why we just skip z.
                    z += quad_z;
                }
            }
        }

        // Z axis
        for z in 0..=CHUNK_SIZE {
            // Mask
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    previous = padded_chunk.get_block_from_xyz(x + 1, y + 1, z);
                    current = padded_chunk.get_block_from_xyz(x + 1, y + 1, z + 1);

                    match (previous.is_air(), current.is_air()) {
                        (true, true) | (false, false) => {
                            continue;
                        }
                        (true, false) => {
                            mask[x as usize][y as usize] =
                                FaceMask::from(false, current.id, Direction::Back);
                        }
                        (false, true) => {
                            mask[x as usize][y as usize] =
                                FaceMask::from(false, previous.id, Direction::Front);
                        }
                    }
                }
            }

            // Mesh quads
            for x in 0..CHUNK_SIZE {
                let mut y = 0;
                while y < CHUNK_SIZE {
                    let face = mask[x as usize][y as usize];
                    if face.get_visited() {
                        y += 1;
                        continue;
                    }

                    mask[x as usize][y as usize].set_visited(true);

                    let mut quad_x = 1;
                    let mut quad_y = 1;

                    // We grow the quad in the x-axis
                    'outer: for ix in (x as usize + 1)..(CHUNK_SIZE as usize) as usize {
                        if mask[ix][y as usize].get_visited()
                            || mask[ix][y as usize].data != face.data
                        {
                            break 'outer;
                        }
                        quad_x += 1;
                        // Clear from the mask
                        mask[ix][y as usize].set_visited(true);
                    }

                    // We grow the quad in the y-axis
                    'outer: for iz in (y + 1)..CHUNK_SIZE {
                        // We check if every face in the x is compatible with our expansion, and if not, we stop it
                        for ix in x..(x + quad_x) {
                            if mask[ix as usize][iz as usize].get_visited()
                                || mask[ix as usize][iz as usize].data != face.data
                            {
                                break 'outer;
                            }
                        }
                        quad_y += 1;
                        // Clear this space from the mask since we expand
                        for iy in (x as usize)..(x + quad_x) as usize {
                            mask[iy][iz as usize].set_visited(true);
                        }
                    }

                    // Add the quad to the mesh
                    let z = (z + offset_z) as f32;

                    let x0 = (x + offset_x) as f32;
                    let x1 = (x + quad_x + offset_x) as f32;
                    let y0 = (y + offset_y) as f32;
                    let y1 = (y + quad_y + offset_y) as f32;

                    let v1 = Vertex::new(x0, y0, z, 0);
                    let v2 = Vertex::new(x1, y0, z, 0);
                    let v3 = Vertex::new(x1, y1, z, 0);
                    let v4 = Vertex::new(x0, y1, z, 0);

                    let is_front = face.get_face() == Direction::Front;

                    if is_front {
                        vertices.extend_from_slice(&[v1, v2, v3, v1, v3, v4]);
                    } else {
                        vertices.extend_from_slice(&[v1, v3, v2, v1, v4, v3]);
                    }

                    // We can at least skip that part, knowing itering over this small part of the quad won't result in anything
                    // Skipping quad_x will probably makes us lose vertex in the process, this is why we just skip y.
                    y += quad_y;
                }
            }
        }

        let vertex_count = vertices.len() as u32;
        let vertex_buffer = (*device).create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("chunk vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        return ChunkMesh {
            vertices: vertices,
            vertex_buffer: vertex_buffer,
            vertices_count: vertex_count,
            dirty: false,
        };
    }
}

pub struct WorldMesh {
    pub meshes: HashMap<(i32, i32, i32), Arc<ChunkMesh>>,
}

impl WorldMesh {
    pub fn new() -> WorldMesh {
        return WorldMesh {
            meshes: HashMap::new(),
        };
    }

    /// Builds simultaneously every single chunk within the player's both horizontal and vertical render distance only if it needs it (if dirty == true).
    pub fn update(device: &Device, world: &World, player: &Player, old: &WorldMesh) -> WorldMesh {
        let meshes: HashMap<_, _> = world
            .get_player_rendered_chunks(player)
            .into_par_iter()
            .map(|(chunk, cx, cy, cz)| {
                let key = (cx, cy, cz);

                if let Some(existing) = old.meshes.get(&key) {
                    if !existing.dirty {
                        return (key, Arc::clone(existing));
                    }
                }

                let mesh = ChunkMesh::make_greedy(chunk, world, device, cx, cy, cz);
                // let mesh = ChunkMesh::make(chunk, world, cx, cy, cz);
                return (key, Arc::new(mesh));
            })
            .collect();

        return WorldMesh { meshes };
    }
}
