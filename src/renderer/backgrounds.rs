/// Procedural background rendering — powered by `noise` and `colorgrad` crates.
/// Provides gradient, noise, and radial backgrounds instead of flat colors.

use tiny_skia::*;
use noise::{NoiseFn, Perlin};

/// Background types that can be rendered.
#[derive(Debug, Clone)]
pub enum BackgroundKind {
    /// Solid flat color (default)
    Solid([u8; 4]),
    /// Linear gradient between two colors (top to bottom)
    Gradient([u8; 4], [u8; 4]),
    /// Radial gradient from center color to edge color
    Radial([u8; 4], [u8; 4]),
    /// Perlin noise-based subtle texture on top of a base color
    Noise([u8; 4]),
}

impl Default for BackgroundKind {
    fn default() -> Self {
        BackgroundKind::Solid([30, 30, 60, 255]) // dark blue default
    }
}

/// Render the background onto the pixmap.
pub fn draw_background(pixmap: &mut Pixmap, kind: &BackgroundKind, _frame_index: usize) {
    let w = pixmap.width() as usize;
    let h = pixmap.height() as usize;

    match kind {
        BackgroundKind::Solid(color) => {
            let c = Color::from_rgba8(color[0], color[1], color[2], color[3]);
            pixmap.fill(c);
        }
        BackgroundKind::Gradient(top_color, bottom_color) => {
            draw_linear_gradient(pixmap, w, h, top_color, bottom_color);
        }
        BackgroundKind::Radial(center_color, edge_color) => {
            draw_radial_gradient(pixmap, w, h, center_color, edge_color);
        }
        BackgroundKind::Noise(base_color) => {
            draw_noise_background(pixmap, w, h, base_color, _frame_index);
        }
    }
}

/// Linear gradient from top color to bottom color.
fn draw_linear_gradient(pixmap: &mut Pixmap, w: usize, h: usize, top: &[u8; 4], bottom: &[u8; 4]) {
    let data = pixmap.data_mut();
    
    for y in 0..h {
        let t = y as f64 / h as f64;
        let r = lerp_u8(top[0], bottom[0], t);
        let g = lerp_u8(top[1], bottom[1], t);
        let b = lerp_u8(top[2], bottom[2], t);
        let a = lerp_u8(top[3], bottom[3], t);

        // Premultiply alpha for tiny-skia
        let pa = a as f64 / 255.0;
        let pr = (r as f64 * pa) as u8;
        let pg = (g as f64 * pa) as u8;
        let pb = (b as f64 * pa) as u8;

        for x in 0..w {
            let idx = (y * w + x) * 4;
            data[idx] = pr;
            data[idx + 1] = pg;
            data[idx + 2] = pb;
            data[idx + 3] = a;
        }
    }
}

/// Radial gradient from center to edges.
fn draw_radial_gradient(pixmap: &mut Pixmap, w: usize, h: usize, center: &[u8; 4], edge: &[u8; 4]) {
    let data = pixmap.data_mut();
    let cx = w as f64 / 2.0;
    let cy = h as f64 / 2.0;
    let max_dist = (cx * cx + cy * cy).sqrt();

    for y in 0..h {
        for x in 0..w {
            let dx = x as f64 - cx;
            let dy = y as f64 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            let t = (dist / max_dist).min(1.0);

            let r = lerp_u8(center[0], edge[0], t);
            let g = lerp_u8(center[1], edge[1], t);
            let b = lerp_u8(center[2], edge[2], t);
            let a = lerp_u8(center[3], edge[3], t);

            let pa = a as f64 / 255.0;
            let pr = (r as f64 * pa) as u8;
            let pg = (g as f64 * pa) as u8;
            let pb = (b as f64 * pa) as u8;

            let idx = (y * w + x) * 4;
            data[idx] = pr;
            data[idx + 1] = pg;
            data[idx + 2] = pb;
            data[idx + 3] = a;
        }
    }
}

/// Perlin noise texture on a base color — creates a subtle, organic feel.
fn draw_noise_background(pixmap: &mut Pixmap, w: usize, h: usize, base: &[u8; 4], _frame_index: usize) {
    let perlin = Perlin::new(42); // consistent seed
    let data = pixmap.data_mut();
    let scale = 0.005; // noise scale (larger = smoother pattern)

    for y in 0..h {
        for x in 0..w {
            let noise_val = perlin.get([x as f64 * scale, y as f64 * scale]);
            // Map noise from [-1, 1] to a subtle brightness variation
            let variation = (noise_val * 20.0) as i16; // ±20 brightness

            let r = (base[0] as i16 + variation).max(0).min(255) as u8;
            let g = (base[1] as i16 + variation).max(0).min(255) as u8;
            let b = (base[2] as i16 + variation).max(0).min(255) as u8;
            let a = base[3];

            let pa = a as f64 / 255.0;
            let pr = (r as f64 * pa) as u8;
            let pg = (g as f64 * pa) as u8;
            let pb = (b as f64 * pa) as u8;

            let idx = (y * w + x) * 4;
            data[idx] = pr;
            data[idx + 1] = pg;
            data[idx + 2] = pb;
            data[idx + 3] = a;
        }
    }
}

fn lerp_u8(a: u8, b: u8, t: f64) -> u8 {
    (a as f64 + (b as f64 - a as f64) * t).max(0.0).min(255.0) as u8
}
