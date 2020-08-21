use crate::math::vector::Vector3;
use crate::world::world::World;

pub struct Thing {
    pub position: Vector3,
    pub delta: Vector3,
    pub rotation: f32,
    pub size: f32,
    pub height: f32,
    pub health: i32,
    pub speed: f32,
    pub ground: bool,
    pub sector: usize,
}

pub trait Updatable {
    fn update(&mut self);
}

impl Thing {
    pub fn new(world: &World, x: f32, z: f32, size: f32, height: f32) -> Self {
        let sector = world.find_sector(x, z).unwrap();
        let y = sector.floor;
        Thing {
            position: Vector3::new(x, y, z),
            delta: Vector3::default(),
            rotation: 0.0,
            size,
            height,
            health: 0,
            speed: 0.0,
            ground: true,
            sector: sector.index,
        }
    }
}

impl Updatable for Thing {
    fn update(&mut self) {}
}
