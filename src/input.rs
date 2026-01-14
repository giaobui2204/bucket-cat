use raylib::prelude::*;
use crate::config;

#[derive(Default, Clone, Copy)]
pub struct Input {
    pub move_x: f32,          // final axis [-1, 1]
    pub using_gamepad: bool,  // optional, for UI later
}

fn first_gamepad_id(rl: &RaylibHandle) -> Option<i32> {
    for id in 0..=3 {
        if rl.is_gamepad_available(id) {
            return Some(id);
        }
    }
    None
}

impl Input {
    pub fn gather(rl: &RaylibHandle) -> Self {
        // Keyboard axis
        let left = rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A);
        let right = rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D);

        let axis_kb = match (left, right) {
            (true, false) => -config::KEYBOARD_AXIS,
            (false, true) => config::KEYBOARD_AXIS,
            _ => 0.0,
        };

        // Gamepad axis (left stick X)
        let mut axis_gp = 0.0;
        if let Some(id) = first_gamepad_id(rl) {
            axis_gp = rl.get_gamepad_axis_movement(id, GamepadAxis::GAMEPAD_AXIS_LEFT_X);

            // Deadzone
            if axis_gp.abs() < config::GAMEPAD_DEADZONE {
                axis_gp = 0.0;
            }
        }

        // Prefer gamepad if active, otherwise keyboard
        let (move_x, using_gamepad) = if axis_gp.abs() > axis_kb.abs() {
            (axis_gp, axis_gp.abs() > 0.0)
        } else {
            (axis_kb, false)
        };

        Self {
            move_x: move_x.clamp(config::MOVE_AXIS_MIN, config::MOVE_AXIS_MAX),
            using_gamepad,
        }
    }
}
