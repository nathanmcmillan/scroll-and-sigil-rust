use crate::math::vector::Vector3;
use crate::world::world::World;

pub struct Particle {
    pub position: Vector3,
    pub delta: Vector3,
    pub size: f32,
    pub height: f32,
    pub sector: usize,
}

impl Particle {
    pub fn new(world: &World, x: f32, y: f32, z: f32, size: f32, height: f32) -> Self {
        let sector = world.find_sector(x, z).unwrap();
        Particle {
            position: Vector3::new(x, y, z),
            delta: Vector3::default(),
            size,
            height,
            sector: sector.index,
        }
    }
}
