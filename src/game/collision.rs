use::raylib::prelude::*;

pub fn check_collision(center: Vector2, radius: f32, rect: Rectangle) -> bool {
    let closest_x = center.x.clamp(rect.x, rect.x + rect.width);
    let closest_y = center.y.clamp(rect.y, rect.y + rect.height);

    let distance_x = center.x - closest_x;
    let distance_y = center.y - closest_y;

    let distance_squared = distance_x * distance_x + distance_y * distance_y;
    distance_squared < radius * radius
}