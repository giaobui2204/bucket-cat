use raylib::prelude::*;
use crate::config;
use crate::game::objects::{CatKind, FallingObject};

pub struct Spawner {
    timer: f32,
}

impl Spawner {
    pub fn new() -> Self {
        Self { timer: 0.0 }
    }

    pub fn update(
        &mut self,
        rl: &RaylibHandle,
        dt: f32,
        screen_w: f32,
        elapsed_time: f32,
    ) -> Option<FallingObject> {
        self.timer += dt;
        if self.timer < config::SPAWN_INTERVAL {
            return None;
        }

        self.timer = 0.0;
        let x = rl.get_random_value::<i32>(20..(screen_w as i32 - 20)) as f32;
        let kind = match rl.get_random_value::<i32>(0..3) {
            0 => CatKind::Normal,
            1 => CatKind::Angel,
            _ => CatKind::Devil,
        };
        let difficulty = 1.0 + elapsed_time * config::OBJ_SPEED_SCALE;
        let initial_speed = config::OBJ_BASE_SPEED * difficulty;
        Some(FallingObject::new(x, kind, initial_speed))
    }
}
