use raylib::prelude::*;

pub fn run() {
    let (mut rl, thread) = raylib::init()
        .size(420, 320)         
        .title("Bucket Catch")
        .resizable()
        .build();

    rl.set_target_fps(120);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_text("Mini Window 🌱", 120, 140, 20, Color::RAYWHITE);
    }
}
