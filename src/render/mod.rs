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

    // Placeholder score text (UI polish in another task)
    let score_text = format!("{}: {}", config::GAME_SCORE_LABEL, world.score());
    d.draw_text(&score_text, 12, 12, 22, Color::WHITE);
}
