use crate::math::util::float_eq;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }
    pub fn normalize(&mut self) {
        let magnitude = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        let multiple = 1.0 / magnitude;
        self.x *= multiple;
        self.y *= multiple;
        self.z *= multiple;
    }
    pub fn cross(&self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }
    pub fn eq(&self, other: Vector2) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y)
    }
    pub fn mul(&self, scalar: f32) -> Vector2 {
        Vector2::new(self.x * scalar, self.y * scalar)
    }
    pub fn normal(&self, other: Vector2) -> Vector2 {
        let x = self.y - other.y;
        let y = -(self.x - other.x);
        let m = (x * x + y * y).sqrt();
        Vector2::new(x / m, y / m)
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Vector2::new(0.0, 0.0)
    }
}
