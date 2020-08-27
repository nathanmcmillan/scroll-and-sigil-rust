use crate::math::tuple::Tuple2;
use crate::math::vector::Vector2;
use crate::math::vector::Vector3;
use crate::world::world::World;
use crate::world::world::WORLD_CELL_SHIFT;

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
    pub fn collision(&self, b: &Thing) -> bool {
        let block = self.size + b.size;
        (self.position.x - b.position.x).abs() <= block && (self.position.z - b.position.z).abs() <= block
    }
}

impl Updatable for Thing {
    fn update(&mut self) {}
}
