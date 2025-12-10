mod controls;
mod physics;
mod player;
mod sim_config;

use controls::Controls;
use physics::{Body, simulate_step};
use player::Player;
use sim_config::SimConfig;

use ggez::conf::WindowMode;
use ggez::{
    Context, ContextBuilder, GameResult, event,
    glam::Vec2,
    graphics::{self, Color, DrawMode, Mesh, PxScale, Text, TextFragment},
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
    player: Player,
    game_over: bool,
}

impl GameState {
    pub fn new() -> Self {
        let config = SimConfig::default();

        // -----------------------------------------
        // Spawn regular particles
        // -----------------------------------------
        let mut bodies = config.spawn_bodies();

        // -----------------------------------------
        // Add Player as a physics body (from Player module)
        // -----------------------------------------
        let (mut player, player_body) = Player::new();

        // Assign real ID
        player.id = bodies.len();
        bodies.push(player_body);

        // -----------------------------------------
        // Return GameState
        // -----------------------------------------
        Self {
            bodies,
            camera: Camera::new(),
            config,
            player,
            game_over: false,
        }
    }
}

// ---------------------------
// EVENT HANDLER
// ---------------------------
impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.bodies.len() <= 1 {
            self.game_over = true;
        }

        simulate_step(&mut self.bodies, self.config.dt);

        // ------------------------------------------------------
        // PLAYER EATS PARTICLES ON COLLISION
        // ------------------------------------------------------
        // First: detect collisions
        let player_id = self.player.id;
        let player_pos = self.bodies[player_id].pos;
        let player_radius = self.bodies[player_id].radius;

        let mut eaten_indices: Vec<usize> = Vec::new();

        for (i, b) in self.bodies.iter().enumerate() {
            if i == player_id {
                continue;
            }

            let dist = b.pos.distance(player_pos);
            if dist < (player_radius + b.radius) {
                eaten_indices.push(i);
            }
        }

        // ---------------------------
        // Second: safely remove them
        // ---------------------------
        let mut new_player_id = player_id;

        for &i in eaten_indices.iter().rev() {
            let eaten_mass = self.bodies[i].mass;

            // Increase player stats
            {
                let player_body = &mut self.bodies[new_player_id];
                self.player.absorb(player_body, eaten_mass);
            }

            let last = self.bodies.len() - 1;

            if i == last {
                self.bodies.pop();
                continue;
            }

            self.bodies.swap_remove(i);

            if new_player_id == last {
                new_player_id = i;
            }
        }

        self.player.id = new_player_id;

        let p = &mut self.bodies[self.player.id];

        let half_w = self.config.map_width * 0.5;
        let half_h = self.config.map_height * 0.5;

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
        let player = &self.bodies[self.player.id];
        self.camera.offset = -player.pos;
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        if self.game_over {
            let msg = Text::new(TextFragment {
                text: "YOU ATE EVERYTHING!".to_string(),
                scale: Some(PxScale::from(48.0)),
                color: Some(Color::WHITE),
                ..Default::default()
            });

            // Compute centered X
            let text_width = msg.measure(ctx)?.x;
            let x = (800.0 - text_width) * 0.5;

            // Fixed bottom position (padding 20 px)
            let y = 600.0 - 20.0 - msg.measure(ctx)?.y;

            canvas.draw(&msg, Vec2::new(x, y));
        }

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
        let player = &self.bodies[self.player.id];
        let p = self.camera.apply(player.pos);

        let screen_p = [p.x + 400.0, p.y + 300.0];
        let pr = player.radius * self.camera.zoom.recip();

        let player_circle =
            Mesh::new_circle(ctx, DrawMode::fill(), screen_p, pr, 0.1, Color::GREEN)?;

        canvas.draw(&player_circle, Vec2::ZERO);

        // --------------------------------
        // UI - Player Mass + Goal
        // --------------------------------

        let mass_text = Text::new(TextFragment {
            text: format!("Mass: {:.2}", player.mass),
            scale: Some(PxScale::from(24.0)), // font size
            color: Some(Color::WHITE),
            ..Default::default()
        });

        // Draw UI in top-left corner of screen
        canvas.draw(&mass_text, Vec2::new(10.0, 10.0));

        // --------------------------------
        // Finish frame
        // --------------------------------
        canvas.finish(ctx)?;
        Ok(())
    }

    // ZOOM with mouse wheel
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        Controls::handle_zoom(&mut self.camera, y);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        let player_body = &mut self.bodies[self.player.id];

        Controls::handle_player_movement(ctx, input, player_body);

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
