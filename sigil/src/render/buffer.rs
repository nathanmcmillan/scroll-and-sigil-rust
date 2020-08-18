pub struct RenderBuffer {
    pub position: usize,
    pub color: usize,
    pub texture: usize,
    pub normal: usize,
    pub bone: usize,
    pub vertex_position: usize,
    pub vertices: Vec<f32>,
    pub index_position: usize,
    pub index_offset: u32,
    pub indices: Vec<u32>,
}

impl RenderBuffer {
    pub fn new(position: usize, color: usize, texture: usize, normal: usize, bone: usize, vertices: usize, indices: usize) -> Self {
        RenderBuffer {
            position,
            color,
            texture,
            normal,
            bone,
            vertex_position: 0,
            vertices: Vec::with_capacity(vertices * (position + color + texture + normal + bone)),
            index_position: 0,
            index_offset: 0,
            indices: Vec::with_capacity(indices),
        }
    }

    pub fn zero(&mut self) {
        self.index_offset = 0;
        self.index_position = 0;
        self.vertex_position = 0;
    }
}
