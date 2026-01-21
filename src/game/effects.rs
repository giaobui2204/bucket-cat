use raylib::prelude::*;
use crate::config;

#[derive(Clone, Copy)]
pub enum DevilEffect {
    InvertControls,
    BucketSmall,
    BucketLarge,
    MusicSwap,
    BucketExplode,
    ScoreDouble,
    ScoreTriple,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MusicMode {
    Default,
    Alternate(usize),
}

pub struct EffectsState {
    invert_timer: f32,
    size_timer: f32,
    size_scale: f32,
    multiplier_timer: f32,
    score_multiplier: i32,
    music_timer: f32, // Kept for struct layout, but logic will be removed
    music_mode: MusicMode,
    message: String,
    message_timer: f32,
    explode_timer: f32,
    explode_pos: Vector2,
}

impl EffectsState {
    pub fn new() -> Self {
        Self {
            invert_timer: 0.0,
            size_timer: 0.0,
            size_scale: 1.0,
            multiplier_timer: 0.0,
            score_multiplier: 1,
            music_timer: 0.0,
            music_mode: MusicMode::Default,
            message: String::new(),
            message_timer: 0.0,
            explode_timer: 0.0,
            explode_pos: Vector2::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.invert_timer = (self.invert_timer - dt).max(0.0);
        self.size_timer = (self.size_timer - dt).max(0.0);
        if self.size_timer <= 0.0 {
            self.size_scale = 1.0;
        }
        self.multiplier_timer = (self.multiplier_timer - dt).max(0.0);
        if self.multiplier_timer <= 0.0 {
            self.score_multiplier = 1;
        }
        // Music timer logic removed to prevent auto-reset
        // self.music_timer = (self.music_timer - dt).max(0.0);
        // if self.music_timer <= 0.0 {
        //    self.music_mode = MusicMode::Default;
        // }
        self.message_timer = (self.message_timer - dt).max(0.0);
        if self.message_timer <= 0.0 {
            self.message.clear();
        }
        self.explode_timer = (self.explode_timer - dt).max(0.0);
    }

    pub fn apply_input(&self, move_x: f32) -> f32 {
        if self.invert_timer > 0.0 {
            -move_x
        } else {
            move_x
        }
    }

    pub fn bucket_scale(&self) -> f32 {
        self.size_scale
    }

    pub fn score_multiplier(&self) -> i32 {
        self.score_multiplier
    }

    pub fn music_mode(&self) -> MusicMode {
        self.music_mode
    }

    pub fn message(&self) -> Option<&str> {
        if self.message_timer > 0.0 && !self.message.is_empty() {
            Some(self.message.as_str())
        } else {
            None
        }
    }

    pub fn message_alpha(&self) -> f32 {
        if config::DEVIL_EFFECT_MESSAGE_DURATION <= 0.0 {
            1.0
        } else {
            (self.message_timer / config::DEVIL_EFFECT_MESSAGE_DURATION).clamp(0.0, 1.0)
        }
    }

    pub fn explosion(&self) -> Option<(Vector2, f32)> {
        if self.explode_timer > 0.0 {
            let t = 1.0 - (self.explode_timer / config::DEVIL_EXPLODE_ANIM_DURATION);
            Some((self.explode_pos, t.clamp(0.0, 1.0)))
        } else {
            None
        }
    }

    pub fn set_message(&mut self, text: &str) {
        self.message = text.to_string();
        self.message_timer = config::DEVIL_EFFECT_MESSAGE_DURATION;
    }

    pub fn apply_invert(&mut self) {
        self.invert_timer = config::DEVIL_INVERT_DURATION;
    }

    pub fn apply_size_scale(&mut self, scale: f32) {
        self.size_scale = scale;
        self.size_timer = config::DEVIL_SIZE_EFFECT_DURATION;
    }

    pub fn apply_score_multiplier(&mut self, multiplier: i32) {
        self.score_multiplier = multiplier.max(1);
        self.multiplier_timer = config::DEVIL_SCORE_MULTIPLIER_DURATION;
    }
    pub fn apply_music_swap(&mut self, track_idx: usize) {
        self.music_mode = MusicMode::Alternate(track_idx);
        // self.music_timer = config::DEVIL_MUSIC_SWAP_DURATION; // Timer no longer used
    }

    pub fn set_music_default(&mut self) {
        self.music_mode = MusicMode::Default;
    }

    pub fn trigger_explosion(&mut self, pos: Vector2) {
        self.explode_pos = pos;
        self.explode_timer = config::DEVIL_EXPLODE_ANIM_DURATION;
    }
}
