use ggez::glam::Vec2;

#[derive(Clone, Debug)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
}

pub fn simulate_step(bodies: &mut [Body], dt: f32, g: f32) {
    let n = bodies.len();
    let mut acc = vec![Vec2::ZERO; n];

    for i in 0..n {
        for j in (i + 1)..n {
            let dir = bodies[j].pos - bodies[i].pos;
            let dist_sq = dir.length_squared().max(0.01);

            let force_mag = g * bodies[i].mass * bodies[j].mass / dist_sq;
            let force = dir.normalize() * force_mag;

            acc[i] += force / bodies[i].mass;
            acc[j] -= force / bodies[j].mass;
        }
    }

    for i in 0..n {
        bodies[i].vel += acc[i] * dt;
        bodies[i].pos += bodies[i].vel * dt;
    }
}
