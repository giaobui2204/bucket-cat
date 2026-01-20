use raylib::prelude::*;
use crate::ui;
use crate::config;

pub enum PauseAction {
    None,
    Resume,
    Exit,
}

pub struct PauseState;

impl PauseState {
    pub fn new() -> Self {
        Self
    }

    pub fn update_and_draw(
        &self,
        d: &mut RaylibDrawHandle,
        score: i32,
        screen_w: f32,
        screen_h: f32,
        mouse: Vector2,
        clicked: bool,
    ) -> PauseAction {
        // Overlay
        d.draw_rectangle(0, 0, screen_w as i32, screen_h as i32, Color::new(0, 0, 0, 150));

        // Score
        let score_text = format!("Score: {}", score);
        let font_size = 40;
        let text_width = d.measure_text(&score_text, font_size);
        d.draw_text(&score_text, (screen_w as i32 - text_width) / 2, 100, font_size, Color::WHITE);

        // Resume Button
        let btn_w = 200.0;
        let btn_h = 50.0;
        let resume_rect = Rectangle::new((screen_w - btn_w) / 2.0, 200.0, btn_w, btn_h);
        
        let font = d.get_font_default();

        if ui::draw_button(
            d, resume_rect, "Resume", mouse, clicked, &font,
            config::COLOR_ACCENT, config::COLOR_ACCENT_HOVER, config::COLOR_ACCENT_TEXT, config::COLOR_ACCENT_BORDER
        ) {
            return PauseAction::Resume;
        }

        // Exit Button
        let exit_rect = Rectangle::new((screen_w - btn_w) / 2.0, 270.0, btn_w, btn_h);

        if ui::draw_button(
            d, exit_rect, "Exit Game", mouse, clicked, &font,
            config::COLOR_LIGHT_BG, config::COLOR_LIGHT_HOVER, config::COLOR_ACCENT_TEXT, config::COLOR_ACCENT_BORDER
        ) {
            return PauseAction::Exit;
        }

        PauseAction::None
    }
}
