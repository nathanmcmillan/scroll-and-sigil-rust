use crate::math::vector::Vector2;
use crate::math::vector::Vector3;
use crate::world::world::World;

#[derive(Default)]
pub struct Decal {
    pub one: Vector3,
    pub two: Vector3,
    pub three: Vector3,
    pub four: Vector3,
    pub u: Vector2,
    pub v: Vector2,
    pub s: Vector2,
    pub t: Vector2,
    pub texture: usize,
}

impl Decal {
    pub fn new(_world: &World) -> Self {
        let decal = Default::default();
        // add to world
        decal
    }
}
