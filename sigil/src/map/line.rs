use crate::map::wall::Wall;
use crate::math::util::float_zero;
use crate::math::vector::Vector2;

pub struct Intersect {
    pub x: f32,
    pub y: f32,
    pub ok: bool,
}

#[derive(Copy, Clone)]
pub struct Line {
    pub index: usize,
    pub a: Vector2,
    pub b: Vector2,
    pub normal: Vector2,
    pub top: Option<Wall>,
    pub middle: Option<Wall>,
    pub bottom: Option<Wall>,
    pub plus: Option<usize>,
    pub minus: Option<usize>,
}

impl Line {
    pub fn new(low: i32, mid: i32, up: i32, a: Vector2, b: Vector2) -> Self {
        Line {
            index: 0,
            a,
            b,
            normal: a.normal(b),
            top: if up >= 0 { Some(Wall::new(a, b, up)) } else { Option::None },
            middle: if mid >= 0 { Some(Wall::new(a, b, mid)) } else { Option::None },
            bottom: if low >= 0 { Some(Wall::new(a, b, low)) } else { Option::None },
            plus: Option::None,
            minus: Option::None,
        }
    }
    pub fn update_sectors(&mut self, plus: Option<usize>, minus: Option<usize>) {
        self.plus = plus;
        self.minus = minus;
    }
    pub fn intersect(&self, with: &Line) -> Intersect {
        let a1: f32 = self.b.y - self.a.y;
        let b1: f32 = self.a.x - self.b.x;
        let c1: f32 = (self.b.x * self.a.y) - (self.a.x * self.b.y);

        let r3: f32 = (a1 * with.a.x) + (b1 * with.a.y) + c1;
        let r4: f32 = (a1 * with.b.x) + (b1 * with.b.y) + c1;
        if !float_zero(r3) && !float_zero(r4) && (r3 * r4 >= 0.0) {
            return Intersect { x: 0.0, y: 0.0, ok: false };
        }

        // TODO

        let x: f32 = 0.0;
        let y: f32 = 0.0;

        Intersect { x, y, ok: true }
    }
}

pub struct LineSystem {
    pub lines: Vec<Line>,
}

impl LineSystem {
    pub fn new() -> Self {
        LineSystem { lines: Vec::new() }
    }
    pub fn push(&mut self, mut line: Line) -> usize {
        let index = self.lines.len();
        line.index = index;
        self.lines.push(line);
        index
    }
    pub fn delete(&mut self, index: usize) {
        self.lines.remove(index);
    }
    pub fn get(&self, index: usize) -> &Line {
        &self.lines[index]
    }
    pub fn get_mutable(&mut self, index: usize) -> &mut Line {
        &mut self.lines[index]
    }
    pub fn size(&self) -> usize {
        self.lines.len()
    }
}
