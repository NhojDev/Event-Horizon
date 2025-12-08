use ggez::Context;
use ggez::glam::Vec2;
use ggez::input::keyboard::{KeyCode, KeyInput};

use crate::Camera;
use crate::physics::Body;

pub struct Controls;

impl Controls {
    /// Handle zoom input
    pub fn handle_zoom(camera: &mut Camera, y: f32) {
        let speed = 0.12;

        if y > 0.0 {
            camera.zoom *= 1.0 + speed;
        } else if y < 0.0 {
            camera.zoom *= 1.0 - speed;
        }

        camera.zoom = camera.zoom.clamp(0.1, 6.0);
    }

    /// Handle WASD movement for the player
    pub fn handle_player_movement(ctx: &mut Context, key: KeyInput, player_body: &mut Body) {
        let dt = ctx.time.delta().as_secs_f32();

        let accel_strength = 50.0 * dt;
        let mut accel = Vec2::ZERO;

        match key.keycode {
            Some(KeyCode::W) => accel.y -= 0.5,
            Some(KeyCode::S) => accel.y += 0.5,
            Some(KeyCode::A) => accel.x -= 0.5,
            Some(KeyCode::D) => accel.x += 0.5,
            _ => {}
        }

        if accel.length_squared() > 0.0 {
            accel = accel.normalize() * accel_strength;
        }

        player_body.vel += accel;

        // Cap speed
        let max_speed = 10.0;
        let speed = player_body.vel.length();
        if speed > max_speed {
            player_body.vel = player_body.vel.normalize() * max_speed;
        }
    }
}
