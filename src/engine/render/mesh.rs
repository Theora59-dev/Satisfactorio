use std::{collections::HashMap, sync::Arc};

use cgmath::num_traits::ToPrimitive;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

<<<<<<< Updated upstream
use crate::{player::Player, world::{BlockInstance, CHUNK_SIZE, CHUNK_SIZE_SQR, Chunk, FIRST_PADDED_CHUNK_AXIS_INDEX, LAST_CHUNK_AXIS_INDEX, LAST_PADDED_CHUNK_AXIS_INDEX, PADDED_CHUNK_SIZE, PaddedChunk, World}};
=======
use crate::{player::Player, world::{BlockInstance, CHUNK_SIZE, CHUNK_SIZE_SQR, Chunk, LAST_CHUNK_BLOCK_INDEX, World}};
>>>>>>> Stashed changes
use crate::engine::render::geometry::Vertex;

const X: f32 = 1.0;
const Y: f32 = 1.0;
const Z: f32 = 1.0;

// Block's vertices positions (counter clockwise)
// Face arrière d'un cube, en partant du coin inférieur gauche
const V_000: [f32; 3] = [0.0, 0.0, 0.0];
const V_100: [f32; 3] = [ X , 0.0, 0.0];
const V_110: [f32; 3] = [ X ,  Y , 0.0];
const V_010: [f32; 3] = [0.0,  Y , 0.0];
// Face devant la première, en partant du coin inférieur gauche
const V_001: [f32; 3] = [0.0, 0.0,  Z ];
const V_101: [f32; 3] = [ X , 0.0,  Z ];
const V_111: [f32; 3] = [ X ,  Y ,  Z ];
const V_011: [f32; 3] = [0.0,  Y ,  Z ];

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Face {
    Above = 0,
    Below = 1,
    Left  = 2,
    Right = 3,
    Front = 4,
    Back  = 5,
}

#[derive(Clone, Copy)]
struct FaceMask {
    data: u64
}

const VISITED_SHIFT: u64 = 63;
const BLOCK_ID_SHIFT: u64 = 31;
const BLOCK_ID_MASK: u64 = 0xFFFF_FFFF;
const FACE_MASK: u64 = 0b111;

impl Face {
    #[inline(always)]
    fn from_bits_unchecked(v: u8) -> Self {
        debug_assert!(v < 6);
        unsafe { std::mem::transmute(v) }
    }
}

impl FaceMask {

    #[inline(always)]
    fn empty() -> FaceMask {
        return FaceMask {
            data: 0u64,
        };
    }

    fn from(visited: bool, id: u32, face: Face) -> FaceMask {
        let mut mask = FaceMask {
            data: 0u64
        };
        mask.set_visited(visited);
        mask.set_block_id(id);
        mask.set_face(face);
        return mask;
    }

    fn to(&self) -> (bool, u32, Face) {
        return (self.get_visited(), self.get_block_id(), self.get_face());
    }

    #[inline(always)]
    fn get_visited(self) -> bool {
        (self.data >> VISITED_SHIFT) != 0
    }

    #[inline(always)]
    fn set_visited(&mut self, v: bool) {
        self.data ^= (-(v as i64) as u64 ^ self.data)
            & (1 << VISITED_SHIFT);
    }

    #[inline(always)]
    fn get_block_id(self) -> u32 {
        ((self.data >> BLOCK_ID_SHIFT) & BLOCK_ID_MASK) as u32
    }

    #[inline(always)]
    fn set_block_id(&mut self, id: u32) {
        self.data =
            (self.data & !(BLOCK_ID_MASK << BLOCK_ID_SHIFT))
            | ((id as u64) << BLOCK_ID_SHIFT);
    }

    #[inline(always)]
    fn get_face(self) -> Face {
        Face::from_bits_unchecked((self.data & FACE_MASK) as u8)
    }

    #[inline(always)]
    fn set_face(&mut self, face: Face) {
        self.data =
            (self.data & !FACE_MASK)
            | (face as u64);
    }
}

pub struct ChunkMesh {
    pub vertices: Vec<Vertex>,
    dirty: bool,
}

impl ChunkMesh {
    fn new() -> ChunkMesh {
        return ChunkMesh {
            vertices: vec![],
            dirty: true
        };
    }

    fn add_behind_face(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.extend_from_slice(&[
            Vertex::new(V_100[0] + x, V_100[1] + y, V_100[2] + z),
            Vertex::new(V_000[0] + x, V_000[1] + y, V_000[2] + z),
            Vertex::new(V_110[0] + x, V_110[1] + y, V_110[2] + z),
            Vertex::new(V_110[0] + x, V_110[1] + y, V_110[2] + z),
            Vertex::new(V_000[0] + x, V_000[1] + y, V_000[2] + z),
            Vertex::new(V_010[0] + x, V_010[1] + y, V_010[2] + z)
        ]);
    }

    fn add_front_face(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.extend_from_slice(&[
            Vertex::new(V_001[0] + x, V_001[1] + y, V_001[2] + z),
            Vertex::new(V_101[0] + x, V_101[1] + y, V_101[2] + z),
            Vertex::new(V_011[0] + x, V_011[1] + y, V_011[2] + z),
            Vertex::new(V_011[0] + x, V_011[1] + y, V_011[2] + z),
            Vertex::new(V_101[0] + x, V_101[1] + y, V_101[2] + z),
            Vertex::new(V_111[0] + x, V_111[1] + y, V_111[2] + z)
        ]);
    }

    fn add_left_face(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.extend_from_slice(&[
            Vertex::new(V_000[0] + x, V_000[1] + y, V_000[2] + z),
            Vertex::new(V_001[0] + x, V_001[1] + y, V_001[2] + z),
            Vertex::new(V_010[0] + x, V_010[1] + y, V_010[2] + z),
            Vertex::new(V_010[0] + x, V_010[1] + y, V_010[2] + z),
            Vertex::new(V_001[0] + x, V_001[1] + y, V_001[2] + z),
            Vertex::new(V_011[0] + x, V_011[1] + y, V_011[2] + z)
        ]);
    }

    fn add_right_face(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.extend_from_slice(&[
            Vertex::new(V_100[0] + x, V_100[1] + y, V_100[2] + z),
            Vertex::new(V_110[0] + x, V_110[1] + y, V_110[2] + z),
            Vertex::new(V_101[0] + x, V_101[1] + y, V_101[2] + z),
            Vertex::new(V_101[0] + x, V_101[1] + y, V_101[2] + z),
            Vertex::new(V_110[0] + x, V_110[1] + y, V_110[2] + z),
            Vertex::new(V_111[0] + x, V_111[1] + y, V_111[2] + z)
        ]);
    }

    fn add_above_face(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.extend_from_slice(&[
            Vertex::new(V_010[0] + x, V_010[1] + y, V_010[2] + z),
            Vertex::new(V_011[0] + x, V_011[1] + y, V_011[2] + z),
            Vertex::new(V_110[0] + x, V_110[1] + y, V_110[2] + z),
            Vertex::new(V_110[0] + x, V_110[1] + y, V_110[2] + z),
            Vertex::new(V_011[0] + x, V_011[1] + y, V_011[2] + z),
            Vertex::new(V_111[0] + x, V_111[1] + y, V_111[2] + z)
        ]);
    }

    fn add_below_face(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.extend_from_slice(&[
            Vertex::new(V_000[0] + x, V_000[1] + y, V_000[2] + z),
            Vertex::new(V_100[0] + x, V_100[1] + y, V_100[2] + z),
            Vertex::new(V_001[0] + x, V_001[1] + y, V_001[2] + z),
            Vertex::new(V_001[0] + x, V_001[1] + y, V_001[2] + z),
            Vertex::new(V_100[0] + x, V_100[1] + y, V_100[2] + z),
            Vertex::new(V_101[0] + x, V_101[1] + y, V_101[2] + z)
        ]);
    }

    pub fn make(chunk: &Chunk, world: &World, cx: i32, cy: i32, cz: i32) -> ChunkMesh {
        // C'est là que ça devient vraiment rigolo
        let mut mesh = ChunkMesh::new();

        let offset_x = cx * CHUNK_SIZE;
        let offset_y = cy * CHUNK_SIZE;
        let offset_z = cz * CHUNK_SIZE;

        let mut block: BlockInstance;
        for lz in 0..CHUNK_SIZE {
            let iz = lz * CHUNK_SIZE_SQR;
            for ly in 0..CHUNK_SIZE {
                let iy = ly * CHUNK_SIZE;
                for lx in 0..CHUNK_SIZE {
                    let li = (lx + iy + iz) as usize;
                    block = chunk.get_block_from_i(li);
                    if block.is_air() {
                        continue;
                    }
                    
                    let gx = lx as i32 + offset_x;
                    let gy = ly as i32 + offset_y;
                    let gz = lz as i32 + offset_z;
                    
                    // println!("{gx} {gy} {gz} ");

                    // Check des voisins intra-chunk
                    let left = if lx == 0 { 
                        world.get_block_from_xyz(gx-1, gy, gz)
                    }
                    else {
                        chunk.get_block_from_xyz(lx-1, ly, lz)
                    };
                    let right = if lx == LAST_CHUNK_AXIS_INDEX {
                        world.get_block_from_xyz(gx+1, gy, gz)
                    }
                    else {
                        chunk.get_block_from_xyz(lx+1, ly, lz)
                    };
                    let below = if ly == 0 {
                        world.get_block_from_xyz(gx, gy-1, gz)
                    }
                    else {
                        chunk.get_block_from_xyz(lx, ly-1, lz)
                    };
                    let above = if ly == LAST_CHUNK_AXIS_INDEX {
                        world.get_block_from_xyz(gx, gy+1, gz)
                    }
                    else {
                        chunk.get_block_from_xyz(lx, ly+1, lz)
                    };
                    let behind = if lz == 0 {
                        world.get_block_from_xyz(gx, gy, gz-1)
                    }
                    else {
                        chunk.get_block_from_xyz(lx, ly, lz-1)
                    };
                    let front = if lz == LAST_CHUNK_AXIS_INDEX {
                        world.get_block_from_xyz(gx, gy, gz+1)
                    }
                    else {
                        chunk.get_block_from_xyz(lx, ly, lz+1)
                    };
                    
                    if front.is_air() {
                        // print!("f ");
                        mesh.add_front_face(gx.to_f32().unwrap(), gy.to_f32().unwrap(), gz.to_f32().unwrap());
                    }

                    if behind.is_air() {
                        // print!("bh ");
                        mesh.add_behind_face(gx.to_f32().unwrap(), gy.to_f32().unwrap(), gz.to_f32().unwrap());
                    }

                    if left.is_air() {
                        // print!("l ");
                        mesh.add_left_face(gx.to_f32().unwrap(), gy.to_f32().unwrap(), gz.to_f32().unwrap());
                    }

                    if right.is_air() {
                        // print!("r ");
                        mesh.add_right_face(gx.to_f32().unwrap(), gy.to_f32().unwrap(), gz.to_f32().unwrap());
                    }

                    if above.is_air() {
                        // print!("a ");
                        mesh.add_above_face(gx.to_f32().unwrap(), gy.to_f32().unwrap(), gz.to_f32().unwrap());
                    }

                    if below.is_air() {
                        // print!("bl ");
                        mesh.add_below_face(gx.to_f32().unwrap(), gy.to_f32().unwrap(), gz.to_f32().unwrap());
                    }
                }
            }
        }

        mesh.dirty = false;

        return mesh;
    }

    pub fn make_greedy(chunk: &Chunk, world: &World, cx: i32, cy: i32, cz: i32) -> ChunkMesh {
        let mut mesh = ChunkMesh::new();

        let offset_x = cx * CHUNK_SIZE;
        let offset_y = cy * CHUNK_SIZE;
        let offset_z = cz * CHUNK_SIZE;

        // We allocate once to avoid memory reallocation/destruction.
        let mut mask: [[FaceMask; CHUNK_SIZE as usize]; CHUNK_SIZE as usize] = [[FaceMask::empty(); CHUNK_SIZE as usize]; CHUNK_SIZE as usize];
        // let mut mask: [[Option<(u32, Face)>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize] = [[None; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];
        
<<<<<<< Updated upstream
        let mut previous: BlockInstance;
        let mut current: BlockInstance;

        // Todo:
        // - replace world.get_block by chunk.get_block whenever possible
        // - check if the block is in the chunk before adding it to the mask
=======
        let mut current: BlockInstance;
        let mut next: BlockInstance;
>>>>>>> Stashed changes
        
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
                            mask[y as usize][z as usize].set_visited(true);
                            // continue;
                        }
                        (true, false) => {
                            mask[y as usize][z as usize] = FaceMask::from(false, current.id, Face::Left);
                            // mask[y as usize][z as usize] = Some((current.id, Face::Left));
                        }
                        (false, true) => {
                            mask[y as usize][z as usize] = FaceMask::from(false, previous.id, Face::Right);
                            // mask[y as usize][z as usize] = Some((previous.id, Face::Right));
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
                    // let Some(face) = mask[y as usize][z as usize] else {
                    //     z += 1;
                    //     continue;
                    // };

                    mask[y as usize][z as usize].set_visited(true);

                    let mut quad_y = 1;
                    let mut quad_z = 1;

                    // We grow the quad in the y-axis
                    'outer: for iy in (y as usize+1)..(CHUNK_SIZE as usize) as usize {
<<<<<<< Updated upstream
                        if mask[iy][z as usize].get_visited() || mask[iy][z as usize].data != face.data {
=======
                        if mask[iy][z as usize] != Some(face) {
>>>>>>> Stashed changes
                            break 'outer;
                        }
                        quad_y += 1;
                        // Clear from the mask
<<<<<<< Updated upstream
                        mask[iy][z as usize].set_visited(true);
=======
                        mask[iy][z as usize] = None;
>>>>>>> Stashed changes
                    }

                    // We grow the quad in the z-axis
                    'outer: for iz in (z+1)..CHUNK_SIZE {
                        // We check if every face in the y is compatible with our expansion, and if not, we stop it
                        for iy in y..(y + quad_y) {
                            if mask[iy as usize][iz as usize].get_visited() || mask[iy as usize][iz as usize].data != face.data {
                                break 'outer;
                            }
                        }
                        quad_z += 1;
                        // Clear this space from the mask since we expand
                        for iy in (y as usize)..(y + quad_y) as usize {
<<<<<<< Updated upstream
                            mask[iy][iz as usize].set_visited(true);
=======
                            mask[iy][iz as usize] = None;
>>>>>>> Stashed changes
                        }
                    }

                    // Add the quad to the mesh
                    let is_left_face = face.get_face() == Face::Left;
                    
                    let x = (x - 1 + offset_x + (is_left_face as i32)) as f32;
                    let y0 = (y + offset_y) as f32;
                    let y1 = (y + quad_y + offset_y) as f32;
                    let z0 = (z + offset_z) as f32;
                    let z1 = (z + quad_z + offset_z) as f32;

                    let v1 = Vertex::new(x, y0, z0);
                    let v2 = Vertex::new(x, y1, z1);
                    let v3 = Vertex::new(x, y1, z0);
                    let v4 = Vertex::new(x, y0, z1);

                    if is_left_face {
                        mesh.vertices.extend_from_slice(&[
                            v1, v2, v3, v1, v4, v2
                        ]);
                    }
                    else {
                        mesh.vertices.extend_from_slice(&[
                            v1, v3, v2, v1, v2, v4
                        ]);
                    }
<<<<<<< Updated upstream

                    // We can at least skip that part, knowing itering over this small part of the quad won't result in anything
                    // Skipping quad_y will probably makes us lose vertex in the process, this is why we just skip z.
                    z += quad_z;
=======
>>>>>>> Stashed changes
                }
            }
        }

        // Y axis

        // Z axis

        return mesh;
    }
}

pub struct WorldMesh {
    pub meshes: HashMap<(i32,i32,i32), Arc<ChunkMesh>>
}

impl WorldMesh {
    pub fn new() -> WorldMesh {
        return WorldMesh {
            meshes: HashMap::new(),
        };
    }

    /// Builds simultaneously every single chunk within the player's both horizontal and vertical render distance only if it needs it (if dirty == true).
    pub fn update(world: &World, player: &Player, old: &WorldMesh) -> WorldMesh {
        let meshes: HashMap<_, _> = world.get_player_rendered_chunks(player)
            .into_par_iter()
            .map(|(chunk, cx, cy, cz)| {
                let key = (cx, cy, cz);

                if let Some(existing) = old.meshes.get(&key) {
                    if !existing.dirty {
                        return (key, Arc::clone(existing));
                    }
                }

                let mesh = ChunkMesh::make_greedy(chunk, world, cx, cy, cz);
                // let mesh = ChunkMesh::make(chunk, world, cx, cy, cz);
                return (key, Arc::new(mesh));
            })
            .collect();

        return WorldMesh {
            meshes
        };
    }
}