use raylib::prelude::*;
use crate::game::world::World;

pub fn draw_world(d: &mut RaylibDrawHandle, world: &World) {
    let r = world.bucket.rect();
    d.draw_rectangle(r.x as i32, r.y as i32, r.width as i32, r.height as i32, Color::RAYWHITE);

    for obj in &world.objects {
        d.draw_circle(obj.pos.x as i32, obj.pos.y as i32, obj.radius, Color::GOLD);
    }
}
