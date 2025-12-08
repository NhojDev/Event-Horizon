use crate::physics::Body;
use ggez::glam::Vec2;

pub struct Player {
    pub id: usize, // index into bodies[]
}

impl Player {
    // Create a new player Body and return both the player struct AND the body.
    pub fn new() -> (Self, Body) {
        let mass = 1.5;
        let base_radius = 1.0;
        let radius = mass * base_radius;
        let gravity = 2.0 * mass;

        let body = Body {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            mass,
            radius,
            gravity,
        };

        (Self { id: 0 }, body)
    }
}
