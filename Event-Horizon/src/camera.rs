use ggez::glam::Vec2;

pub struct Camera {
    pub zoom: f32,     // zoom factor
    pub offset: Vec2,  // world offset (panning)
}

impl Camera {
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            offset: Vec2::ZERO,
        }
    }

    pub fn apply(&self, pos: Vec2) -> Vec2 {
        (pos + self.offset) * self.zoom
    }
}