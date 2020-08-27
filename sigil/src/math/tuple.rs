#[derive(Copy, Clone)]
pub struct Tuple2 {
    pub x: i32,
    pub y: i32,
}

impl Tuple2 {
    pub fn new(x: i32, y: i32) -> Self {
        Tuple2 { x, y }
    }
}

impl Default for Tuple2 {
    fn default() -> Self {
        Tuple2::new(0, 0)
    }
}
