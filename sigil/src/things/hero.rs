use crate::things::thing::Thing;
use crate::world::world::World;

pub struct Hero {
    pub thing: Thing,
}

impl Hero {
    pub fn new(world: &World, x: f32, z: f32) -> Self {
        let thing = Thing::new(world, x, z, 0.0, 0.025, 1.76);
        Hero { thing }
    }
}
