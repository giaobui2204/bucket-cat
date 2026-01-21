use raylib::prelude::*;
use crate::config;
use crate::game::objects::CatKind;
use crate::game::world::World;

pub fn draw_world(
    d: &mut RaylibDrawHandle,
    world: &World,
    bucket_texture: &Texture2D,
    normal_texture: &Texture2D,
    angel_texture: &Texture2D,
    devil_texture: &Texture2D,
) {
    let r = world.bucket.rect();
    let source = Rectangle {
        x: (world.bucket.frame() as i32 % config::BUCKET_FRAME_COLS) as f32
            * (bucket_texture.width as f32 / config::BUCKET_FRAME_COLS as f32),
        y: (world.bucket.frame() as i32 / config::BUCKET_FRAME_COLS) as f32
            * (bucket_texture.height as f32 / config::BUCKET_FRAME_ROWS as f32),
        width: bucket_texture.width as f32 / config::BUCKET_FRAME_COLS as f32,
        height: bucket_texture.height as f32 / config::BUCKET_FRAME_ROWS as f32,
    };
    let dest = Rectangle {
        x: r.x,
        y: r.y,
        width: r.width,
        height: r.height,
    };
    d.draw_texture_pro(
        bucket_texture,
        source,
        dest,
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );

    for obj in &world.objects {
        let texture = match obj.kind() {
            CatKind::Normal => normal_texture,
            CatKind::Angel => angel_texture,
            CatKind::Devil | CatKind::Explode => devil_texture,
        };
        let frame_w = texture.width as f32 / config::OBJ_FRAME_COLS as f32;
        let frame_h = texture.height as f32 / config::OBJ_FRAME_ROWS as f32;
        let source = Rectangle {
            x: (obj.frame() as i32 % config::OBJ_FRAME_COLS) as f32 * frame_w,
            y: (obj.frame() as i32 / config::OBJ_FRAME_COLS) as f32 * frame_h,
            width: frame_w,
            height: frame_h,
        };
        let dest = Rectangle {
            x: obj.pos.x - (frame_w * config::OBJ_DRAW_SCALE) / 2.0,
            y: obj.pos.y - (frame_h * config::OBJ_DRAW_SCALE) / 2.0,
            width: frame_w * config::OBJ_DRAW_SCALE,
            height: frame_h * config::OBJ_DRAW_SCALE,
        };
        d.draw_texture_pro(
            texture,
            source,
            dest,
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
    }

    // Placeholder score text
    let score_text = format!("{}: {}", config::GAME_SCORE_LABEL, world.score());
    d.draw_text(&score_text, 12, 12, 22, Color::WHITE);

    if let Some((pos, t)) = world.explosion_effect() {
        let radius = 18.0 + t * 90.0;
        let alpha = ((1.0 - t) * 200.0).clamp(0.0, 200.0) as u8;
        let color = Color::new(255, 150, 210, alpha);
        d.draw_circle_lines(pos.x as i32, pos.y as i32, radius, color);
        d.draw_circle_lines(
            pos.x as i32,
            pos.y as i32,
            radius * 0.6,
            Color::new(255, 210, 235, alpha),
        );
    }

    if let Some(message) = world.effect_message() {
        let font_size = 22;
        let text_w = d.measure_text(message, font_size);
        let pad_x = 18;
        let pad_y = 8;
        let box_w = text_w + pad_x * 2;
        let box_h = font_size + pad_y * 2;
        let box_x = ((config::SCREEN_W - box_w) / 2).max(0);
        let box_y = 50;
        let alpha = (world.effect_message_alpha() * 220.0).clamp(0.0, 220.0) as u8;
        d.draw_rectangle(
            box_x,
            box_y,
            box_w,
            box_h,
            Color::new(255, 230, 245, alpha),
        );
        d.draw_rectangle_lines(
            box_x,
            box_y,
            box_w,
            box_h,
            Color::new(233, 130, 180, alpha),
        );
        d.draw_text(
            message,
            box_x + pad_x,
            box_y + pad_y,
            font_size,
            Color::new(109, 43, 80, alpha),
        );
    }
}
