use crate::physics::Body;
use ggez::glam::Vec2;

/// All tunable parameters for the simulation.
pub struct SimConfig {
    pub particle_count: usize,
    pub dt: f32,

    pub min_mass: f32,
    pub max_mass: f32,
    pub base_radius: f32,

    pub spawn_width: f32,
    pub spawn_height: f32,

    pub min_gravity: f32,
    pub max_gravity: f32,

    pub map_width: f32,
    pub map_height: f32,
}
impl SimConfig {
    pub fn default() -> Self {
        Self {
            particle_count: 300,
            dt: 0.5,

            min_mass: 0.5,
            max_mass: 5.0,
            base_radius: 1.0,

            spawn_width: 3200.0,
            spawn_height: 1800.0,

            min_gravity: 0.05,
            max_gravity: 0.5,

            map_width: 3200.0,
            map_height: 1800.0,
        }
    }
    /// Spawns a list of randomized bodies
    pub fn spawn_bodies(&self) -> Vec<Body> {
        let mut bodies = Vec::with_capacity(self.particle_count);

        for _ in 0..self.particle_count {
            // random mass
            let mass = rand::random::<f32>() * (self.max_mass - self.min_mass) + self.min_mass;

            // random gravity
            let gravity = rand::random::<f32>() * (self.max_gravity - self.min_gravity) * mass;

            // radius scales with mass
            let radius = mass * self.base_radius;

            bodies.push(Body {
                pos: Vec2::new(
                    rand::random::<f32>() * self.spawn_width - (self.spawn_width / 2.0),
                    rand::random::<f32>() * self.spawn_height - (self.spawn_height / 2.0),
                ),
                vel: Vec2::ZERO,
                mass,
                gravity,
                radius,
            });
        }

        bodies
    }
}
