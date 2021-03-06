use crate::map::triangle::Triangle;
use crate::math::vector::Vector2;

pub struct Sector {
    pub index: usize,
    pub bottom: f32,
    pub floor: f32,
    pub ceiling: f32,
    pub top: f32,
    pub floor_texture: i32,
    pub ceiling_texture: i32,
    pub vecs: Vec<Vector2>,
    pub lines: Vec<usize>,
    pub triangles: Vec<Triangle>,
    pub inside: Vec<usize>,
    pub outside: Option<usize>,
}

impl Sector {
    pub fn new(bottom: f32, floor: f32, ceiling: f32, top: f32, floor_texture: i32, ceiling_texture: i32, vecs: Vec<Vector2>, lines: Vec<usize>) -> Self {
        Sector {
            index: 0,
            bottom,
            floor,
            ceiling,
            top,
            floor_texture,
            ceiling_texture,
            vecs,
            lines,
            triangles: Vec::new(),
            inside: Vec::new(),
            outside: Option::None,
        }
    }
    pub fn update_triangles(&mut self, triangles: Vec<Triangle>) {
        self.triangles = triangles;
    }
    pub fn contains(&self, x: f32, y: f32) -> bool {
        let mut odd: bool = false;
        let vecs: &Vec<Vector2> = &self.vecs;
        let count: usize = vecs.len();
        let mut k: usize = count - 1;
        for i in 0..count {
            let a: &Vector2 = &vecs[i];
            let b: &Vector2 = &vecs[k];
            if (a.y > y) != (b.y > y) {
                let value: f32 = (b.x - a.x) * (y - a.y) / (b.y - a.y) + a.x;
                if x < value {
                    odd = !odd;
                }
            }
            k = i;
        }
        odd
    }
    pub fn has_floor(&self) -> bool {
        self.floor_texture >= 0
    }
    pub fn has_ceiling(&self) -> bool {
        self.ceiling_texture >= 0
    }
}

pub fn find<'s>(sectors: &'s Vec<Sector>, sector: &'s Sector, x: f32, y: f32) -> &'s Sector {
    for i in sector.inside.iter().copied() {
        let sector = &sectors[i];
        if sector.contains(x, y) {
            return find(sectors, sector, x, y);
        }
    }
    return sector;
}
