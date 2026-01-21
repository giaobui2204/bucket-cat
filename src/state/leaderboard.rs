use raylib::prelude::*;
use std::fs;
use std::path::Path;
use crate::config;
use crate::ui;

#[derive(Clone)]
pub struct HighScore {
    pub name: String,
    pub score: i32,
}

pub enum LeaderboardAction {
    None,
    Back,
}

pub struct LeaderboardState {
    high_scores: Vec<HighScore>,
}

impl LeaderboardState {
    pub fn new() -> Self {
        let mut high_scores = Self::load_scores();
        
        Self {
            high_scores,
        }
    }

    fn load_scores() -> Vec<HighScore> {
        let mut scores = Vec::new();
        if Path::new(config::SCORES_FILE).exists() {
            if let Ok(content) = fs::read_to_string(config::SCORES_FILE) {
                for line in content.lines() {
                    // split_once returns (first, rest), so "Name,Score" works
                    if let Some((name, score_str)) = line.rsplit_once(',') {
                        if let Ok(score) = score_str.trim().parse::<i32>() {
                            scores.push(HighScore {
                                name: name.trim().to_string(),
                                score,
                            });
                        }
                    }
                }
            }
        }
        scores.sort_by(|a, b| b.score.cmp(&a.score));
        scores
    }

    pub fn save_scores(scores: &[HighScore]) {
        let mut content = String::new();
        for score in scores {
            content.push_str(&format!("{},{}\n", score.name, score.score));
        }
        let _ = fs::write(config::SCORES_FILE, content);
    }
    
    // Add a public method to update scores from outside
    #[allow(dead_code)]
    pub fn add_score(name: &str, score: i32) {
        let mut scores = Self::load_scores();
        scores.push(HighScore { name: name.to_string(), score });
        scores.sort_by(|a, b| b.score.cmp(&a.score));
        if scores.len() > 10 {
            scores.truncate(10); // Keep top 10
        }
        Self::save_scores(&scores);
    }

    #[allow(dead_code)]
    pub fn reload(&mut self) {
        self.high_scores = Self::load_scores();
    }

    pub fn update_and_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        screen_w: f32,
        screen_h: f32,
        mouse: Vector2,
        clicked: bool,
        font: &WeakFont,
    ) -> LeaderboardAction {
        ui::draw_menu_background(d, screen_w, screen_h);

        let title = "Leaderboard";
        let title_size = 32.0;
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

        // Draw scores
        if self.high_scores.is_empty() {
            let msg = "Let's start a legendary";
            let msg_size = 24.0;
            let msg_w = font.measure_text(msg, msg_size, spacing).x;
            d.draw_text_ex(
                font,
                msg,
                Vector2::new((screen_w - msg_w) / 2.0, screen_h * 0.4),
                msg_size,
                spacing,
                config::COLOR_ACCENT_TEXT,
            );
        } else {
            let start_y = 125.0;
            let line_height = 27.0;
            let score_font_size = 22.0;

            for (i, entry) in self.high_scores.iter().enumerate() {
                if i >= 10 { break; }
                let y = start_y + i as f32 * line_height;
                let rank_str = format!("{}.", i + 1);
                
                // Rank
                d.draw_text_ex(font, &rank_str, Vector2::new(screen_w * 0.25, y), score_font_size, spacing, config::COLOR_ACCENT_TEXT);
                // Name
                d.draw_text_ex(font, &entry.name, Vector2::new(screen_w * 0.35, y), score_font_size, spacing, config::COLOR_ACCENT_TEXT);
                // Score
                let score_str = entry.score.to_string();
                let score_w = font.measure_text(&score_str, score_font_size, spacing).x;
                d.draw_text_ex(font, &score_str, Vector2::new(screen_w * 0.75 - score_w, y), score_font_size, spacing, config::COLOR_ACCENT_TEXT);
            }
        }

        let back_rect = Rectangle {
            x: (screen_w - 180.0) / 2.0,
            y: screen_h - 70.0,
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
            return LeaderboardAction::Back;
        }

        LeaderboardAction::None
    }
}
