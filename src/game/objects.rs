use crate::config;
use raylib::prelude::*;

#[derive(Clone, Copy)]
pub enum CatKind {
    Normal,
    Angel,
    Devil,
    Explode,
}

#[derive(Clone)]
pub struct FallingObject {
    pub pos: Vector2,
    pub radius: f32,
    kind: CatKind,
    anim_time: f32,
    anim_frame: usize,
}

impl FallingObject {
    pub fn new(x: f32, kind: CatKind) -> Self {
        Self {
            pos: Vector2::new(x, -10.0),
            radius: config::OBJ_RADIUS,
            kind,
            anim_time: 0.0,
            anim_frame: 0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos.y += config::OBJ_FALL_SPEED * dt;
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
