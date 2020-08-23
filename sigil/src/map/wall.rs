use crate::math::vector::Vector2;

pub struct Wall {
    pub a: Vector2,
    pub b: Vector2,
    pub normal: Vector2,
    pub texture: i32,
    pub floor: f32,
    pub ceiling: f32,
    pub u: f32,
    pub v: f32,
    pub s: f32,
    pub t: f32,
}

impl Wall {
    pub fn new(a: Vector2, b: Vector2, texture: i32) -> Self {
        Wall {
            a,
            b,
            normal: a.normal(b),
            texture,
            floor: 0.0,
            ceiling: 0.0,
            u: 0.0,
            v: 0.0,
            s: 0.0,
            t: 0.0,
        }
    }
    pub fn update(&mut self, floor: f32, ceiling: f32, u: f32, v: f32, s: f32, t: f32) {
        self.floor = floor;
        self.ceiling = ceiling;
        self.u = u;
        self.v = v;
        self.s = s;
        self.t = t;
    }
}
