/// Function plotting module — adapted from MathLikeAnim-rs plotting concepts.
/// Provides axes drawing, tick marks, labels, and f(x) function evaluation/plotting.

use tiny_skia::*;
use exmex::prelude::*;

/// Evaluate a math expression string at a given x value.
/// Supports: sin, cos, tan, sqrt, abs, ln, log, exp, and standard operators.
pub fn eval_expr(expr_str: &str, x: f64) -> Option<f64> {
    // Pre-process common math notation
    let processed = expr_str
        .replace("pi", &format!("{}", std::f64::consts::PI))
        .replace("PI", &format!("{}", std::f64::consts::PI));

    let expr = FlatEx::<f64>::parse(&processed).ok()?;
    expr.eval(&[x]).ok()
}

/// Draw labeled axes with tick marks and optional grid.
pub fn draw_axes(
    pixmap: &mut Pixmap,
    cx: f32,          // center x
    cy: f32,          // center y
    width: f32,       // total width
    height: f32,      // total height
    x_range: (f64, f64), // x-axis range (e.g., -5.0 to 5.0)
    y_range: (f64, f64), // y-axis range (e.g., -3.0 to 3.0)
    color: Color,
    thickness: f32,
    draw_grid_lines: bool,
) {
    let half_w = width / 2.0;
    let half_h = height / 2.0;
    let left = cx - half_w;
    let right = cx + half_w;
    let top = cy - half_h;
    let bottom = cy + half_h;

    let mut paint = Paint::default();
    paint.anti_alias = true;

    // Grid color (semi-transparent)
    if draw_grid_lines {
        let grid_color = Color::from_rgba8(
            (color.red() * 255.0) as u8,
            (color.green() * 255.0) as u8,
            (color.blue() * 255.0) as u8,
            30, // very subtle
        );
        paint.set_color(grid_color);

        let mut stroke = Stroke::default();
        stroke.width = thickness * 0.3;

        // Vertical grid lines
        let x_step = find_nice_step(x_range.0, x_range.1);
        let mut x_val = (x_range.0 / x_step).ceil() * x_step;
        while x_val <= x_range.1 {
            let px = map_range(x_val, x_range.0, x_range.1, left as f64, right as f64) as f32;
            let mut pb = PathBuilder::new();
            pb.move_to(px, top);
            pb.line_to(px, bottom);
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }
            x_val += x_step;
        }

        // Horizontal grid lines
        let y_step = find_nice_step(y_range.0, y_range.1);
        let mut y_val = (y_range.0 / y_step).ceil() * y_step;
        while y_val <= y_range.1 {
            let py = map_range(y_val, y_range.0, y_range.1, bottom as f64, top as f64) as f32;
            let mut pb = PathBuilder::new();
            pb.move_to(left, py);
            pb.line_to(right, py);
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }
            y_val += y_step;
        }
    }

    // Main axes
    paint.set_color(color);
    let mut stroke = Stroke::default();
    stroke.width = thickness;
    stroke.line_cap = LineCap::Round;

    // X axis (horizontal) — at y=0
    let y_zero = map_range(0.0, y_range.0, y_range.1, bottom as f64, top as f64) as f32;
    let y_zero = y_zero.max(top).min(bottom);
    {
        let mut pb = PathBuilder::new();
        pb.move_to(left, y_zero);
        pb.line_to(right, y_zero);
        if let Some(path) = pb.finish() {
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }
    }

    // Y axis (vertical) — at x=0
    let x_zero = map_range(0.0, x_range.0, x_range.1, left as f64, right as f64) as f32;
    let x_zero = x_zero.max(left).min(right);
    {
        let mut pb = PathBuilder::new();
        pb.move_to(x_zero, top);
        pb.line_to(x_zero, bottom);
        if let Some(path) = pb.finish() {
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }
    }

    // Arrowheads
    draw_arrow_tip(pixmap, right, y_zero, 1.0, 0.0, thickness * 3.0, color);
    draw_arrow_tip(pixmap, x_zero, top, 0.0, -1.0, thickness * 3.0, color);

    // Tick marks on X axis
    let x_step = find_nice_step(x_range.0, x_range.1);
    let tick_len = 6.0_f32;
    let mut x_val = (x_range.0 / x_step).ceil() * x_step;
    while x_val <= x_range.1 {
        if x_val.abs() > 0.001 {
            let px = map_range(x_val, x_range.0, x_range.1, left as f64, right as f64) as f32;
            let mut pb = PathBuilder::new();
            pb.move_to(px, y_zero - tick_len);
            pb.line_to(px, y_zero + tick_len);
            if let Some(path) = pb.finish() {
                let mut tick_stroke = Stroke::default();
                tick_stroke.width = thickness * 0.7;
                pixmap.stroke_path(&path, &paint, &tick_stroke, Transform::identity(), None);
            }
        }
        x_val += x_step;
    }

    // Tick marks on Y axis
    let y_step = find_nice_step(y_range.0, y_range.1);
    let mut y_val = (y_range.0 / y_step).ceil() * y_step;
    while y_val <= y_range.1 {
        if y_val.abs() > 0.001 {
            let py = map_range(y_val, y_range.0, y_range.1, bottom as f64, top as f64) as f32;
            let mut pb = PathBuilder::new();
            pb.move_to(x_zero - tick_len, py);
            pb.line_to(x_zero + tick_len, py);
            if let Some(path) = pb.finish() {
                let mut tick_stroke = Stroke::default();
                tick_stroke.width = thickness * 0.7;
                pixmap.stroke_path(&path, &paint, &tick_stroke, Transform::identity(), None);
            }
        }
        y_val += y_step;
    }
}

/// Plot a mathematical function f(x) on the canvas.
pub fn draw_function_plot(
    pixmap: &mut Pixmap,
    cx: f32,
    cy: f32,
    width: f32,
    height: f32,
    expr_str: &str,
    x_range: (f64, f64),
    y_range: (f64, f64),
    color: Color,
    thickness: f32,
    progress: f32,  // 0.0 to 1.0 for progressive draw
) {
    let half_w = width / 2.0;
    let half_h = height / 2.0;
    let left = cx - half_w;
    let right = cx + half_w;
    let top = cy - half_h;
    let bottom = cy + half_h;

    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;

    let steps = (width * 2.0) as usize; // high resolution
    let visible_steps = ((steps as f32 * progress) as usize).max(1);

    let mut pb = PathBuilder::new();
    let mut started = false;
    let mut prev_valid = false;

    for i in 0..=visible_steps {
        let t = i as f64 / steps as f64;
        let x_val = x_range.0 + t * (x_range.1 - x_range.0);

        if let Some(y_val) = eval_expr(expr_str, x_val) {
            if y_val.is_finite() && y_val >= y_range.0 && y_val <= y_range.1 {
                let px = map_range(x_val, x_range.0, x_range.1, left as f64, right as f64) as f32;
                let py = map_range(y_val, y_range.0, y_range.1, bottom as f64, top as f64) as f32;

                if !started || !prev_valid {
                    pb.move_to(px, py);
                    started = true;
                } else {
                    pb.line_to(px, py);
                }
                prev_valid = true;
            } else {
                prev_valid = false;
            }
        } else {
            prev_valid = false;
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

// ===== Utility Functions =====

/// Map a value from one range to another.
fn map_range(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
    let normalized = (value - from_min) / (from_max - from_min);
    to_min + normalized * (to_max - to_min)
}

/// Find a "nice" step size for tick marks.
fn find_nice_step(min: f64, max: f64) -> f64 {
    let range = max - min;
    let rough_step = range / 10.0;
    let magnitude = 10.0_f64.powf(rough_step.log10().floor());
    let normalized = rough_step / magnitude;

    let nice = if normalized <= 1.5 {
        1.0
    } else if normalized <= 3.5 {
        2.0
    } else if normalized <= 7.5 {
        5.0
    } else {
        10.0
    };

    nice * magnitude
}

/// Draw an arrowhead at a point in a given direction.
fn draw_arrow_tip(pixmap: &mut Pixmap, x: f32, y: f32, dx: f32, dy: f32, size: f32, color: Color) {
    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;

    let px = -dy;
    let py = dx;

    let mut pb = PathBuilder::new();
    pb.move_to(x, y);
    pb.line_to(x - dx * size + px * size * 0.35, y - dy * size + py * size * 0.35);
    pb.line_to(x - dx * size - px * size * 0.35, y - dy * size - py * size * 0.35);
    pb.close();

    if let Some(path) = pb.finish() {
        pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    }
}
