use crate::physics::Body;
use ggez::glam::Vec2;

pub struct Player {
    pub id: usize, // index into bodies[]
}

impl Player {
    // Create a new player Body and return both the player struct AND the body.
    pub fn new() -> (Self, Body) {
        let mass = 1.5;
        let radius = mass * 1.0;
        let gravity = mass * 2.0;

        let body = Body {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            mass,
            radius,
            gravity,
        };

        (Self { id: 0 }, body)
    }

    /// Called after the player eats a particle
    pub fn absorb(&self, player_body: &mut Body, eaten_mass: f32) {
        player_body.mass += eaten_mass;

        if player_body.mass * 0.1 > player_body.radius {
            player_body.radius = player_body.mass * 0.1;
        }

        player_body.gravity = player_body.mass * 2.0;
    }
}
