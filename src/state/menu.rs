use raylib::prelude::*;
use crate::config;
use crate::ui;

pub enum MenuAction {
    None,
    Start,
    Leaderboard,
}

pub struct MenuState;

impl MenuState {
    pub fn new() -> Self {
        Self
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
        ui::draw_menu_background(d, screen_w, screen_h);

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
            return MenuAction::Leaderboard;
        }

        MenuAction::None
    }
}
