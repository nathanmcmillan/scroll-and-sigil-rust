use crate::map::triangle::Triangle;
use crate::map::wall::Wall;
use crate::render::buffer::RenderBuffer;
use crate::render::render::index3;
use crate::render::render::index4;

pub fn triangle(b: &mut RenderBuffer, t: &Triangle) {
    let pos = b.vertex_position;
    let vertices = &mut b.vertices;

    vertices[pos] = t.c.x;
    vertices[pos + 1] = t.height;
    vertices[pos + 2] = t.c.y;
    vertices[pos + 3] = t.uvc.x;
    vertices[pos + 4] = t.uvc.y;
    vertices[pos + 5] = 0.0;
    vertices[pos + 6] = t.normal;
    vertices[pos + 7] = 0.0;

    vertices[pos + 8] = t.b.x;
    vertices[pos + 9] = t.height;
    vertices[pos + 10] = t.b.y;
    vertices[pos + 11] = t.uvb.x;
    vertices[pos + 12] = t.uvb.y;
    vertices[pos + 13] = 0.0;
    vertices[pos + 14] = t.normal;
    vertices[pos + 15] = 0.0;

    vertices[pos + 16] = t.a.x;
    vertices[pos + 17] = t.height;
    vertices[pos + 18] = t.a.y;
    vertices[pos + 19] = t.uva.x;
    vertices[pos + 20] = t.uva.y;
    vertices[pos + 21] = 0.0;
    vertices[pos + 22] = t.normal;
    vertices[pos + 23] = 0.0;

    b.vertex_position = pos + 24;
    index3(b);
}

pub fn wall(b: &mut RenderBuffer, w: &Wall) {
    let pos = b.vertex_position;
    let vertices = &mut b.vertices;

    vertices[pos] = w.a.x;
    vertices[pos + 1] = w.ceiling;
    vertices[pos + 2] = w.a.y;
    vertices[pos + 3] = w.u;
    vertices[pos + 4] = w.t;
    vertices[pos + 5] = w.normal.x;
    vertices[pos + 6] = 0.0;
    vertices[pos + 7] = w.normal.y;

    vertices[pos + 8] = w.a.x;
    vertices[pos + 9] = w.floor;
    vertices[pos + 10] = w.a.y;
    vertices[pos + 11] = w.u;
    vertices[pos + 12] = w.v;
    vertices[pos + 13] = w.normal.x;
    vertices[pos + 14] = 0.0;
    vertices[pos + 15] = w.normal.y;

    vertices[pos + 16] = w.b.x;
    vertices[pos + 17] = w.floor;
    vertices[pos + 18] = w.b.y;
    vertices[pos + 19] = w.s;
    vertices[pos + 20] = w.v;
    vertices[pos + 21] = w.normal.x;
    vertices[pos + 22] = 0.0;
    vertices[pos + 23] = w.normal.y;

    vertices[pos + 24] = w.b.x;
    vertices[pos + 25] = w.ceiling;
    vertices[pos + 26] = w.b.y;
    vertices[pos + 27] = w.s;
    vertices[pos + 28] = w.t;
    vertices[pos + 29] = w.normal.x;
    vertices[pos + 30] = 0.0;
    vertices[pos + 31] = w.normal.y;

    b.vertex_position = pos + 32;
    index4(b);
}
