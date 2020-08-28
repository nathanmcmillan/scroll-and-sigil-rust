use crate::map::line::Line;
use crate::math::tuple::Tuple2;
use crate::math::util::float_zero;
use crate::math::vector::Vector2;
use crate::math::vector::Vector3;
use crate::world::world::World;
use crate::world::world::WORLD_CELL_SHIFT;

const GRAVITY: f32 = 0.028;
const FRICTION: f32 = 0.88;

pub struct Thing {
    pub index: usize,
    pub position: Vector3,
    pub previous: Vector2,
    pub delta: Vector3,
    pub rotation: f32,
    pub size: f32,
    pub height: f32,
    pub health: i32,
    pub speed: f32,
    pub ground: bool,
    pub sector: usize,
    pub min: Tuple2,
    pub max: Tuple2,
}

pub trait Updatable {
    fn update(&mut self);
}

impl Thing {
    pub fn new(world: &World, x: f32, z: f32, rotation: f32, size: f32, height: f32) -> Self {
        let sector = world.find_sector(x, z).unwrap();
        let y = sector.floor;
        Thing {
            index: 0,
            position: Vector3::new(x, y, z),
            previous: Vector2::new(x, z),
            delta: Vector3::default(),
            rotation,
            size,
            height,
            health: 0,
            speed: 0.0,
            ground: true,
            sector: sector.index,
            min: Tuple2::default(),
            max: Tuple2::default(),
        }
    }
    pub fn add_to_cells(&mut self, world: &mut World) {
        let size = self.size;
        let c_min = (self.position.x - size) as i32 >> WORLD_CELL_SHIFT;
        let c_max = (self.position.x + size) as i32 >> WORLD_CELL_SHIFT;
        let r_min = (self.position.z - size) as i32 >> WORLD_CELL_SHIFT;
        let r_max = (self.position.z + size) as i32 >> WORLD_CELL_SHIFT;
        for r in r_min..=r_max {
            for c in c_min..=c_max {
                world.cells[c as usize + r as usize * world.cell_columns].add_thing(self.index);
            }
        }
        self.min.x = c_min;
        self.max.x = c_max;
        self.min.y = r_min;
        self.max.y = r_max;
    }
    pub fn remove_from_cells(&mut self, world: &mut World) {
        for r in self.min.y..=self.max.y {
            for c in self.min.x..=self.max.x {
                world.cells[c as usize + r as usize * world.cell_columns].remove_thing(self.index);
            }
        }
    }
    pub fn collision(&self, b: &Thing) -> bool {
        let block = self.size + b.size;
        (self.position.x - b.position.x).abs() <= block && (self.position.z - b.position.z).abs() <= block
    }
    pub fn resolve_collision(&mut self, b: &Thing) {
        let block = self.size + b.size;
        if (self.position.x - b.position.x).abs() > block || (self.position.z - b.position.z).abs() > block {
            return;
        }
        if (self.previous.x - b.position.x).abs() > (self.previous.y - b.position.z).abs() {
            if self.previous.x - b.position.x < 0.0 {
                self.position.x = b.position.x - block;
            } else {
                self.position.x = b.position.x + block;
            }
            self.delta.x = 0.0;
        } else {
            if self.previous.y - b.position.z < 0.0 {
                self.position.z = b.position.z - block;
            } else {
                self.position.z = b.position.z + block;
            }
            self.delta.z = 0.0;
        }
    }
    pub fn line_collision(&mut self, world: &World, line: &Line) {
        let size = self.size;

        let vx = line.b.x - line.a.x;
        let vz = line.b.y - line.a.y;

        let wx = self.position.x - line.a.x;
        let wz = self.position.z - line.a.y;

        let mut t = (wx * vx + wz * vz) / (vx * vx + vz * vz);

        let mut endpoint = false;

        if t < 0.0 {
            t = 0.0;
            endpoint = true;
        } else if t > 1.0 {
            t = 1.0;
            endpoint = true;
        }

        let mut px = line.a.x + vx * t;
        let mut pz = line.a.y + vz * t;

        px -= self.position.x;
        pz -= self.position.z;

        if (px * px + pz * pz) > size * size {
            return;
        }

        let mut collision = false;

        if line.middle.is_some() {
            collision = true;
        } else if self.position.y + self.height > world.get_sector(line.plus.unwrap()).ceiling
            || self.position.y + 1.0 < world.get_sector(line.plus.unwrap()).floor
        {
            collision = true;
        }

        if collision {
            if self.sector == line.plus.unwrap() {
                return;
            }

            let overlap;

            let normal_x;
            let normal_z;

            if endpoint {
                let mut ex = -px;
                let mut ez = -pz;

                let em = (ex * ex + ez * ez).sqrt();

                ex /= em;
                ez /= em;

                overlap = ((px + size * ex) * (px + size * ex) + (pz + size * ez) * (pz + size * ez)).sqrt();

                normal_x = ex;
                normal_z = ez;
            } else {
                overlap = ((px + size * line.normal.x) * (px + size * line.normal.x) + (pz + size * line.normal.y) * (pz + size * line.normal.y)).sqrt();

                normal_x = line.normal.x;
                normal_z = line.normal.y;
            }

            self.position.x += normal_x * overlap;
            self.position.z += normal_z * overlap;
        }
    }
    pub fn nop_update(&self) {}
    pub fn integrate(&mut self, world: &mut World) {
        if self.ground {
            self.delta.x *= FRICTION;
            self.delta.z *= FRICTION;
        }

        if !float_zero(self.delta.x) || !float_zero(self.delta.z) {
            self.previous.x = self.position.x;
            self.previous.y = self.position.z;

            self.position.x += self.delta.x;
            self.position.z += self.delta.z;

            self.remove_from_cells(world);

            let size = self.size;
            let c_min = (self.position.x - size) as i32 >> WORLD_CELL_SHIFT;
            let c_max = (self.position.x + size) as i32 >> WORLD_CELL_SHIFT;
            let r_min = (self.position.z - size) as i32 >> WORLD_CELL_SHIFT;
            let r_max = (self.position.z + size) as i32 >> WORLD_CELL_SHIFT;

            self.add_to_cells(world);
        }

        if !self.ground || !float_zero(self.delta.y) {
            self.delta.y -= GRAVITY;
            self.position.y += self.delta.y;
            let floor = world.get_sector(self.sector).floor;
            if self.position.y < floor {
                self.ground = true;
                self.delta.y = 0.0;
                self.position.y = floor;
            } else {
                self.ground = false;
            }
        }
    }
}

impl Updatable for Thing {
    fn update(&mut self) {}
}
