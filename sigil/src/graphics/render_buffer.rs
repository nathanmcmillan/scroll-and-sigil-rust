pub struct RenderBufferData {
    pub position: u32,
    pub color: u32,
    pub texture: u32,
    pub normal: u32,
    pub bone: u32,
    pub vertex_position: usize,
    pub vertices: Vec<f32>,
    pub index_position: usize,
    pub index_offset: usize,
    pub indices: Vec<u32>,
}

pub trait RenderBuffer {
    fn init(&mut self);
    fn zero(&mut self);
}
