use crate::config::*;
use crate::game::objects::CatKind;
use raylib::prelude::*;

pub struct Scoring {
    score: i32,
}

impl Scoring {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    pub fn register_catch(&mut self, kind: CatKind, rl: &RaylibHandle, multiplier: i32) -> i32 {
        let delta = match kind {
            CatKind::Normal => NORMAL_CAT_SCORE,
            CatKind::Angel => rl.get_random_value::<i32>(2..6),
            CatKind::Devil => DEVIL_CAT_SCORE,
            CatKind::Explode => DEVIL_CAT_SCORE,
        };
        let total = delta * multiplier.max(1);
        self.score += total;
        total
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn reset(&mut self) {
        self.score = 0;
    }
}
