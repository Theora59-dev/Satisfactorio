use crate::engine::render::camera::Camera;
use crate::game::world::world::World;

#[derive(Clone, Copy, Debug)]
pub struct BlockHit {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub face: BlockFace,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockFace {
    PositiveX,
    NegativeX,
    PositiveY,
    NegativeY,
    PositiveZ,
    NegativeZ,
}

pub fn raycast_algorithm(camrea: &Camera, world: &World, max_distance: f32) -> Option<BlockHit> {
    let origin = camrea.eye;
    let direction = camrea.forward();

    let step_x: i32 = if direction.x >= 0.0 { 1 } else { -1 };
    let step_y: i32 = if direction.y >= 0.0 { 1 } else { -1 };
    let step_z: i32 = if direction.z >= 0.0 { 1 } else { -1 };

    let mut current_x = origin.x.floor() as i32;
    let mut current_y = origin.y.floor() as i32;
    let mut current_z = origin.z.floor() as i32;

    if direction.x < 0.0 {
        current_x -= 1;
    }
    if direction.y < 0.0 {
        current_y -= 1;
    }
    if direction.z < 0.0 {
        current_z -= 1;
    }

    let inv_dx = if direction.x != 0.0 {
        1.0 / direction.x.abs()
    } else {
        f32::INFINITY
    };
    let inv_dy = if direction.y != 0.0 {
        1.0 / direction.y.abs()
    } else {
        f32::INFINITY
    };
    let inv_dz = if direction.z != 0.0 {
        1.0 / direction.z.abs()
    } else {
        f32::INFINITY
    };

    let mut t_max_x = if direction.x != 0.0 {
        (current_x as f32 + if step_x > 0 { 1.0 } else { 0.0 } - origin.x) * inv_dx
    } else {
        f32::INFINITY
    };
    let mut t_max_y = if direction.y != 0.0 {
        (current_y as f32 + if step_y > 0 { 1.0 } else { 0.0 } - origin.y) * inv_dy
    } else {
        f32::INFINITY
    };
    let mut t_max_z = if direction.z != 0.0 {
        (current_z as f32 + if step_z > 0 { 1.0 } else { 0.0 } - origin.z) * inv_dz
    } else {
        f32::INFINITY
    };

    let t_delta_x = inv_dx;
    let t_delta_y = inv_dy;
    let t_delta_z = inv_dz;

    let mut distance = 0.0;

    loop {
        if distance > max_distance {
            return None;
        }

        let block = world.get_block_from_xyz(current_x, current_y, current_z);
        if block.id != 0 {
            let face = if t_max_x < t_max_y && t_max_x < t_max_z {
                if step_x > 0 {
                    BlockFace::NegativeX
                } else {
                    BlockFace::PositiveX
                }
            } else if t_max_y < t_max_z {
                if step_y > 0 {
                    BlockFace::NegativeY
                } else {
                    BlockFace::PositiveY
                }
            } else {
                if step_z > 0 {
                    BlockFace::NegativeZ
                } else {
                    BlockFace::PositiveZ
                }
            };
            return Some(BlockHit {
                x: current_x,
                y: current_y,
                z: current_z,
                face,
            });
        }

        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                current_x += step_x;
                t_max_x += t_delta_x;
                distance = t_max_x - t_delta_x;
            } else {
                current_z += step_z;
                t_max_z += t_delta_z;
                distance = t_max_z - t_delta_z;
            }
        } else {
            if t_max_y < t_max_z {
                current_y += step_y;
                t_max_y += t_delta_y;
                distance = t_max_y - t_delta_y;
            } else {
                current_z += step_z;
                t_max_z += t_delta_z;
                distance = t_max_z - t_delta_z;
            }
        }
    }
}
