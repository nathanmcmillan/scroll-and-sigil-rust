use crate::game::camera::Camera;
use crate::io::input::Input;
use crate::map::line::Line;
use crate::map::sector::Sector;
use crate::math::vector::Vector2;
use crate::things::hero::Hero;
use crate::world::world::World;

const SECTOR_NO_SURFACE: i32 = -1;
const LINE_NO_WALL: i32 = -1;
const TEXTURE_GRASS: i32 = 0;
const TEXTURE_STONE: i32 = 1;

pub struct Game {
    pub input: Input,
    pub world: World,
    pub camera: Camera,
}

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
    world.add_sector(sector);
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
    world.add_sector(sector);
}

fn place(world: &mut World) {
    place_grass(world);
    place_house(world, 10.0, 10.0);
    place_house(world, 40.0, 60.0);
    world.build();
    Hero::new(world, 10.0, 40.0);
    // Baron::new(world, 8.0, 45.0);
    // Blood::new(world, 5.0, 1.0, 30.0);
    // Tree::new(world, 14.0, 42.0);
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();
        place(&mut world);
        Game {
            world,
            camera: Camera::new(0.0, 0.0, 0.0, 0.0, 0.0, 6.0),
            input: Input::new(),
        }
    }
    pub fn update(&mut self) {
        self.world.update();

        let input = &self.input;
        let camera = &mut self.camera;
        if input.look_left {
            camera.ry -= 0.05;
            if camera.ry < 0.0 {
                camera.ry += 2.0 * std::f32::consts::PI;
            }
        }
        if input.look_right {
            camera.ry += 0.05;
            if camera.ry >= 2.0 * std::f32::consts::PI {
                camera.ry -= 2.0 * std::f32::consts::PI;
            }
        }
        if input.look_up {
            camera.rx -= 0.05;
            if camera.rx < 0.0 {
                camera.rx += 2.0 * std::f32::consts::PI;
            }
        }
        if input.look_down {
            camera.rx += 0.05;
            if camera.rx >= 2.0 * std::f32::consts::PI {
                camera.rx -= 2.0 * std::f32::consts::PI;
            }
        }
        let target = &self.world.things[0];
        camera.update_orbit(target);
    }
}
