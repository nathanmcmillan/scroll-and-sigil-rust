use crate::things::thing::Thing;

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rx: f32,
    pub ry: f32,
    pub radius: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, z: f32, rx: f32, ry: f32, radius: f32) -> Self {
        Camera { x, y, z, rx, ry, radius }
    }

    pub fn update_orbit(&mut self, target: &Thing) {
        let sin_x = self.rx.sin();
        let cos_x = self.rx.cos();
        let sin_y = self.ry.sin();
        let cos_y = self.ry.cos();
        self.x = target.position.x - self.radius * cos_x * sin_y;
        self.y = target.position.y + self.radius * sin_x + target.height;
        self.z = target.position.z + self.radius * cos_x * cos_y;
    }
}
