use raylib::prelude::*;

use crate::game::bucket::Bucket;
use crate::game::objects::FallingObject;
use crate::game::spawn::Spawner;

pub struct World {
    pub bucket: Bucket,
    pub objects: Vec<FallingObject>,
    spawner: Spawner,
}

impl World {
    pub fn new(screen_w: f32, screen_h: f32) -> Self {
        Self {
            bucket: Bucket::new(screen_w, screen_h),
            objects: Vec::new(),
            spawner: Spawner::new(),
        }
    }

    pub fn update(
        &mut self,
        rl: &RaylibHandle,
        axis: f32,
        dt: f32,
        screen_w: f32,
        screen_h: f32,
    ) {
        self.bucket.update(axis, dt, screen_w);

        if let Some(obj) = self.spawner.update(rl, dt, screen_w) {
            self.objects.push(obj);
        }

        for obj in &mut self.objects {
            obj.update(dt);
        }

        self.objects.retain(|o| !o.offscreen(screen_h));
    }
}
