use ggez::glam::Vec2;

#[derive(Clone, Debug)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub radius: f32,
}

pub fn simulate_step(bodies: &mut [Body], dt: f32, g: f32) {
    let n = bodies.len();
    let mut acc = vec![Vec2::ZERO; n];

    let softening = 5.0;
    let damping = 0.98;

    for i in 0..n {
        for j in (i + 1)..n {
            let dir = bodies[j].pos - bodies[i].pos;

            let dist_sq = dir.length_squared() + softening * softening;
            let dist = dist_sq.sqrt();

            let force_mag = g * bodies[i].mass * bodies[j].mass / dist_sq;
            let force = (dir / dist) * force_mag;

            acc[i] += force / bodies[i].mass;
            acc[j] -= force / bodies[j].mass;

            // To prevent particles from freaking out when they get too close.
            let actual_dist = dir.length();
            let min_dist = bodies[i].radius + bodies[j].radius;

            if actual_dist < min_dist {
                let push = (min_dist - actual_dist) * 0.5;
                let norm = dir.normalize_or_zero();

                bodies[i].pos -= norm * push;
                bodies[j].pos += norm * push;
            }
        }
    }

    for i in 0..n {
        bodies[i].vel += acc[i] * dt;
        bodies[i].vel *= damping;

        let vel = bodies[i].vel;
        bodies[i].pos += vel * dt;
    }
}
