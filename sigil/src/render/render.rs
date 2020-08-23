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
    indices[pos + 3] = offset + 2;
    indices[pos + 4] = offset + 3;
    indices[pos + 5] = offset;
    b.index_position = pos + 6;
    b.index_offset = offset + 4;
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

pub fn rectangle(b: &mut RenderBuffer, x: f32, y: f32, width: f32, height: f32, red: f32, green: f32, blue: f32, alpha: f32) {
    let pos = b.vertex_position;
    let vertices = &mut b.vertices;
    vertices[pos] = x;
    vertices[pos + 1] = y;
    vertices[pos + 2] = red;
    vertices[pos + 3] = green;
    vertices[pos + 4] = blue;
    vertices[pos + 5] = alpha;

    vertices[pos + 6] = x + width;
    vertices[pos + 7] = y;
    vertices[pos + 8] = red;
    vertices[pos + 9] = green;
    vertices[pos + 10] = blue;
    vertices[pos + 11] = alpha;

    vertices[pos + 12] = x + width;
    vertices[pos + 13] = y + height;
    vertices[pos + 14] = red;
    vertices[pos + 15] = green;
    vertices[pos + 16] = blue;
    vertices[pos + 17] = alpha;

    vertices[pos + 18] = x;
    vertices[pos + 19] = y + height;
    vertices[pos + 20] = red;
    vertices[pos + 21] = green;
    vertices[pos + 22] = blue;
    vertices[pos + 23] = alpha;
    b.vertex_position = pos + 24;
    index4(b);
}

pub fn image(
    b: &mut RenderBuffer,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
) {
    let pos = b.vertex_position;
    let vertices = &mut b.vertices;
    vertices[pos] = x;
    vertices[pos + 1] = y;
    vertices[pos + 2] = red;
    vertices[pos + 3] = green;
    vertices[pos + 4] = blue;
    vertices[pos + 5] = alpha;
    vertices[pos + 6] = left;
    vertices[pos + 7] = bottom;

    vertices[pos + 8] = x + width;
    vertices[pos + 9] = y;
    vertices[pos + 10] = red;
    vertices[pos + 11] = green;
    vertices[pos + 12] = blue;
    vertices[pos + 13] = alpha;
    vertices[pos + 14] = right;
    vertices[pos + 15] = bottom;

    vertices[pos + 16] = x + width;
    vertices[pos + 17] = y + height;
    vertices[pos + 18] = red;
    vertices[pos + 19] = green;
    vertices[pos + 20] = blue;
    vertices[pos + 21] = alpha;
    vertices[pos + 22] = right;
    vertices[pos + 23] = top;

    vertices[pos + 24] = x;
    vertices[pos + 25] = y + height;
    vertices[pos + 26] = red;
    vertices[pos + 27] = green;
    vertices[pos + 28] = blue;
    vertices[pos + 29] = alpha;
    vertices[pos + 30] = left;
    vertices[pos + 31] = top;
    b.vertex_position = pos + 32;
    index4(b);
}
