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
    
    // Angry Cat Mechanism
    pub angry_points: i32,
    pub max_angry_points: i32,
    pub giant_cat_y: Option<f32>,
    pub giant_cat_frame: usize,
    giant_cat_anim_timer: f32,
    giant_cat_landed_timer: f32,
    pub game_over_trigger: bool,
}

impl World {
    pub fn new(screen_w: f32, screen_h: f32, initial_max_angry: i32) -> Self {
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

            angry_points: 0,
            max_angry_points: initial_max_angry,
            giant_cat_y: None,
            giant_cat_frame: 0,
            giant_cat_landed_timer: 0.0,
            giant_cat_anim_timer: 0.0,
            game_over_trigger: false,
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
        if let Some(y) = self.giant_cat_y {
            let speed = config::GIANT_CAT_SPEED;
            let target_y = screen_h - (config::CRYING_CAT_FRAME_H * config::CRYING_CAT_SCALE);
            
            // Move if we haven't reached the target
            if y < target_y {
                let new_y = y + speed * dt;
                // clamp
                let final_y = if new_y > target_y { target_y } else { new_y };
                self.giant_cat_y = Some(final_y);
                
                // Animate Giant Cat while moving
                self.giant_cat_anim_timer += dt;
                if self.giant_cat_anim_timer >= 1.0 / config::CRYING_CAT_FPS {
                    self.giant_cat_anim_timer = 0.0;
                    self.giant_cat_frame = (self.giant_cat_frame + 1) % config::CRYING_CAT_FRAMES;
                }
                
                if final_y >= target_y {
                    // self.game_over_trigger = true;
                    // Start timer
                    self.giant_cat_landed_timer += dt;
                    if self.giant_cat_landed_timer >= config::GIANT_CAT_LANDED_DELAY {
                        self.game_over_trigger = true;
                    }
                }
            } else {
                // Ensure we stay at target
                self.giant_cat_y = Some(target_y);
                self.giant_cat_landed_timer += dt;
                if self.giant_cat_landed_timer >= config::GIANT_CAT_LANDED_DELAY {
                    self.game_over_trigger = true;
                }
            }
            return;
        }

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
            } else {
                 match obj.kind() {
                     CatKind::Normal => self.angry_points += config::ANGRY_PENALTY_NORMAL,
                     CatKind::Angel => self.angry_points += config::ANGRY_PENALTY_ANGEL,
                     _ => {}
                 }
            }
        }
        self.objects = remaining;
        for kind in caught {
            self.handle_catch(kind, rl);
        }

        if self.angry_points >= self.max_angry_points && self.giant_cat_y.is_none() {
             self.giant_cat_y = Some(-800.0); // Start way above
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
                self.apply_devil_effect(effect, rl);
            }
            CatKind::Explode => {
                self.apply_devil_effect(DevilEffect::BucketExplode, rl);
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
        // Fix: get_random_value uses GetRandomValue(min, max) which is INCLUSIVE.
        // So for an array of size 7 (indices 0..6), we must pass max=6.
        let max_index = effects.len() as i32 - 1;
        let index = rl.get_random_value::<i32>(0..max_index) as usize;
        effects[index]
    }

    fn apply_devil_effect(&mut self, effect: DevilEffect, rl: &RaylibHandle) {
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
                // 0..5: Alternate tracks (6 tracks total index 0-5)
                // 6: Default track
                // GetRandomValue(0, 6) returns 0..6 inclusive
                let mut pick = rl.get_random_value::<i32>(0..6);
                let current_mode = self.effects.music_mode();

                loop {
                    let is_same = match current_mode {
                        MusicMode::Default => pick == 6,
                        MusicMode::Alternate(idx) => pick == idx as i32,
                    };

                    if !is_same { break; }
                    pick = rl.get_random_value::<i32>(0..6);
                }

                if pick == 6 {
                    self.effects.set_music_default();
                    self.effects.set_message("Music Reset!");
                } else {
                    self.effects.apply_music_swap(pick as usize);
                    self.effects.set_message("Music changed!");
                }
            }
            DevilEffect::BucketExplode => {
                let rect = self.bucket.rect();
                let center = Vector2::new(rect.x + rect.width / 2.0, rect.y + rect.height / 2.0);
                self.effects.trigger_explosion(center);
                self.scoring.reset();
                self.effects.set_message("Bucket exploded! Score reset!");
            }
            DevilEffect::ScoreDouble => {
                self.scoring.apply_multiplier(2);
                self.effects.set_message("Score x2!");
            }
            DevilEffect::ScoreTriple => {
                self.scoring.apply_multiplier(3);
                self.effects.set_message("Score x3!");
            }
        }
    }
}
