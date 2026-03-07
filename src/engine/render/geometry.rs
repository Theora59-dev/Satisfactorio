use bytemuck::{Pod, Zeroable};

/// The color to display the world in, in RGB format.
/// 
/// Components must be between 0 and 1 inclusive.
pub const BASE_VERTEX_COLOR: [f32; 3] = [0.5, 0.5, 0.5]; // Gris neutre

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
    uv: u32,
    // tex_coords: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, uv: u32) -> Vertex {
        return Vertex {
            position: [x, y, z],
            color: BASE_VERTEX_COLOR,
            uv: uv
        }
    }

    pub fn new_with_rgb(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, uv: u32) -> Vertex {
        return Vertex {
            position: [x, y, z],
            color: [r, g, b],
            uv: uv
        }
    }

    pub fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// +Y
    Above = 0,
    /// -Y
    Below = 1,
    /// -X
    Left  = 2,
    /// +X
    Right = 3,
    /// +Z
    Front = 4,
    /// -Z
    Back  = 5,
}

#[derive(Clone, Copy)]
pub struct FaceMask {
    pub data: u64
}

const VISITED_SHIFT: u64 = 63;
const BLOCK_ID_SHIFT: u64 = 31;
const BLOCK_ID_MASK: u64 = 0xFFFF_FFFF;
const FACE_MASK: u64 = 0b111;

impl Direction {
    #[inline(always)]
    pub fn from_bits_unchecked(v: u8) -> Self {
        debug_assert!(v < 6);
        unsafe { std::mem::transmute(v) }
    }

    #[inline(always)]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}

impl FaceMask {
    #[inline(always)]
    pub fn empty() -> FaceMask {
        return FaceMask {
            data: 0x8000_0000_0000_0000u64,
        };
    }

    pub fn from(visited: bool, id: u32, face: Direction) -> FaceMask {
        let mut mask = FaceMask::empty();
        mask.set_visited(visited);
        mask.set_block_id(id);
        mask.set_face(face);
        return mask;
    }

    pub fn to(&self) -> (bool, u32, Direction) {
        return (self.get_visited(), self.get_block_id(), self.get_face());
    }

    #[inline(always)]
    pub fn get_visited(self) -> bool {
        (self.data >> VISITED_SHIFT) != 0
    }

    #[inline(always)]
    pub fn set_visited(&mut self, v: bool) {
        self.data ^= (-(v as i64) as u64 ^ self.data)
            & (1 << VISITED_SHIFT);
    }

    #[inline(always)]
    pub fn get_block_id(self) -> u32 {
        ((self.data >> BLOCK_ID_SHIFT) & BLOCK_ID_MASK) as u32
    }

    #[inline(always)]
    pub fn set_block_id(&mut self, id: u32) {
        self.data =
            (self.data & !(BLOCK_ID_MASK << BLOCK_ID_SHIFT))
            | ((id as u64) << BLOCK_ID_SHIFT);
    }

    #[inline(always)]
    pub fn get_face(self) -> Direction {
        Direction::from_bits_unchecked((self.data & FACE_MASK) as u8)
    }

    #[inline(always)]
    pub fn set_face(&mut self, face: Direction) {
        self.data =
            (self.data & !FACE_MASK)
            | (face as u64);
    }
}

/// Stores all the informations required by the GPU to draw a block's face on the screen, using 64 bits (8 bytes), in which 20 are still unused.
/// 
/// Read each component's description for more information.
pub struct RenderFaceTexto { 
    /// Stores: chunk-local top_left vertex position (15 bits - x: 5, y: 5, z: 5), quad width (5 bits) & height (5 bits) and face orientation (as Direction) (3 bits) packed together.
    /// 
    /// Structure : 0000_VVVV_VVVV_VVVV_VVVW_WWWW_HHHH_HFFF
    /// 
    /// - 28 bits used
    /// - 4 left (for flags, etc.)
    geometry: u32,
    /// Stores; texture id in the texture atlas (16 bits, up to 2^16 = 65536 textures)
    /// 
    /// Structure : 0000_0000_0000_0000_TTTT_TTTT_TTTT_TTTT
    /// 
    /// - 16 bits used
    /// - 16 left (shader information, other, etc.)
    material: u32,
}

impl RenderFaceTexto {
    pub fn new(x: u8, y: u8, z: u8, w: u8, h: u8, direction: Direction, texture: u16) -> RenderFaceTexto {
        let mut texto = RenderFaceTexto {
            geometry: 0,
            material: 0
        };

        texto.set_top_left_vertex(x, y, z);
        texto.set_quad_dimensions(w, h);
        texto.set_direction(direction);
        texto.set_texture(texture);

        return texto;
    }

    pub fn get_direction(&self) -> Direction {
        return Direction::from_bits_unchecked((self.geometry & 0x7) as u8);
    }

    pub fn get_quad_dimensions(&self) -> [u8; 2] {
        return [
            ((self.geometry & 0xF8) >> 3) as u8,
            ((self.geometry & 0x1F00) >> 8) as u8
        ];
    }

    pub fn get_top_left_vertex(&self) -> [u8; 3] {
        return [
            ((self.geometry & 0x0F80_0000) >> 23) as u8,
            ((self.geometry & 0x007C_0000) >> 18) as u8,
            ((self.geometry & 0x0003_E000) >> 13) as u8,
        ];
    }

    pub fn get_texture(&self) -> u16 {
        return (self.material & 0xFFFF) as u16;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.geometry = (self.geometry & !0x7) | direction as u32;
    }

    pub fn set_quad_dimensions(&mut self, w: u8, h: u8) {
        self.geometry = (self.geometry & !0x1FF8) | (
            (w as u32) << 8 |
            (h as u32) << 3
        );
    }

    pub fn set_top_left_vertex(&mut self, x: u8, y: u8, z: u8) {
        self.geometry = (self.geometry & !0x0FFF_E000) | (
            (x as u32) << 23 |
            (y as u32) << 18 |
            (z as u32) << 13
        );
    }

    pub fn set_texture(&mut self, texture: u16) {
        self.material = (self.material & !0xFFFF) | (texture as u32);
    }
}