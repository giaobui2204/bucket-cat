use raylib::prelude::*;

use crate::config;
use crate::input::Input;
use crate::game::world::World;
use crate::render;

pub fn run() {
    let (mut rl, thread) = raylib::init()
        .size(config::SCREEN_W, config::SCREEN_H)
        .title("Bucket Catch")
        .resizable()
        .build();

    rl.set_target_fps(config::TARGET_FPS);

    //access to the assets texture
    let bucket_texture = rl
        .load_texture(&thread, "src/assets/cat/bucket.png")
        .expect("load bucket texture");
    let normal_texture = rl
        .load_texture(&thread, "src/assets/cat/normalneko.png")
        .expect("load normal neko texture");
    let angel_texture = rl
        .load_texture(&thread, "src/assets/cat/angelneko.png")
        .expect("load angel neko texture");
    let devil_texture = rl
        .load_texture(&thread, "src/assets/cat/devilneko.png")
        .expect("load devil neko texture");

    let mut world = World::new(config::SCREEN_W as f32, config::SCREEN_H as f32);
    let bucket_frame_w =
        bucket_texture.width as f32 / config::BUCKET_FRAME_COLS as f32 * config::BUCKET_DRAW_SCALE;
    let bucket_frame_h = bucket_texture.height as f32 / config::BUCKET_FRAME_ROWS as f32
        * config::BUCKET_DRAW_SCALE;
    world.bucket.set_size(
        Vector2::new(bucket_frame_w, bucket_frame_h),
        config::SCREEN_W as f32,
        config::SCREEN_H as f32,
    );

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let screen_w = rl.get_screen_width() as f32;
        let screen_h = rl.get_screen_height() as f32;

        let input = Input::gather(&rl);
        world.update(&rl, input, dt, screen_w, screen_h);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        render::draw_world(
            &mut d,
            &world,
            &bucket_texture,
            &normal_texture,
            &angel_texture,
            &devil_texture,
        );
    }
}
