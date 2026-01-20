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

    pub fn register_catch(&mut self, kind: CatKind, rl: &RaylibHandle) {
        let delta = match kind {
            CatKind::Normal => NORMAL_CAT_SCORE,
            CatKind::Angel => rl.get_random_value::<i32>(2..6),
            CatKind::Devil => DEVIL_CAT_SCORE,
            CatKind::Explode => DEVIL_CAT_SCORE,
        };
        self.score += delta;
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}
