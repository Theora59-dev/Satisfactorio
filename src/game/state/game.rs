use std::time::Instant;

use cgmath::{InnerSpace, Vector3, num_traits::ToPrimitive};
use wgpu::{Device, Queue};

use crate::{engine::render::{camera::Camera, mesh::world::WorldMesh, render::Renderer}, game::{player::{camera::CameraController, player::Player}, world::{chunk::Chunk, world::World}}};

pub struct GameState {
    pub world: World,
    pub world_mesh: WorldMesh,
    pub camera: Camera,
    pub camera_controller: CameraController,
    pub player: Player,
}

impl GameState {
    pub fn new(
        world: World,
        world_mesh: WorldMesh,
        camera: Camera,
        camera_controller: CameraController,
        player: Player
    ) -> Self {
        Self {
            world,
            world_mesh,
            camera,
            camera_controller,
            player
        }
    }

    pub fn init(&mut self, device: &Device) {
        let world_start = Instant::now();

        let [min_x, max_x, min_y, max_y, min_z, max_z] = self.player.get_rendered_chunk_range();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    let chunk = Chunk::generate(x, y, z);
                    self.world.set_chunk(x, y, z, chunk);
                }
            }
        }

        println!(
            "Time to make the world: {:.3}ms.",
            world_start.elapsed().as_micros().to_f64().unwrap() / 1_000.0
        );

        let mesh_start = Instant::now();

        self.world_mesh.update(&device, &self.world, &self.player);

        println!(
            "Time to make meshes: {:.3}ms.",
            mesh_start.elapsed().as_micros().to_f64().unwrap() / 1_000.0
        );
    }
    
    pub fn update(&mut self, queue: &Queue, renderer: &mut Renderer, dt: f32) {
        let forward = self.camera.forward();
        let right = self.camera.right();
        let up = cgmath::Vector3::unit_y();
        let mut direction = Vector3::new(0.0, 0.0, 0.0);

        if self.camera_controller.is_forward_pressed {
            direction += forward;
        }
        if self.camera_controller.is_backward_pressed {
            direction -= forward
        }
        if self.camera_controller.is_right_pressed {
            direction += right;
        }
        if self.camera_controller.is_left_pressed {
            direction -= right;
        }
        if self.camera_controller.is_up_pressed {
            direction += up;
        }
        if self.camera_controller.is_down_pressed {
            direction -= up;
        }

        if direction.magnitude2() > 0.0 {
            self.player.vel = direction.normalize() * (self.camera_controller.speed * dt);
        }
        else {
            self.player.vel = Vector3::new(0.0, 0.0, 0.0);
        }

        self.player.update();
        self.camera_controller.update_camera(dt, &mut self.camera, &self.player);
        renderer.camera_uniform.update_view_proj(&self.camera);
        
        queue.write_buffer(
            &renderer.camera_buffer,
            0,
            bytemuck::cast_slice(&[renderer.camera_uniform]),
        );
    }
}