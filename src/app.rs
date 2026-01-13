use raylib::prelude::*;

use crate::config;
use crate::game::world::World;
use crate::render;

pub fn run() {
    let (mut rl, thread) = raylib::init()
        .size(config::SCREEN_W, config::SCREEN_H)
        .title("Bucket Catch")
        .resizable()
        .build();

    rl.set_target_fps(config::TARGET_FPS);

    let mut world = World::new(
        config::SCREEN_W as f32,
        config::SCREEN_H as f32,
    );

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let screen_w = rl.get_screen_width() as f32;
        let screen_h = rl.get_screen_height() as f32;

        let mut axis = 0.0;
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
            axis -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
            axis += 1.0;
        }

        world.update(&rl, axis, dt, screen_w, screen_h);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        render::draw_world(&mut d, &world);
    }
}
