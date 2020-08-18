use sigil::map::line::Line;
use sigil::map::sector::Sector;
use sigil::math::vector::Vector2;
use sigil::world::world::World;

const SECTOR_NO_SURFACE: i32 = -1;
const LINE_NO_WALL: i32 = -1;
const TEXTURE_GRASS: i32 = 0;
const TEXTURE_STONE: i32 = 1;

fn place_house(world: &mut World, x: f32, y: f32) {
    const COUNT: usize = 12;
    let mut vecs = Vec::with_capacity(COUNT);
    vecs.push(Vector2::new(x, y));
    vecs.push(Vector2::new(x, y + 20.0));
    vecs.push(Vector2::new(x + 6.0, y + 20.0));
    vecs.push(Vector2::new(x + 6.0, y + 19.0));
    vecs.push(Vector2::new(x + 1.0, y + 19.0));
    vecs.push(Vector2::new(x + 1.0, y + 1.0));
    vecs.push(Vector2::new(x + 19.0, y + 1.0));
    vecs.push(Vector2::new(x + 19.0, y + 19.0));
    vecs.push(Vector2::new(x + 14.0, y + 19.0));
    vecs.push(Vector2::new(x + 14.0, y + 20.0));
    vecs.push(Vector2::new(x + 20.0, y + 20.0));
    vecs.push(Vector2::new(x + 20.0, y));
    let mut lines = Vec::with_capacity(COUNT);
    let mut k: usize = COUNT - 1;
    for i in 0..COUNT {
        lines.push(Line::new(LINE_NO_WALL, TEXTURE_STONE, LINE_NO_WALL, vecs[k], vecs[i]));
        k = i;
    }
    let bottom: f32 = 0.0;
    let floor: f32 = 0.0;
    let ceiling: f32 = 10.0;
    let top: f32 = 0.0;
    let sector = Sector::new(bottom, floor, ceiling, top, TEXTURE_GRASS, SECTOR_NO_SURFACE, vecs, lines);
    world.push_sector(sector);
}

fn place_grass(world: &mut World) {
    let mut vecs = Vec::with_capacity(4);
    vecs.push(Vector2::new(0.0, 0.0));
    vecs.push(Vector2::new(0.0, 50.0));
    vecs.push(Vector2::new(60.0, 50.0));
    vecs.push(Vector2::new(60.0, 0.0));
    let lines = Vec::new();
    let bottom: f32 = 0.0;
    let floor: f32 = 0.0;
    let ceiling: f32 = 10.0;
    let top: f32 = 0.0;
    let sector = Sector::new(bottom, floor, ceiling, top, TEXTURE_GRASS, SECTOR_NO_SURFACE, vecs, lines);
    world.push_sector(sector);
}

pub fn run(world: &mut World) {
    place_grass(world);
    place_house(world, 10.0, 10.0);
    place_house(world, 35.0, 10.0);
    world.build();
}
