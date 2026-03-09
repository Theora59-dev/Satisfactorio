use crate::game::world::chunk::CHUNK_SIZE;
use cgmath::num_traits::ToPrimitive;

/// Must be odd for semantic reasons (otherwise it will render one chunk more than this value)
const DEBUG_HORIZONTAL_RENDER_DISTANCE: u16 = 12;
/// Must be odd for semantic reasons (otherwise it will render one chunk more than this value)
const DEBUG_VERTICAL_RENDER_DISTANCE: u16 = 8;

pub struct Player {
    pub pos: cgmath::Point3<f32>,
    pub vel: cgmath::Vector3<f32>,
    pub horizontal_render_distance: u16,
    pub vertical_render_distance: u16,
}

impl Player {
    pub fn new() -> Player {
        return Player {
            pos: cgmath::Point3::new(0.0, 0.0, 0.0),
            vel: cgmath::Vector3::new(0.0, 0.0, 0.0),
            horizontal_render_distance: DEBUG_HORIZONTAL_RENDER_DISTANCE,
            vertical_render_distance: DEBUG_VERTICAL_RENDER_DISTANCE,
        };
    }

    pub fn set_render_distance(&mut self, horizontal: u16, vertical: u16) {
        self.horizontal_render_distance = horizontal;
        self.vertical_render_distance = vertical;
    }

    pub fn update(&mut self) {
        self.pos += self.vel;
    }

    pub fn get_pos(&self) -> cgmath::Point3<f32> {
        self.pos
    }

    pub fn get_rendered_chunk_range(&self) -> [i32; 6] {
        let halfed_hrd = self
            .horizontal_render_distance
            .to_f32()
            .unwrap()
            .div_euclid(2.0);
        let halfed_vrd = self
            .vertical_render_distance
            .to_f32()
            .unwrap()
            .div_euclid(2.0);

        let cx = self.pos.x.div_euclid(CHUNK_SIZE as f32);
        let cy = self.pos.y.div_euclid(CHUNK_SIZE as f32);
        let cz = self.pos.z.div_euclid(CHUNK_SIZE as f32);

        let min_cx = (cx - halfed_hrd).floor().to_i32().unwrap();
        let max_cx = (cx + halfed_hrd).floor().to_i32().unwrap();
        let min_cy = (cy - halfed_vrd).floor().to_i32().unwrap();
        let max_cy = (cy + halfed_vrd).floor().to_i32().unwrap();
        let min_cz = (cz - halfed_hrd).floor().to_i32().unwrap();
        let max_cz = (cz + halfed_hrd).floor().to_i32().unwrap();

        return [min_cx, max_cx, min_cy, max_cy, min_cz, max_cz];
    }

    pub fn get_rendered_chunk_number(&self) -> u32 {
        let [min_cx, max_cx, min_cy, max_cy, min_cz, max_cz] = self.get_rendered_chunk_range();
        return ((max_cx - min_cx) * (max_cy - min_cy) * (max_cz - min_cz))
            .to_u32()
            .unwrap_or(1);
    }
}
