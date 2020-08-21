#[derive(Default)]
pub struct Input {
    pub look_left: bool,
    pub look_right: bool,
    pub look_up: bool,
    pub look_down: bool,
}

impl Input {
    pub fn new() -> Self {
        Default::default()
    }
}
