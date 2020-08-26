pub struct Sprite {
    pub width: f32,
    pub height: f32,
    pub half_width: f32,
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Sprite {
    pub fn new(
        left: u32,
        top: u32,
        width: u32,
        height: u32,
        offset_x: u32,
        offset_y: u32,
        atlas_inverse_width: f32,
        atlas_inverse_height: f32,
        scale: f32,
    ) -> Self {
        let w = width as f32 * scale;
        Sprite {
            left: left as f32 * atlas_inverse_width,
            top: top as f32 * atlas_inverse_height,
            right: (left + width) as f32 * atlas_inverse_width,
            bottom: (top + height) as f32 * atlas_inverse_height,
            width: w,
            height: height as f32 * scale,
            half_width: w * 0.5,
            offset_x: offset_x as f32 * scale,
            offset_y: offset_y as f32 * scale,
        }
    }
}
