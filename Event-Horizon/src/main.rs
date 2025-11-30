mod physics;
mod sim_config;

use physics::{Body, simulate_step};
use sim_config::SimConfig;

use ggez::conf::WindowMode;
use ggez::{
    Context, ContextBuilder, GameResult, event,
    glam::Vec2,
    graphics::{self, Color, DrawMode, Mesh},
    input::keyboard::KeyCode,
    input::keyboard::KeyInput,
};

// ---------------------------
// CAMERA
// ---------------------------
struct Camera {
    zoom: f32,
    offset: Vec2,
}

impl Camera {
    fn new() -> Self {
        Self {
            zoom: 1.0,
            offset: Vec2::ZERO,
        }
    }

    fn apply(&self, world: Vec2) -> Vec2 {
        (world + self.offset) * self.zoom
    }
}

// ---------------------------
// GAME STATE
// ---------------------------
struct GameState {
    bodies: Vec<Body>,
    camera: Camera,
    config: SimConfig,
    player_id: usize, // <--- new
}

impl GameState {
    pub fn new() -> Self {
        let config = SimConfig::default();

        // -----------------------------------------
        // Spawn regular particles
        // -----------------------------------------
        let mut bodies = config.spawn_bodies();

        // -----------------------------------------
        // Add Player as a physics body
        // -----------------------------------------
        let player_body = Body {
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::ZERO,
            mass: 15.0,
            radius: 4.0,
            gravity: 0.0,
        };

        // Save the index so we can control/draw it later
        let player_id = bodies.len();
        bodies.push(player_body);

        // -----------------------------------------
        // Return GameState
        // -----------------------------------------
        Self {
            bodies,
            camera: Camera::new(),
            config,
            player_id,
        }
    }
}

// ---------------------------
// EVENT HANDLER
// ---------------------------
impl event::EventHandler for GameState {
    // UPDATE
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        simulate_step(&mut self.bodies, self.config.dt);

        let p = &mut self.bodies[self.player_id];

        let half_w = 400.0;
        let half_h = 300.0;

        let left = -half_w;
        let right = half_w;
        let top = -half_h;
        let bottom = half_h;

        // LEFT WALL
        if p.pos.x - p.radius < left {
            p.pos.x = left + p.radius;
            p.vel.x = 0.0;
        }

        // RIGHT WALL
        if p.pos.x + p.radius > right {
            p.pos.x = right - p.radius;
            p.vel.x = 0.0;
        }

        // TOP WALL
        if p.pos.y - p.radius < top {
            p.pos.y = top + p.radius;
            p.vel.y = 0.0;
        }

        // BOTTOM WALL
        if p.pos.y + p.radius > bottom {
            p.pos.y = bottom - p.radius;
            p.vel.y = 0.0;
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // --------------------------------
        // Draw all bodies
        // --------------------------------
        for b in &self.bodies {
            let cam_pos = self.camera.apply(b.pos);
            let screen_pos = [cam_pos.x + 400.0, cam_pos.y + 300.0];

            let radius = b.radius * self.camera.zoom.recip();

            let circle =
                Mesh::new_circle(ctx, DrawMode::fill(), screen_pos, radius, 0.1, Color::RED)?;

            canvas.draw(&circle, Vec2::ZERO);
        }

        // --------------------------------
        // Draw Player
        // --------------------------------
        let player = &self.bodies[self.player_id];
        let p = self.camera.apply(player.pos);

        let screen_p = [p.x + 400.0, p.y + 300.0];
        let pr = player.radius * self.camera.zoom.recip();

        let player_circle =
            Mesh::new_circle(ctx, DrawMode::fill(), screen_p, pr, 0.1, Color::GREEN)?;

        canvas.draw(&player_circle, Vec2::ZERO);

        // --------------------------------
        // Finish frame
        // --------------------------------
        canvas.finish(ctx)?;
        Ok(())
    }

    // ZOOM with mouse wheel
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        let speed = 0.12;

        if y > 0.0 {
            self.camera.zoom *= 1.0 + speed;
        } else if y < 0.0 {
            self.camera.zoom *= 1.0 - speed;
        }

        self.camera.zoom = self.camera.zoom.clamp(0.1, 6.0);

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();

        // Get player body
        let player = &mut self.bodies[self.player_id];

        // -------------------------------------
        // Smooth acceleration (no sharp turns)
        // -------------------------------------
        let accel_strength = 120.0 * dt;
        let mut accel = Vec2::ZERO;

        match input.keycode {
            Some(KeyCode::W) => accel.y -= 1.0,
            Some(KeyCode::S) => accel.y += 1.0,
            Some(KeyCode::A) => accel.x -= 1.0,
            Some(KeyCode::D) => accel.x += 1.0,
            _ => {}
        }

        // Normalize diagonal input
        if accel.length_squared() > 0.0 {
            accel = accel.normalize() * accel_strength;
        }

        // Apply acceleration to velocity
        player.vel += accel;

        // -------------------------------------
        // SPEED CAP
        // -------------------------------------
        let max_speed = 10.0;
        let speed = player.vel.length();
        if speed > max_speed {
            player.vel = player.vel.normalize() * max_speed;
        }

        Ok(())
    }
}

// ---------------------------
// MAIN
// ---------------------------
fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("event-horizon", "johpham")
        .window_mode(WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    let state = GameState::new();
    event::run(ctx, event_loop, state)
}
