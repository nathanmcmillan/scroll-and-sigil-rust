use crate::render::buffer::RenderBuffer;

pub fn index3(b: &mut RenderBuffer) {
    let pos = b.index_position;
    let offset = b.index_offset;
    let indices = &mut b.indices;
    indices[pos] = offset;
    indices[pos + 1] = offset + 1;
    indices[pos + 2] = offset + 2;
    b.index_position = pos + 3;
    b.index_offset = offset + 3;
}

pub fn index4(b: &mut RenderBuffer) {
    let pos = b.index_position;
    let offset = b.index_offset;
    let indices = &mut b.indices;
    indices[pos] = offset;
    indices[pos + 1] = offset + 1;
    indices[pos + 2] = offset + 2;
    b.index_position = pos + 3;
    b.index_offset = offset + 3;
}

pub fn screen(b: &mut RenderBuffer, x: f32, y: f32, width: f32, height: f32) {
    let pos = b.vertex_position;
    let vertices = &mut b.vertices;
    vertices[pos] = x;
    vertices[pos + 1] = y;
    vertices[pos + 2] = x + width;
    vertices[pos + 3] = y;
    vertices[pos + 4] = x + width;
    vertices[pos + 5] = y + height;
    vertices[pos + 6] = x;
    vertices[pos + 7] = y + height;
    b.vertex_position = pos + 8;
    index4(b);
}
