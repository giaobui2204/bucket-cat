use raylib::prelude::*;
use raylib::audio::RaylibAudio;

use crate::config;
use crate::rng;
use crate::input::Input;
use crate::game::world::World;
use crate::game::effects::MusicMode;
use crate::render;
use crate::ui;
use crate::state::menu::{MenuAction, MenuState};
use crate::state::pause::{PauseState, PauseAction};
use crate::state::leaderboard::{LeaderboardState, LeaderboardAction};
use crate::state::game_over::{GameOverState, GameOverAction};
use std::path::{Path, PathBuf};

fn asset(rel: &str) -> String {
    // 1. Try relative to the executable (for release builds)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            let path = exe_dir.join(rel);
            if path.exists() {
                return path.to_str().unwrap().to_string();
            }
        }
    }

    // 2. Try src/{rel} (for development with cargo run, if assets are in src/assets)
    let src_path = Path::new("src").join(rel);
    if src_path.exists() {
        return src_path.to_str().unwrap().to_string();
    }
    
    // 3. Fallback to assuming it's in CWD or handle absolute path
    rel.to_string()
}

enum Screen {
    Menu,
    Playing,
    Paused,
    Leaderboard,
    GameOver,
}

pub fn run() {
    let (mut rl, thread) = raylib::init()
        .size(config::SCREEN_W, config::SCREEN_H)
        .title("Bucket Catch")
        .build();

    rl.set_target_fps(config::TARGET_FPS);
    let audio = RaylibAudio::init_audio_device().expect("init audio device");

    //access to the assets texture
    let bucket_texture = rl
        .load_texture(&thread, &asset("assets/cat/bucket.png"))
        .expect("load bucket texture");
    let normal_texture = rl
        .load_texture(&thread, &asset("assets/cat/normalneko.png"))
        .expect("load normal neko texture");
    let angel_texture = rl
        .load_texture(&thread, &asset("assets/cat/angelneko.png"))
        .expect("load angel neko texture");
    let devil_texture = rl
        .load_texture(&thread, &asset("assets/cat/devilneko.png"))
        .expect("load devil neko texture");
    let logo_texture = rl
        .load_texture(&thread, &asset("assets/UI/bucket-logo.png"))
        .expect("load logo texture");
    let bg_texture = rl
        .load_texture(&thread, &asset("assets/UI/play_background.png"))
        .expect("load background texture");
    let crying_cat_texture = rl
        .load_texture(&thread, &asset("assets/cat/crying_cat.png"))
        .expect("load crying cat texture");
    let music_default = audio
        .new_music(&asset("assets/sound_effects/sakura-default-music.mp3"))
        .expect("load default music");
    
    let alt_files = [
        "coffee-time-bgm.mp3",
        "chipi_chapa.mp3",
        "happy_cat.mp3",
        "maxwell_cat.mp3",
        "rickroll.mp3",
        "uiia_cat.mp3",
    ];
    let mut music_alt_list = Vec::new();
    for file in alt_files.iter() {
        music_alt_list.push(
            audio
                .new_music(&asset(&format!("assets/sound_effects/{}", file)))
                .expect("load alternate music"),
        );
    }

    let mut screen = Screen::Menu;
    let mut menu = MenuState::new();
    let pause_menu = PauseState::new();
    let mut leaderboard = LeaderboardState::new();
    let mut game_over = GameOverState::new();
    let mut world = create_world(
        config::SCREEN_W as f32,
        config::SCREEN_H as f32,
        &bucket_texture,
    );
    let mut current_music = MusicMode::Default;
    let mut music_playing = false;

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
                if world.game_over_trigger {
                    game_over.set_score(world.score());
                    screen = Screen::GameOver;
                }
            }
        }
        
        if let Screen::GameOver = screen {
             game_over.update_input(&mut rl);
        }

        let should_play_music = matches!(screen, Screen::Playing | Screen::Paused);
        if should_play_music {
            let desired = world.music_mode();
            if !music_playing {
                match desired {
                    MusicMode::Default => music_default.play_stream(),
                    MusicMode::Alternate(idx) => {
                        let len = music_alt_list.len();
                        if let Some(m) = music_alt_list.get_mut(idx % len) {
                            m.play_stream();
                        }
                    }
                }
                current_music = desired;
                music_playing = true;
            } else if desired != current_music {
                match current_music {
                    MusicMode::Default => music_default.stop_stream(),
                    MusicMode::Alternate(idx) => {
                         let len = music_alt_list.len();
                         if let Some(m) = music_alt_list.get_mut(idx % len) {
                            m.stop_stream();
                        }
                    }
                }
                match desired {
                    MusicMode::Default => music_default.play_stream(),
                    MusicMode::Alternate(idx) => {
                         let len = music_alt_list.len();
                         if let Some(m) = music_alt_list.get_mut(idx % len) {
                            m.play_stream();
                        }
                    }
                }
                current_music = desired;
            }

            match current_music {
                MusicMode::Default => music_default.update_stream(),
                MusicMode::Alternate(idx) => {
                     let len = music_alt_list.len();
                     if let Some(m) = music_alt_list.get_mut(idx % len) {
                        m.update_stream();
                    }
                }
            }
        } else if music_playing {
            music_default.stop_stream();
            for m in music_alt_list.iter_mut() {
                m.stop_stream();
            }
            music_playing = false;
            current_music = MusicMode::Default;
        }

        let mut d = rl.begin_drawing(&thread);
        match screen {
            Screen::Menu => {
                let action = menu.update_and_draw(
                    &mut d,
                    screen_w,
                    screen_h,
                    mouse,
                    clicked,
                    &font,
                    &logo_texture,
                );

                match action {
                    MenuAction::Start => {
                        world = create_world(screen_w, screen_h, &bucket_texture);
                        screen = Screen::Playing;
                    }
                    MenuAction::Leaderboard => {
                        leaderboard.reload();
                        screen = Screen::Leaderboard;
                    }
                    MenuAction::None => {}
                }
            }
            Screen::Leaderboard => {
                 match leaderboard.update_and_draw(&mut d, screen_w, screen_h, mouse, clicked, &font) {
                    LeaderboardAction::Back => screen = Screen::Menu,
                    LeaderboardAction::None => {},
                 }
            }
            Screen::Playing => {
                // d.clear_background(Color::BLACK);
                let bg_scale = (screen_w / bg_texture.width as f32).max(screen_h / bg_texture.height as f32);
                let bg_dest_w = bg_texture.width as f32 * bg_scale;
                let bg_dest_h = bg_texture.height as f32 * bg_scale;
                let bg_dest_x = (screen_w - bg_dest_w) / 2.0;
                let bg_dest_y = (screen_h - bg_dest_h) / 2.0;
                d.draw_texture_pro(
                    &bg_texture,
                    Rectangle::new(0.0, 0.0, bg_texture.width as f32, bg_texture.height as f32),
                    Rectangle::new(bg_dest_x, bg_dest_y, bg_dest_w, bg_dest_h),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::WHITE,
                );

                render::draw_world(
                    &mut d,
                    &world,
                    &bucket_texture,
                    &normal_texture,
                    &angel_texture,
                    &devil_texture,
                    &crying_cat_texture,
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
                let bg_scale = (screen_w / bg_texture.width as f32).max(screen_h / bg_texture.height as f32);
                let bg_dest_w = bg_texture.width as f32 * bg_scale;
                let bg_dest_h = bg_texture.height as f32 * bg_scale;
                let bg_dest_x = (screen_w - bg_dest_w) / 2.0;
                let bg_dest_y = (screen_h - bg_dest_h) / 2.0;
                d.draw_texture_pro(
                    &bg_texture,
                    Rectangle::new(0.0, 0.0, bg_texture.width as f32, bg_texture.height as f32),
                    Rectangle::new(bg_dest_x, bg_dest_y, bg_dest_w, bg_dest_h),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::WHITE,
                );

                render::draw_world(
                    &mut d,
                    &world,
                    &bucket_texture,
                    &normal_texture,
                    &angel_texture,
                    &devil_texture,
                    &crying_cat_texture,
                );

                match pause_menu.update_and_draw(&mut d, world.score(), screen_w, screen_h, mouse, clicked) {
                    PauseAction::Resume => screen = Screen::Playing,
                    PauseAction::Exit => {
                        game_over.set_score(world.score());
                        screen = Screen::GameOver;
                    }
                    PauseAction::None => {},
                }
            }
            Screen::GameOver => {
                 d.clear_background(Color::BLACK);
                 /* 
                 render::draw_world(
                    &mut d,
                    &world,
                    &bucket_texture,
                    &normal_texture,
                    &angel_texture,
                    &devil_texture, 
                    &crying_cat_texture,
                );
                */

                 match game_over.draw(&mut d, screen_w, screen_h, mouse, clicked, &font) {
                    GameOverAction::Submit => {
                        leaderboard.reload();
                        screen = Screen::Leaderboard;
                    },
                    GameOverAction::None => {},
                 }
            }
        }
    }
}

fn create_world(screen_w: f32, screen_h: f32, bucket_texture: &Texture2D) -> World {
    let max_angry = rng::get_random_value(config::ANGRY_BAR_MIN_MAX, config::ANGRY_BAR_MAX_MAX);
    let mut world = World::new(screen_w, screen_h, max_angry);
    let bucket_frame_w =
        bucket_texture.width as f32 / config::BUCKET_FRAME_COLS as f32 * config::BUCKET_DRAW_SCALE;
    let bucket_frame_h = bucket_texture.height as f32 / config::BUCKET_FRAME_ROWS as f32
        * config::BUCKET_DRAW_SCALE;
    world.bucket.set_size(Vector2::new(bucket_frame_w, bucket_frame_h), screen_w, screen_h);
    world.set_base_bucket_size(Vector2::new(bucket_frame_w, bucket_frame_h));
    world
}
