use raylib::prelude::*;
use crate::config;
use crate::game::world::World;

pub fn draw_world(d: &mut RaylibDrawHandle, world: &World, bucket_texture: &Texture2D) {
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
        d.draw_circle(obj.pos.x as i32, obj.pos.y as i32, obj.radius, Color::GOLD);
    }
}
