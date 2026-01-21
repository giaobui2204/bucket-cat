use raylib::prelude::*;
use crate::config;
use crate::input::Input;

use crate::game::bucket::Bucket;
use crate::game::collision;
use crate::game::effects::{DevilEffect, EffectsState, MusicMode};
use crate::game::objects::FallingObject;
use crate::game::objects::CatKind;
use crate::game::scoring::Scoring;
use crate::game::spawn::Spawner;

pub struct World {
    pub bucket: Bucket,
    pub objects: Vec<FallingObject>,
    spawner: Spawner,
    scoring: Scoring,
    effects: EffectsState,
    elapsed_time: f32,
    base_bucket_size: Vector2,
    bucket_scale: f32,
}

impl World {
    pub fn new(screen_w: f32, screen_h: f32) -> Self {
        let bucket = Bucket::new(screen_w, screen_h);
        let base_bucket_size = bucket.size;
        Self {
            bucket,
            objects: Vec::new(),
            spawner: Spawner::new(),
            scoring: Scoring::new(),
            effects: EffectsState::new(),
            elapsed_time: 0.0,
            base_bucket_size,
            bucket_scale: 1.0,
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
        self.elapsed_time += dt;
        self.effects.update(dt);
        let move_x = self.effects.apply_input(input.move_x);
        let difficulty = 1.0 + self.elapsed_time * config::OBJ_SPEED_SCALE;
        self.bucket.update(move_x, dt, screen_w, difficulty);

        let desired_scale = self.effects.bucket_scale();
        if (desired_scale - self.bucket_scale).abs() > f32::EPSILON {
            self.bucket_scale = desired_scale;
            let size = Vector2::new(
                self.base_bucket_size.x * desired_scale,
                self.base_bucket_size.y * desired_scale,
            );
            self.bucket.apply_size(size, screen_w, screen_h);
        }

        if let Some(obj) = self.spawner.update(rl, dt, screen_w, self.elapsed_time) {
            self.objects.push(obj);
        }

        for obj in &mut self.objects {
            obj.update(dt);
        }

        let mut remaining = Vec::with_capacity(self.objects.len());
        let mut caught = Vec::new();
        let bucket_rect = self.bucket.rect();
        for obj in self.objects.drain(..) {
            if collision::check_collision(obj.pos, obj.radius, bucket_rect) {
                caught.push(obj.kind());
                continue;
            }
            if !obj.offscreen(screen_h) {
                remaining.push(obj);
            }
        }
        self.objects = remaining;
        for kind in caught {
            self.handle_catch(kind, rl);
        }
    }

    pub fn score(&self) -> i32 {
        self.scoring.score()
    }

    pub fn music_mode(&self) -> MusicMode {
        self.effects.music_mode()
    }

    pub fn effect_message(&self) -> Option<&str> {
        self.effects.message()
    }

    pub fn effect_message_alpha(&self) -> f32 {
        self.effects.message_alpha()
    }

    pub fn explosion_effect(&self) -> Option<(Vector2, f32)> {
        self.effects.explosion()
    }

    pub fn set_base_bucket_size(&mut self, size: Vector2) {
        self.base_bucket_size = size;
        self.bucket_scale = 1.0;
    }

    fn handle_catch(&mut self, kind: CatKind, rl: &RaylibHandle) {
        let multiplier = self.effects.score_multiplier();
        self.scoring.register_catch(kind, rl, multiplier);

        match kind {
            CatKind::Devil => {
                let effect = self.random_devil_effect(rl);
                self.apply_devil_effect(effect);
            }
            CatKind::Explode => {
                self.apply_devil_effect(DevilEffect::BucketExplode);
            }
            _ => {}
        }
    }

    fn random_devil_effect(&self, rl: &RaylibHandle) -> DevilEffect {
        let effects = [
            DevilEffect::InvertControls,
            DevilEffect::BucketSmall,
            DevilEffect::BucketLarge,
            DevilEffect::MusicSwap,
            DevilEffect::BucketExplode,
            DevilEffect::ScoreDouble,
            DevilEffect::ScoreTriple,
        ];
        let max_index = (effects.len() as i32 - 1).max(0);
        let index = rl.get_random_value::<i32>(0..max_index) as usize;
        effects[index]
    }

    fn apply_devil_effect(&mut self, effect: DevilEffect) {
        match effect {
            DevilEffect::InvertControls => {
                self.effects.apply_invert();
                self.effects.set_message("Controls inverted!");
            }
            DevilEffect::BucketSmall => {
                self.effects.apply_size_scale(config::DEVIL_SIZE_SMALL_SCALE);
                self.effects.set_message("Bucket shrunk!");
            }
            DevilEffect::BucketLarge => {
                self.effects.apply_size_scale(config::DEVIL_SIZE_LARGE_SCALE);
                self.effects.set_message("Bucket enlarged!");
            }
            DevilEffect::MusicSwap => {
                self.effects.apply_music_swap();
                self.effects.set_message("Music changed!");
            }
            DevilEffect::BucketExplode => {
                let rect = self.bucket.rect();
                let center = Vector2::new(rect.x + rect.width / 2.0, rect.y + rect.height / 2.0);
                self.effects.trigger_explosion(center);
                self.scoring.reset();
                self.effects.set_message("Bucket exploded! Score reset!");
            }
            DevilEffect::ScoreDouble => {
                self.effects.apply_score_multiplier(2);
                self.effects.set_message("Score x2!");
            }
            DevilEffect::ScoreTriple => {
                self.effects.apply_score_multiplier(3);
                self.effects.set_message("Score x3!");
            }
        }
    }
}
