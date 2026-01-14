use raylib::prelude::*;

pub struct Rng {
    // placeholder for seeded RNG later
}

impl Rng {
    pub fn new() -> Self {
        Self {}
    }

    pub fn range_i32(&mut self, rl: &RaylibHandle, range: std::ops::Range<i32>) -> i32 {
        rl.get_random_value(range)
    }
}
