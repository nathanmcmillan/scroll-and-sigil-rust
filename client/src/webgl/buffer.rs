use sigil::render::buffer::RenderBuffer;
use web_sys::WebGlBuffer;
use web_sys::WebGlVertexArrayObject;

pub struct WebGlRenderBuffer {
    pub vao: Option<WebGlVertexArrayObject>,
    pub vbo: Option<WebGlBuffer>,
    pub ebo: Option<WebGlBuffer>,
    pub buffer: RenderBuffer,
}

impl WebGlRenderBuffer {
    pub fn new(position: usize, color: usize, texture: usize, normal: usize, bone: usize, vertices: usize, indices: usize) -> Self {
        let buffer = RenderBuffer::new(position, color, texture, normal, bone, vertices, indices);
        WebGlRenderBuffer {
            vao: Option::None,
            vbo: Option::None,
            ebo: Option::None,
            buffer,
        }
    }

    pub fn zero(&mut self) {
        self.buffer.zero();
    }
}
