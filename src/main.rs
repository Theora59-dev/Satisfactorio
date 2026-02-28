mod state;
mod texture;

use bytemuck::{Pod, Zeroable};
use state::run;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2]
}
impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
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
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }
    }
}

const VERTICES: &[Vertex] = &[
    // Première face du carré
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], },
    Vertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 0.0], },
    Vertex { position: [0.5, 0.5, -0.5], tex_coords: [1.0, 0.0], },
    Vertex { position: [0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], },
    // 2è, parallèle à la 1è
    Vertex { position: [-0.5, -0.5, 0.5], tex_coords: [0.0, 1.0], },
    Vertex { position: [-0.5, 0.5, 0.5], tex_coords: [0.0, 0.0], },
    Vertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], },
    Vertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 1.0], },
    // 3è, sur le flanc horizontal des 2 premières
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], },
    Vertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 0.0], },
    Vertex { position: [-0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], },
    Vertex { position: [-0.5, -0.5, 0.5], tex_coords: [1.0, 1.0], },
    // 4ème, parallèle à la 3è
    Vertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], },
    Vertex { position: [0.5, 0.5, -0.5], tex_coords: [0.0, 0.0], },
    Vertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], },
    Vertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 1.0], },
    // 5ème, faisant face au ciel
    Vertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], },
    Vertex { position: [0.5, 0.5, -0.5], tex_coords: [0.0, 0.0], },
    Vertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], },
    Vertex { position: [-0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], },
    // 6ème, parallèle à la 5è
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], },
    Vertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], },
    Vertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], },
    Vertex { position: [-0.5, -0.5, 0.5], tex_coords: [1.0, 1.0], },
];

const INDICES: &[u16] = &[
    0, 1, 2,
    2, 3, 0,

    6, 5, 4,
    4, 7, 6,

    8, 10, 9,
    8, 11, 10,

    13, 14, 12,
    14, 15, 12,

    16, 18, 17,
    16, 19, 18,

    21, 22, 20,
    22, 23, 20
];

fn main() {
    println!("Hello, world!");
    run();
}

// Tutoriel à voir : https://sotrh.github.io/learn-wgpu/beginner/tutorial4-buffer/
