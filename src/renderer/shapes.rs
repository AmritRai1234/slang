use tiny_skia::*;
use crate::renderer::scene::SceneObject;
use crate::renderer::text::TextRenderer;
use crate::parser::ast::ShapeKind;

/// Draw a single SceneObject onto the pixmap.
pub fn draw_object(pixmap: &mut Pixmap, obj: &SceneObject, _canvas_w: f64, _canvas_h: f64, text_renderer: &TextRenderer) {
    if obj.opacity <= 0.0 {
        return;
    }

    let [r, g, b, a] = obj.effective_color();
    let color = Color::from_rgba8(r, g, b, a);

    match &obj.shape {
        ShapeKind::Circle => {
            draw_circle(pixmap, obj.x as f32, obj.y as f32, (obj.radius * obj.scale) as f32, color);
        }
        ShapeKind::Square => {
            let half = (obj.size * obj.scale / 2.0) as f32;
            draw_rect(pixmap, obj.x as f32 - half, obj.y as f32 - half, (obj.size * obj.scale) as f32, (obj.size * obj.scale) as f32, color);
        }
        ShapeKind::Rectangle => {
            let w = (obj.size * obj.scale) as f32;
            let h = (obj.size * obj.scale * 0.6) as f32;
            draw_rect(pixmap, obj.x as f32 - w / 2.0, obj.y as f32 - h / 2.0, w, h, color);
        }
        ShapeKind::Triangle => {
            let s = (obj.size * obj.scale) as f32;
            draw_triangle(pixmap, obj.x as f32, obj.y as f32, s, color);
        }
        ShapeKind::Line => {
            let ex = obj.end_x.unwrap_or(obj.x + 100.0) as f32;
            let ey = obj.end_y.unwrap_or(obj.y) as f32;
            draw_line(pixmap, obj.x as f32, obj.y as f32, ex, ey, obj.thickness as f32, color);
        }
        ShapeKind::Arrow => {
            let ex = obj.end_x.unwrap_or(obj.x + 100.0) as f32;
            let ey = obj.end_y.unwrap_or(obj.y) as f32;
            draw_line(pixmap, obj.x as f32, obj.y as f32, ex, ey, obj.thickness as f32, color);
            draw_arrowhead(pixmap, obj.x as f32, obj.y as f32, ex, ey, 12.0, color);
        }
        ShapeKind::Text(content) => {
            let display_text = if let Some(n) = obj.visible_chars {
                &content[..n.min(content.len())]
            } else {
                content.as_str()
            };
            if !display_text.is_empty() {
                text_renderer.draw_text(pixmap, obj.x as f32, obj.y as f32, display_text, obj.size as f32, color);
            }
        }
        ShapeKind::Math(expr) => {
            let display_text = if let Some(n) = obj.visible_chars {
                &expr[..n.min(expr.len())]
            } else {
                expr.as_str()
            };
            if !display_text.is_empty() {
                text_renderer.draw_math(pixmap, obj.x as f32, obj.y as f32, display_text, obj.size as f32, color);
            }
        }
        ShapeKind::Wave { amplitude, frequency } => {
            draw_wave(
                pixmap,
                obj.x as f32,
                obj.y as f32,
                (obj.size * obj.scale) as f32,
                *amplitude as f32,
                *frequency as f32,
                obj.wave_progress as f32,
                obj.thickness as f32,
                color,
            );
        }
        ShapeKind::Grid => {
            draw_grid(
                pixmap,
                obj.x as f32,
                obj.y as f32,
                (obj.size * obj.scale) as f32,
                obj.thickness as f32,
                color,
            );
        }
        ShapeKind::Curve { points } => {
            if points.len() >= 2 {
                draw_bezier_curve(pixmap, points, obj.thickness as f32, color);
            }
        }
        ShapeKind::NumberAxis { x_range, y_range } => {
            crate::renderer::plotting::draw_axes(
                pixmap,
                obj.x as f32,
                obj.y as f32,
                (obj.size * obj.scale) as f32,
                (obj.size * obj.scale * 0.6) as f32,
                *x_range,
                *y_range,
                color,
                obj.thickness as f32,
                true,
            );
        }
        ShapeKind::FunctionPlot { expr, x_range, y_range } => {
            crate::renderer::plotting::draw_function_plot(
                pixmap,
                obj.x as f32,
                obj.y as f32,
                (obj.size * obj.scale) as f32,
                (obj.size * obj.scale * 0.6) as f32,
                expr,
                *x_range,
                *y_range,
                color,
                obj.thickness as f32,
                obj.wave_progress as f32, // reuse wave_progress for progressive draw
            );
        }
    }
}

fn draw_circle(pixmap: &mut Pixmap, cx: f32, cy: f32, radius: f32, color: Color) {
    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;

    let mut pb = PathBuilder::new();
    let steps = 64;
    for i in 0..=steps {
        let angle = 2.0 * std::f32::consts::PI * (i as f32) / (steps as f32);
        let x = cx + radius * angle.cos();
        let y = cy + radius * angle.sin();
        if i == 0 {
            pb.move_to(x, y);
        } else {
            pb.line_to(x, y);
        }
    }
    pb.close();

    if let Some(path) = pb.finish() {
        pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    }
}

fn draw_rect(pixmap: &mut Pixmap, x: f32, y: f32, w: f32, h: f32, color: Color) {
    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;

    let rect = Rect::from_xywh(x, y, w, h);
    if let Some(rect) = rect {
        let path = PathBuilder::from_rect(rect);
        pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    }
}

fn draw_triangle(pixmap: &mut Pixmap, cx: f32, cy: f32, size: f32, color: Color) {
    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;

    let half = size / 2.0;
    let height = size * 0.866;

    let mut pb = PathBuilder::new();
    pb.move_to(cx, cy - height * 0.67);
    pb.line_to(cx - half, cy + height * 0.33);
    pb.line_to(cx + half, cy + height * 0.33);
    pb.close();

    if let Some(path) = pb.finish() {
        pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    }
}

fn draw_line(pixmap: &mut Pixmap, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) {
    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;

    let mut pb = PathBuilder::new();
    pb.move_to(x1, y1);
    pb.line_to(x2, y2);

    if let Some(path) = pb.finish() {
        let mut stroke = Stroke::default();
        stroke.width = thickness;
        stroke.line_cap = LineCap::Round;
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
}

fn draw_arrowhead(pixmap: &mut Pixmap, x1: f32, y1: f32, x2: f32, y2: f32, size: f32, color: Color) {
    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;

    let dx = x2 - x1;
    let dy = y2 - y1;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.01 {
        return;
    }
    let ux = dx / len;
    let uy = dy / len;
    let px = -uy;
    let py = ux;

    let mut pb = PathBuilder::new();
    pb.move_to(x2, y2);
    pb.line_to(x2 - ux * size + px * size * 0.4, y2 - uy * size + py * size * 0.4);
    pb.line_to(x2 - ux * size - px * size * 0.4, y2 - uy * size - py * size * 0.4);
    pb.close();

    if let Some(path) = pb.finish() {
        pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    }
}

/// Draw a smooth sine wave.
fn draw_wave(
    pixmap: &mut Pixmap,
    cx: f32,
    cy: f32,
    width: f32,
    amplitude: f32,
    frequency: f32,
    progress: f32, // 0.0 to 1.0 — how much of the wave to draw
    thickness: f32,
    color: Color,
) {
    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;

    let half_w = width / 2.0;
    let start_x = cx - half_w;
    let steps = (width * 2.0) as usize; // high detail for smooth wave
    let visible_steps = ((steps as f32 * progress) as usize).max(1);

    let mut pb = PathBuilder::new();
    for i in 0..=visible_steps {
        let t = i as f32 / steps as f32;
        let x = start_x + t * width;
        let y = cy + amplitude * (t * frequency * 2.0 * std::f32::consts::PI).sin();
        if i == 0 {
            pb.move_to(x, y);
        } else {
            pb.line_to(x, y);
        }
    }

    if let Some(path) = pb.finish() {
        let mut stroke = Stroke::default();
        stroke.width = thickness;
        stroke.line_cap = LineCap::Round;
        stroke.line_join = LineJoin::Round;
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
}

/// Draw a coordinate grid with axes.
fn draw_grid(
    pixmap: &mut Pixmap,
    cx: f32,
    cy: f32,
    size: f32,
    thickness: f32,
    color: Color,
) {
    let half = size / 2.0;
    let grid_color = Color::from_rgba8(
        (color.red() * 255.0) as u8,
        (color.green() * 255.0) as u8,
        (color.blue() * 255.0) as u8,
        60, // semi-transparent grid lines
    );
    let axis_color = color;

    // Draw grid lines
    let grid_step = size / 10.0;
    let mut i = -5.0_f32;
    while i <= 5.0 {
        let offset = i * grid_step;

        // Vertical grid line
        draw_line(pixmap, cx + offset, cy - half, cx + offset, cy + half, thickness * 0.5, grid_color);
        // Horizontal grid line
        draw_line(pixmap, cx - half, cy + offset, cx + half, cy + offset, thickness * 0.5, grid_color);

        i += 1.0;
    }

    // Draw main axes (thicker)
    draw_line(pixmap, cx - half, cy, cx + half, cy, thickness * 1.5, axis_color);
    draw_line(pixmap, cx, cy - half, cx, cy + half, thickness * 1.5, axis_color);

    // Arrowheads on axes
    draw_arrowhead(pixmap, cx - half, cy, cx + half, cy, 10.0, axis_color);
    draw_arrowhead(pixmap, cx, cy + half, cx, cy - half, 10.0, axis_color);
}

/// Draw a smooth bezier curve through a set of points.
fn draw_bezier_curve(
    pixmap: &mut Pixmap,
    points: &[(f64, f64)],
    thickness: f32,
    color: Color,
) {
    if points.len() < 2 {
        return;
    }

    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;

    let mut pb = PathBuilder::new();
    pb.move_to(points[0].0 as f32, points[0].1 as f32);

    for i in 1..points.len() {
        pb.line_to(points[i].0 as f32, points[i].1 as f32);
    }

    if let Some(path) = pb.finish() {
        let mut stroke = Stroke::default();
        stroke.width = thickness;
        stroke.line_cap = LineCap::Round;
        stroke.line_join = LineJoin::Round;
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
}
