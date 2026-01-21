use raylib::prelude::*;
use crate::config;
use crate::ui;
use crate::state::leaderboard::LeaderboardState;

pub enum GameOverAction {
    None,
    Submit,
}

pub struct GameOverState {
    name_buffer: String,
    score: i32,
}

impl GameOverState {
    pub fn new() -> Self {
        Self {
            name_buffer: String::new(),
            score: 0,
        }
    }
    
    pub fn set_score(&mut self, score: i32) {
        self.score = score;
        self.name_buffer.clear();
    }

    pub fn update_input(&mut self, rl: &mut RaylibHandle) {
        while let Some(char_code) = rl.get_char_pressed() {
             if self.name_buffer.len() < 12 && (char_code.is_ascii_graphic() || char_code == ' ') {
                self.name_buffer.push(char_code);
             }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            self.name_buffer.pop();
        }
    }

    pub fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        screen_w: f32,
        screen_h: f32,
        mouse: Vector2,
        clicked: bool,
        font: &WeakFont,
    ) -> GameOverAction {
        ui::draw_menu_background(d, screen_w, screen_h);
        // Draw semi-transparent overlay instead so we can see the catastrophe
        // d.draw_rectangle(0, 0, screen_w as i32, screen_h as i32, Color::new(0, 0, 0, 150));

        let title = "Game Over";
        let title_size = 40.0;
        let spacing = 1.0;
        let title_w = font.measure_text(title, title_size, spacing).x;

        d.draw_text_ex(
            font,
            title,
            Vector2::new((screen_w - title_w) / 2.0, 60.0),
            title_size,
            spacing,
            config::COLOR_ACCENT_TEXT,
        );

        let score_text = format!("Score: {}", self.score);
        let score_size = 30.0;
        let score_w = font.measure_text(&score_text, score_size, spacing).x;
        d.draw_text_ex(
            font,
            &score_text,
            Vector2::new((screen_w - score_w) / 2.0, 120.0),
            score_size,
            spacing,
            config::COLOR_ACCENT_TEXT,
        );

        // Input Box
        let input_w = 300.0;
        let input_h = 50.0;
        let input_x = (screen_w - input_w) / 2.0;
        let input_y = 200.0;
        let input_rect = Rectangle::new(input_x, input_y, input_w, input_h);

        d.draw_rectangle_rec(input_rect, config::COLOR_LIGHT_BG);
        d.draw_rectangle_lines_ex(input_rect, 2.0, config::COLOR_ACCENT_BORDER);

        let prompt = "Enter Name:";
        let prompt_size = 20.0;
        d.draw_text_ex(
            font,
            prompt,
            Vector2::new(input_x, input_y - 25.0),
            prompt_size,
            spacing,
            config::COLOR_ACCENT_TEXT,
        );
        
        // Input handled in update_input
        // Draw input field

        let name_text = &self.name_buffer;
        let name_size = 30.0;
        // Blinking cursor - approximate time checking without rl handle
        // We can use frame count if we had it, or just always draw cursor for now to simplify
        // Or if we need time, we can pass it in. RaylibDrawHandle doesn't give time.
        // Let's passed 'frame_counter' or similar if needed. For now, static cursor or simple block.
        
        let cursor_visible = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() / 500) % 2 == 0;
        
        if cursor_visible {
             d.draw_text_ex(font, "_", Vector2::new(input_x + 10.0 + font.measure_text(name_text, name_size, spacing).x, input_y + 10.0), name_size, spacing, config::COLOR_ACCENT_TEXT);
        }
        
        d.draw_text_ex(
            font,
            name_text,
            Vector2::new(input_x + 10.0, input_y + 10.0),
            name_size,
            spacing,
            config::COLOR_ACCENT_TEXT,
        );
        

        let submit_rect = Rectangle {
            x: (screen_w - 180.0) / 2.0,
            y: input_y + input_h + 40.0,
            width: 180.0,
            height: 48.0,
        };

        if ui::draw_button(
            d,
            submit_rect,
            "Save Score",
            mouse,
            clicked,
            font,
            config::COLOR_ACCENT,
            config::COLOR_ACCENT_HOVER,
            config::COLOR_ACCENT_TEXT,
            config::COLOR_ACCENT_BORDER,
        ) {
            if !self.name_buffer.is_empty() {
                LeaderboardState::add_score(&self.name_buffer, self.score);
                return GameOverAction::Submit;
            }
        }

        GameOverAction::None
    }
}
