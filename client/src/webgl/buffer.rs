use sigil::render::buffer::RenderBuffer;

pub struct WebGlRenderBuffer {
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    pub buffer: RenderBuffer,
}

impl WebGlRenderBuffer {
    pub fn new(position: usize, color: usize, texture: usize, normal: usize, bone: usize, vertices: usize, indices: usize) -> Self {
        let buffer = RenderBuffer::new(position, color, texture, normal, bone, vertices, indices);
        WebGlRenderBuffer {
            vao: 0,
            vbo: 0,
            ebo: 0,
            buffer,
        }
    }

    pub fn zero(&mut self) {
        self.buffer.zero();
    }
}
