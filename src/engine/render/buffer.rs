use wgpu::Buffer;

pub struct BufferData {
    pub vertex_buffer: Option<Buffer>,
    pub index_buffer: Option<Buffer>,
    pub vertex_number: Option<u32>,
    pub index_number: Option<u32>
}

impl BufferData {
    pub fn empty() -> Self {
        Self {
            vertex_buffer: None,
            vertex_number: None,
            index_buffer: None,
            index_number: None,
        }
    }
}