use raylib::prelude::*;
use crate::input::Input;

use crate::game::bucket::Bucket;
use crate::game::collision;
use crate::game::objects::FallingObject;
use crate::game::scoring::Scoring;
use crate::game::spawn::Spawner;

pub struct World {
    pub bucket: Bucket,
    pub objects: Vec<FallingObject>,
    spawner: Spawner,
    scoring: Scoring,
}

impl World {
    pub fn new(screen_w: f32, screen_h: f32) -> Self {
        Self {
            bucket: Bucket::new(screen_w, screen_h),
            objects: Vec::new(),
            spawner: Spawner::new(),
            scoring: Scoring::new(),
        }
    }

    pub fn update(
        &mut self,
        rl: &RaylibHandle,
        input: Input,
        dt: f32,
        screen_w: f32,
        screen_h: f32,
    ) {
        self.bucket.update(input.move_x, dt, screen_w);

        if let Some(obj) = self.spawner.update(rl, dt, screen_w) {
            self.objects.push(obj);
        }

        for obj in &mut self.objects {
            obj.update(dt);
        }

        self.objects.retain(|o| {
            if collision::check_collision(o.pos, o.radius, self.bucket.rect()) {
                self.scoring.register_catch(o.kind());
                return false;
            }
            !o.offscreen(screen_h)
        });
    }

    pub fn score(&self) -> i32 {
        self.scoring.score()
    }
}
