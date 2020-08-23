use crate::math::vector::Vector2;

pub struct Triangle {
    pub height: f32,
    pub a: Vector2,
    pub b: Vector2,
    pub c: Vector2,
    pub texture: i32,
    pub uva: Vector2,
    pub uvb: Vector2,
    pub uvc: Vector2,
    pub normal: f32,
}

impl Triangle {
    pub fn new(height: f32, texture: i32, a: Vector2, b: Vector2, c: Vector2, floor: bool, scale: f32) -> Self {
        Triangle {
            height,
            texture,
            a,
            b,
            c,
            uva: Vector2::new(a.x * scale, a.y * scale),
            uvb: Vector2::new(b.x * scale, b.y * scale),
            uvc: Vector2::new(c.x * scale, c.y * scale),
            normal: if floor { 1.0 } else { -1.0 },
        }
    }
}
