use raylib::prelude::*;

use crate::config;
use crate::input::Input;
use crate::game::world::World;
use crate::render;
use crate::ui;
use crate::state::menu::{MenuAction, MenuState};
use crate::state::pause::{PauseState, PauseAction};

enum Screen {
    Menu,
    Playing,
    Paused,
}

pub fn run() {
    let (mut rl, thread) = raylib::init()
        .size(config::SCREEN_W, config::SCREEN_H)
        .title("Bucket Catch")
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
    let logo_texture = rl
        .load_texture(&thread, "src/assets/UI/bucket-logo.png")
        .expect("load logo texture");

    let mut screen = Screen::Menu;
    let mut menu = MenuState::new();
    let pause_menu = PauseState::new();
    let mut world = create_world(
        config::SCREEN_W as f32,
        config::SCREEN_H as f32,
        &bucket_texture,
    );

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let screen_w = rl.get_screen_width() as f32;
        let screen_h = rl.get_screen_height() as f32;
        let mouse = rl.get_mouse_position();
        let clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        let font = rl.get_font_default();

        if let Screen::Playing = screen {
            // Check for pause button click BEFORE update
            let pause_btn_rect = Rectangle::new(screen_w - 50.0, 10.0, 40.0, 40.0);
            if clicked && pause_btn_rect.check_collision_point_rec(mouse) {
                screen = Screen::Paused;
            } else {
                let input = Input::gather(&rl);
                world.update(&rl, input, dt, screen_w, screen_h);
            }
        }

        let mut d = rl.begin_drawing(&thread);
        match screen {
            Screen::Menu => {
                if let MenuAction::Start =
                    menu.update_and_draw(
                        &mut d,
                        screen_w,
                        screen_h,
                        mouse,
                        clicked,
                        &font,
                        &logo_texture,
                    )
                {
                    world = create_world(screen_w, screen_h, &bucket_texture);
                    screen = Screen::Playing;
                }
            }
            Screen::Playing => {
                d.clear_background(Color::BLACK);
                render::draw_world(
                    &mut d,
                    &world,
                    &bucket_texture,
                    &normal_texture,
                    &angel_texture,
                    &devil_texture,
                );
                
                // Draw Pause Button
                let pause_btn_rect = Rectangle::new(screen_w - 50.0, 10.0, 40.0, 40.0);
                ui::draw_button(
                    &mut d,
                    pause_btn_rect,
                    "||",
                    mouse,
                    false,
                    &font,
                    config::COLOR_LIGHT_BG,
                    config::COLOR_LIGHT_HOVER,
                    config::COLOR_ACCENT_TEXT,
                    config::COLOR_ACCENT_BORDER,
                );
            }
            Screen::Paused => {
                // Draw world as background
                d.clear_background(Color::BLACK);
                render::draw_world(
                    &mut d,
                    &world,
                    &bucket_texture,
                    &normal_texture,
                    &angel_texture,
                    &devil_texture,
                );

                match pause_menu.update_and_draw(&mut d, world.score(), screen_w, screen_h, mouse, clicked) {
                    PauseAction::Resume => screen = Screen::Playing,
                    PauseAction::Exit => screen = Screen::Menu,
                    PauseAction::None => {},
                }
            }
        }
    }
}

fn create_world(screen_w: f32, screen_h: f32, bucket_texture: &Texture2D) -> World {
    let mut world = World::new(screen_w, screen_h);
    let bucket_frame_w =
        bucket_texture.width as f32 / config::BUCKET_FRAME_COLS as f32 * config::BUCKET_DRAW_SCALE;
    let bucket_frame_h = bucket_texture.height as f32 / config::BUCKET_FRAME_ROWS as f32
        * config::BUCKET_DRAW_SCALE;
    world.bucket.set_size(Vector2::new(bucket_frame_w, bucket_frame_h), screen_w, screen_h);
    world
}
