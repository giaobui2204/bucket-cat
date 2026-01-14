use crate::config::*;
use crate::game::objects::CatKind;

pub struct Scoring {
    score: i32,
}

impl Scoring {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    pub fn register_catch(&mut self, kind: CatKind) {
        let delta = match kind {
            CatKind::Normal => NORMAL_CAT_SCORE,
            CatKind::Angel => ANGEL_CAT_SCORE,
            CatKind::Devil => DEVIL_CAT_SCORE,
        };
        self.score += delta;
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}

