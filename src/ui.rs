use raylib::prelude::*;

pub fn draw_button(
    d: &mut RaylibDrawHandle,
    rect: Rectangle,
    label: &str,
    mouse: Vector2,
    clicked: bool,
    font: &WeakFont,
    base: Color,
    hover: Color,
    text: Color,
    border: Color,
) -> bool {
    let hovered = rect.check_collision_point_rec(mouse);
    let fill = if hovered { hover } else { base };

    // Shadow
    let shadow = Color::new(162, 74, 120, 60);
    d.draw_rectangle_rounded(
        Rectangle {
            y: rect.y + 4.0,
            ..rect
        },
        0.35,
        8,
        shadow,
    );

    // Background
    d.draw_rectangle_rounded(rect, 0.35, 8, fill);

    // Border
    d.draw_rectangle_rounded_lines_ex(rect, 0.35, 8, 2.0, border);

    // Text
    let font_size = 22.0;
    let spacing = 1.0;
    let text_size = font.measure_text(label, font_size, spacing);

    d.draw_text_ex(
        font,
        label,
        Vector2::new(
            rect.x + (rect.width - text_size.x) / 2.0,
            rect.y + (rect.height - text_size.y) / 2.0 - 2.0,
        ),
        font_size,
        spacing,
        text,
    );

    hovered && clicked
}
