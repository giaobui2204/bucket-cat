use crate::config;
use raylib::prelude::*;

#[derive(Clone, Copy)]
pub enum CatKind {
    Normal,
    Angel,
    Devil,
}

#[derive(Clone)]
pub struct FallingObject {
    pub pos: Vector2,
    pub radius: f32,
    velocity: Vector2,
    accel: Vector2,
    kind: CatKind,
    anim_time: f32,
    anim_frame: usize,
}

impl FallingObject {
    pub fn new(x: f32, kind: CatKind, initial_speed: f32) -> Self {
        Self {
            pos: Vector2::new(x, -10.0),
            radius: config::OBJ_RADIUS,
            velocity: Vector2::new(0.0, initial_speed),
            accel: Vector2::new(0.0, config::OBJ_GRAVITY),
            kind,
            anim_time: 0.0,
            anim_frame: 0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity.y += self.accel.y * dt;
        self.velocity.y = self.velocity.y.min(config::OBJ_MAX_SPEED);
        self.pos.y += self.velocity.y * dt;
        if config::OBJ_FRAME_COUNT > 1 {
            let frame_dt = 1.0 / config::OBJ_ANIM_FPS.max(1.0);
            self.anim_time += dt;
            while self.anim_time >= frame_dt {
                self.anim_time -= frame_dt;
                self.anim_frame = (self.anim_frame + 1) % config::OBJ_FRAME_COUNT;
            }
        }
    }

    pub fn offscreen(&self, screen_h: f32) -> bool {
        self.pos.y - self.radius > screen_h
    }

    pub fn frame(&self) -> usize {
        self.anim_frame
    }

    pub fn kind(&self) -> CatKind {
        self.kind
    }
}
