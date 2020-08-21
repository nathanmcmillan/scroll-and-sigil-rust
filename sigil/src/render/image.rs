pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pointer: usize,
}

impl Image {
    pub fn new(width: u32, height: u32, pointer: usize) -> Self {
        Image { width, height, pointer }
    }
}
