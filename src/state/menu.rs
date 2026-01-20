use raylib::prelude::*;
use crate::config;
use crate::ui;

pub enum MenuAction {
    None,
    Start,
}

enum MenuView {
    Main,
    Leaderboard,
}

pub struct MenuState {
    view: MenuView,
}

impl MenuState {
    pub fn new() -> Self {
        Self { view: MenuView::Main }
    }

    pub fn update_and_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        screen_w: f32,
        screen_h: f32,
        mouse: Vector2,
        clicked: bool,
        font: &WeakFont,
        logo_texture: &Texture2D,
    ) -> MenuAction {
        draw_menu_background(d, screen_w, screen_h);

        match self.view {
            MenuView::Main => {
                let button_w = 240.0;
                let button_h = 56.0;
                let button_stack_h = button_h * 2.0 + 16.0;
                let top_padding = 28.0;
                let logo_gap = 24.0;
                let bottom_padding = 28.0;
                let max_logo_h =
                    (screen_h - top_padding - logo_gap - button_stack_h - bottom_padding).max(80.0);
                let logo_scale_w = (screen_w * 0.6 / logo_texture.width as f32).min(1.0);
                let logo_scale_h = (max_logo_h / logo_texture.height as f32).min(1.0);
                let logo_scale = logo_scale_w.min(logo_scale_h);
                let logo_w = logo_texture.width as f32 * logo_scale;
                let logo_h = logo_texture.height as f32 * logo_scale;
                let logo_x = (screen_w - logo_w) / 2.0;
                let logo_y = top_padding;
                d.draw_texture_ex(
                    logo_texture,
                    Vector2::new(logo_x, logo_y),
                    0.0,
                    logo_scale,
                    Color::WHITE,
                );

                let button_x = (screen_w - button_w) / 2.0;
                let start_y = logo_y + logo_h + logo_gap;
                let start_rect = Rectangle {
                    x: button_x,
                    y: start_y,
                    width: button_w,
                    height: button_h,
                };
                let leaderboard_rect = Rectangle {
                    x: button_x,
                    y: start_y + button_h + 16.0,
                    width: button_w,
                    height: button_h,
                };

                if ui::draw_button(
                    d,
                    start_rect,
                    "Start",
                    mouse,
                    clicked,
                    font,
                    config::COLOR_ACCENT,
                    config::COLOR_ACCENT_HOVER,
                    config::COLOR_ACCENT_TEXT,
                    config::COLOR_ACCENT_BORDER,
                ) {
                    return MenuAction::Start;
                }

                if ui::draw_button(
                    d,
                    leaderboard_rect,
                    "Leaderboard",
                    mouse,
                    clicked,
                    font,
                    config::COLOR_LIGHT_BG,
                    config::COLOR_LIGHT_HOVER,
                    config::COLOR_ACCENT_TEXT,
                    config::COLOR_ACCENT_BORDER,
                ) {
                    self.view = MenuView::Leaderboard;
                }
            }
            MenuView::Leaderboard => {
                let title = "Leaderboard";
                let title_size = 32;
                let title_w = font.measure_text(title, title_size as f32, 1.0).x;
                d.draw_text(
                    title,
                    ((screen_w - title_w) / 2.0) as i32,
                    60,
                    title_size,
                    config::COLOR_ACCENT_TEXT,
                );
                let subtitle = "Coming soon";
                let subtitle_size = 18;
                let subtitle_w = font.measure_text(subtitle, subtitle_size as f32, 1.0).x;
                d.draw_text(
                    subtitle,
                    ((screen_w - subtitle_w) / 2.0) as i32,
                    110,
                    subtitle_size,
                    config::COLOR_LIGHT_TEXT_SUB,
                );

                let back_rect = Rectangle {
                    x: (screen_w - 180.0) / 2.0,
                    y: screen_h - 90.0,
                    width: 180.0,
                    height: 48.0,
                };
                if ui::draw_button(
                    d,
                    back_rect,
                    "Back",
                    mouse,
                    clicked,
                    font,
                    config::COLOR_LIGHT_BG,
                    config::COLOR_LIGHT_HOVER,
                    config::COLOR_ACCENT_TEXT,
                    config::COLOR_ACCENT_BORDER,
                ) {
                    self.view = MenuView::Main;
                }
            }
        }

        MenuAction::None
    }
}

fn draw_menu_background(d: &mut RaylibDrawHandle, screen_w: f32, screen_h: f32) {
    d.draw_rectangle_gradient_v(
        0,
        0,
        screen_w as i32,
        screen_h as i32,
        config::COLOR_MENU_BG_START,
        config::COLOR_MENU_BG_END,
    );
    d.draw_circle(
        (screen_w * 0.2) as i32,
        (screen_h * 0.28) as i32,
        60.0,
        config::COLOR_MENU_CIRCLE,
    );
    d.draw_circle(
        (screen_w * 0.82) as i32,
        (screen_h * 0.18) as i32,
        46.0,
        config::COLOR_MENU_CIRCLE,
    );
    d.draw_circle(
        (screen_w * 0.78) as i32,
        (screen_h * 0.72) as i32,
        70.0,
        config::COLOR_MENU_CIRCLE,
    );
}