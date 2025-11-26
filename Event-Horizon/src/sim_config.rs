use crate::physics::Body;
use ggez::glam::Vec2;

/// All tunable parameters for the simulation.
pub struct SimConfig {
    pub particle_count: usize,
    pub dt: f32,
    pub g: f32,
    pub mass: f32,
    pub radius: f32,
    pub spawn_width: f32,
    pub spawn_height: f32,
}

impl SimConfig {
    pub fn default() -> Self {
        Self {
            particle_count: 500,
            dt: 0.5,
            g: 0.1,
            mass: 10.0,
            radius: 3.0,
            spawn_width: 800.0,
            spawn_height: 600.0,
        }
    }

    /// Spawns a list of randomized bodies
    pub fn spawn_bodies(&self) -> Vec<Body> {
        let mut bodies = Vec::with_capacity(self.particle_count);

        for _ in 0..self.particle_count {
            bodies.push(Body {
                pos: Vec2::new(
                    rand::random::<f32>() * self.spawn_width - (self.spawn_width / 2.0),
                    rand::random::<f32>() * self.spawn_height - (self.spawn_height / 2.0),
                ),
                vel: Vec2::ZERO,
                radius: self.radius,
                mass: self.mass,
            });
        }

        bodies
    }
}
