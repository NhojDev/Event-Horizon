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
}

impl GameState {
    pub fn new() -> Self {
        let config = SimConfig::default();

        Self {
            bodies: config.spawn_bodies(),
            camera: Camera::new(),
            config,
        }
    }
}

// ---------------------------
// EVENT HANDLER
// ---------------------------
impl event::EventHandler for GameState {
    // UPDATE
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        simulate_step(&mut self.bodies, self.config.dt, self.config.g);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        for b in &self.bodies {
            // World → camera → screen
            let cam_pos = self.camera.apply(b.pos);

            let screen_pos = [cam_pos.x + 400.0, cam_pos.y + 300.0];

            // Radius NOT affected by zoom
            let radius = b.radius * self.camera.zoom.recip();

            let circle =
                Mesh::new_circle(ctx, DrawMode::fill(), screen_pos, radius, 0.1, Color::RED)?;

            canvas.draw(&circle, Vec2::ZERO);
        }

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

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        let pan_speed = 20.0 / self.camera.zoom;

        match input.keycode {
            Some(KeyCode::W) => {
                self.camera.offset.y += pan_speed;
            }
            Some(KeyCode::S) => {
                self.camera.offset.y -= pan_speed;
            }
            Some(KeyCode::A) => {
                self.camera.offset.x += pan_speed;
            }
            Some(KeyCode::D) => {
                self.camera.offset.x -= pan_speed;
            }
            _ => {}
        }

        Ok(())
    }
}

// ---------------------------
// MAIN
// ---------------------------
fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("nbody_ggez", "you")
        .window_mode(WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    let state = GameState::new();
    event::run(ctx, event_loop, state)
}
