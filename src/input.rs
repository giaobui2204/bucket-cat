use raylib::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub pause_pressed: bool,
    pub start_pressed: bool,
    pub restart_pressed: bool,
}

impl Input {
    pub fn gather(rl: &RaylibHandle) -> Self {
        let left = rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A);
        let right = rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D);

        let pause_pressed = rl.is_key_pressed(KeyboardKey::KEY_P) || rl.is_key_pressed(KeyboardKey::KEY_ESCAPE);
        let start_pressed = rl.is_key_pressed(KeyboardKey::KEY_ENTER) || rl.is_key_pressed(KeyboardKey::KEY_SPACE);
        let restart_pressed = rl.is_key_pressed(KeyboardKey::KEY_R);

        Self {
            left,
            right,
            pause_pressed,
            start_pressed,
            restart_pressed,
        }
    }

    pub fn move_axis(&self) -> f32 {
        let mut axis = 0.0;
        if self.left { axis -= 1.0; }
        if self.right { axis += 1.0; }
        axis
    }
}
