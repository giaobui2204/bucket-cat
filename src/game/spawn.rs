use raylib::prelude::*;
use crate::config;
use crate::game::objects::{CatKind, FallingObject};

pub struct Spawner {
    timer: f32,
    elapsed_time: f32,
    devil_cooldown: f32,
}

impl Spawner {
    pub fn new() -> Self {
        Self {
            timer: 0.0,
            elapsed_time: 0.0,
            devil_cooldown: 0.0,
        }
    }

    pub fn update(
        &mut self,
        rl: &RaylibHandle,
        dt: f32,
        screen_w: f32,
        elapsed_time: f32,
    ) -> Option<FallingObject> {
        self.timer += dt;
        self.elapsed_time += dt;
        if self.devil_cooldown > 0.0 {
            self.devil_cooldown = (self.devil_cooldown - dt).max(0.0);
        }
        if self.timer < config::SPAWN_INTERVAL {
            return None;
        }

        self.timer = 0.0;
        let x = rl.get_random_value::<i32>(20..(screen_w as i32 - 20)) as f32;
        let devil_allowed = self.devil_cooldown <= 0.0;
        let devil_rate = if devil_allowed {
            devil_rate(self.elapsed_time)
        } else {
            0.0
        };
        let angel_rate = config::ANGEL_RATE;
        let normal_rate = (1.0 - angel_rate - devil_rate).max(0.0);
        let roll = rl.get_random_value::<i32>(0..1000) as f32 / 1000.0;
        let kind = if roll < devil_rate {
            self.devil_cooldown = config::DEVIL_COOLDOWN;
            CatKind::Devil
        } else if roll < devil_rate + angel_rate {
            CatKind::Angel
        } else if roll < devil_rate + angel_rate + normal_rate {
            CatKind::Normal
        } else {
            CatKind::Normal
        };
        let difficulty = 1.0 + elapsed_time * config::OBJ_SPEED_SCALE;
        let initial_speed = config::OBJ_BASE_SPEED * difficulty;
        Some(FallingObject::new(x, kind, initial_speed))
    }
}

fn devil_rate(elapsed_time: f32) -> f32 {
    if elapsed_time <= config::DEVIL_RATE_EARLY_END {
        config::DEVIL_RATE_EARLY
    } else if elapsed_time <= config::DEVIL_RATE_MID_END {
        config::DEVIL_RATE_MID
    } else {
        config::DEVIL_RATE_LATE
    }
}
