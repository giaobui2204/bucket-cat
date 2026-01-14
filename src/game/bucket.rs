use raylib::prelude::*;
use crate::config;

#[derive(Clone)]
pub struct Bucket {
    pub pos: Vector2,
    vel: Vector2,
    pub size: Vector2,
    anim_time: f32,
    anim_frame: usize,
}

impl Bucket {
    pub fn new(screen_w: f32, screen_h: f32) -> Self {
        Self {
            pos: Vector2::new(
                screen_w / 2.0 - config::BUCKET_W / 2.0,
                screen_h - config::BUCKET_Y_OFFSET,
            ),
            vel: Vector2::new(0.0, 0.0),
            size: Vector2::new(config::BUCKET_W, config::BUCKET_H),
            anim_time: 0.0,
            anim_frame: 0,
        }
    }

    pub fn rect(&self) -> Rectangle {
        Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            width: self.size.x,
            height: self.size.y,
        }
    }

    pub fn update(&mut self, axis: f32, dt: f32, screen_w: f32) {
        self.vel.x += axis * config::BUCKET_ACCEL * dt;

        if axis == 0.0 {
            self.vel.x *= 1.0 / (1.0 + config::BUCKET_FRICTION * dt);
        }

        self.vel.x = self
            .vel
            .x
            .clamp(-config::BUCKET_MAX_SPEED, config::BUCKET_MAX_SPEED);

        self.pos.x += self.vel.x * dt;

        let max_x = (screen_w - self.size.x).max(0.0);
        if self.pos.x < 0.0 {
            self.pos.x = 0.0;
            self.vel.x = 0.0;
        }
        if self.pos.x > max_x {
            self.pos.x = max_x;
            self.vel.x = 0.0;
        }

        if config::BUCKET_FRAME_COUNT > 1 {
            let frame_dt = 1.0 / config::BUCKET_ANIM_FPS.max(1.0);
            self.anim_time += dt;
            while self.anim_time >= frame_dt {
                self.anim_time -= frame_dt;
                self.anim_frame = (self.anim_frame + 1) % config::BUCKET_FRAME_COUNT;
            }
        }
    }

    pub fn frame(&self) -> usize {
        self.anim_frame
    }

    pub fn set_size(&mut self, size: Vector2, screen_w: f32, screen_h: f32) {
        self.size = size;
        self.pos.x = (screen_w - self.size.x) / 2.0;
        self.pos.y = screen_h - config::BUCKET_Y_OFFSET;
    }
}
