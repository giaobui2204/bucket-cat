use raylib::prelude::*;
use crate::config;

#[derive(Clone)]
pub struct FallingObject {
    pub pos: Vector2,
    pub radius: f32,
}

impl FallingObject {
    pub fn new(x: f32) -> Self {
        Self {
            pos: Vector2::new(x, -10.0),
            radius: config::OBJ_RADIUS,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos.y += config::OBJ_FALL_SPEED * dt;
    }

    pub fn offscreen(&self, screen_h: f32) -> bool {
        self.pos.y - self.radius > screen_h
    }
}
