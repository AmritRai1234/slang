use crate::parser::ast::*;
use crate::renderer::scene::SceneObject;

// ===== Easing Functions =====
// Professional easing powered by `keyframe` crate where available,
// plus custom implementations for Back, Elastic, Bounce, Expo, Circ, Sine.

/// Apply easing based on the name — 30+ easing options.
pub fn apply_easing(t: f64, easing: &str) -> f64 {
    let t = t.max(0.0).min(1.0);

    match easing {
        // --- Linear ---
        "linear" => t,

        // --- Sine (custom) ---
        "ease-in-sine" | "sine-in" => 1.0 - ((t * std::f64::consts::FRAC_PI_2).cos()),
        "ease-out-sine" | "sine-out" => (t * std::f64::consts::FRAC_PI_2).sin(),
        "ease-in-out-sine" | "sine" => -(((std::f64::consts::PI * t).cos() - 1.0) / 2.0),

        // --- Quad (keyframe crate) ---
        "ease-in-quad" | "quad-in" => keyframe::ease(keyframe::functions::EaseInQuad, 0.0, 1.0, t),
        "ease-out-quad" | "quad-out" => keyframe::ease(keyframe::functions::EaseOutQuad, 0.0, 1.0, t),
        "ease-in-out-quad" | "quad" => keyframe::ease(keyframe::functions::EaseInOutQuad, 0.0, 1.0, t),

        // --- Cubic (keyframe crate) ---
        "ease-in-cubic" | "cubic-in" | "ease-in" | "easein" => {
            keyframe::ease(keyframe::functions::EaseInCubic, 0.0, 1.0, t)
        }
        "ease-out-cubic" | "cubic-out" | "ease-out" | "easeout" => {
            keyframe::ease(keyframe::functions::EaseOutCubic, 0.0, 1.0, t)
        }
        "ease-in-out-cubic" | "cubic" | "smooth" | "ease-in-out" | "easeinout" => {
            keyframe::ease(keyframe::functions::EaseInOutCubic, 0.0, 1.0, t)
        }

        // --- Quart (keyframe crate) ---
        "ease-in-quart" | "quart-in" => keyframe::ease(keyframe::functions::EaseInQuart, 0.0, 1.0, t),
        "ease-out-quart" | "quart-out" => keyframe::ease(keyframe::functions::EaseOutQuart, 0.0, 1.0, t),
        "ease-in-out-quart" | "quart" => keyframe::ease(keyframe::functions::EaseInOutQuart, 0.0, 1.0, t),

        // --- Quint (keyframe crate) ---
        "ease-in-quint" | "quint-in" => keyframe::ease(keyframe::functions::EaseInQuint, 0.0, 1.0, t),
        "ease-out-quint" | "quint-out" => keyframe::ease(keyframe::functions::EaseOutQuint, 0.0, 1.0, t),
        "ease-in-out-quint" | "quint" => keyframe::ease(keyframe::functions::EaseInOutQuint, 0.0, 1.0, t),

        // --- Expo (custom) ---
        "ease-in-expo" | "expo-in" => {
            if t == 0.0 { 0.0 } else { 2.0_f64.powf(10.0 * t - 10.0) }
        }
        "ease-out-expo" | "expo-out" => {
            if t == 1.0 { 1.0 } else { 1.0 - 2.0_f64.powf(-10.0 * t) }
        }
        "ease-in-out-expo" | "expo" => {
            if t == 0.0 { 0.0 }
            else if t == 1.0 { 1.0 }
            else if t < 0.5 { 2.0_f64.powf(20.0 * t - 10.0) / 2.0 }
            else { (2.0 - 2.0_f64.powf(-20.0 * t + 10.0)) / 2.0 }
        }

        // --- Circ (custom) ---
        "ease-in-circ" | "circ-in" => 1.0 - (1.0 - t * t).sqrt(),
        "ease-out-circ" | "circ-out" => ((1.0 - (t - 1.0).powi(2)) as f64).sqrt(),
        "ease-in-out-circ" | "circ" => {
            if t < 0.5 {
                (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
            } else {
                ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
            }
        }

        // --- Back (custom) ---
        "ease-in-back" | "back-in" => {
            let c1 = 1.70158;
            let c3 = c1 + 1.0;
            c3 * t * t * t - c1 * t * t
        }
        "ease-out-back" | "back-out" | "back" => {
            let c1 = 1.70158;
            let c3 = c1 + 1.0;
            1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
        }
        "ease-in-out-back" => {
            let c1 = 1.70158;
            let c2 = c1 * 1.525;
            if t < 0.5 {
                ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
            } else {
                ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
            }
        }

        // --- Elastic (custom) ---
        "ease-in-elastic" | "elastic-in" => {
            if t == 0.0 { return 0.0; }
            if t == 1.0 { return 1.0; }
            let c4 = (2.0 * std::f64::consts::PI) / 3.0;
            -(2.0_f64.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * c4).sin())
        }
        "ease-out-elastic" | "elastic-out" | "elastic" => {
            if t == 0.0 { return 0.0; }
            if t == 1.0 { return 1.0; }
            let c4 = (2.0 * std::f64::consts::PI) / 3.0;
            2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
        }
        "ease-in-out-elastic" => {
            if t == 0.0 { return 0.0; }
            if t == 1.0 { return 1.0; }
            let c5 = (2.0 * std::f64::consts::PI) / 4.5;
            if t < 0.5 {
                -(2.0_f64.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0
            } else {
                (2.0_f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0 + 1.0
            }
        }

        // --- Bounce (custom) ---
        "ease-out-bounce" | "bounce-out" | "bounce" => ease_out_bounce(t),
        "ease-in-bounce" | "bounce-in" => 1.0 - ease_out_bounce(1.0 - t),
        "ease-in-out-bounce" => {
            if t < 0.5 {
                (1.0 - ease_out_bounce(1.0 - 2.0 * t)) / 2.0
            } else {
                (1.0 + ease_out_bounce(2.0 * t - 1.0)) / 2.0
            }
        }

        // --- Spring (custom) ---
        "spring" => {
            let factor = 0.4;
            let pow = 2.0_f64.powf(-10.0 * t);
            pow * ((t - factor / 4.0) * (2.0 * std::f64::consts::PI) / factor).sin() + 1.0
        }

        // Default: smooth ease-in-out via keyframe
        _ => keyframe::ease(keyframe::functions::EaseInOutCubic, 0.0, 1.0, t),
    }
}

/// Bounce out helper.
fn ease_out_bounce(t: f64) -> f64 {
    let n1 = 7.5625;
    let d1 = 2.75;
    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        let t = t - 1.5 / d1;
        n1 * t * t + 0.75
    } else if t < 2.5 / d1 {
        let t = t - 2.25 / d1;
        n1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / d1;
        n1 * t * t + 0.984375
    }
}

/// Convenience: smooth ease-in-out (used by renderer for defaults)
pub fn ease_in_out(t: f64) -> f64 {
    keyframe::ease(keyframe::functions::EaseInOutCubic, 0.0, 1.0, t.max(0.0).min(1.0))
}

// ===== Interpolation =====

/// Linear interpolation
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Interpolate colors
pub fn lerp_color(a: [u8; 4], b: [u8; 4], t: f64) -> [u8; 4] {
    [
        lerp(a[0] as f64, b[0] as f64, t) as u8,
        lerp(a[1] as f64, b[1] as f64, t) as u8,
        lerp(a[2] as f64, b[2] as f64, t) as u8,
        lerp(a[3] as f64, b[3] as f64, t) as u8,
    ]
}

// ===== Animation Application =====

/// Apply an animation to a SceneObject at progress `t` (0.0 to 1.0).
pub fn apply_animation(
    obj: &mut SceneObject,
    initial: &SceneObject,
    kind: &AnimKind,
    t: f64,
    canvas_w: f64,
    canvas_h: f64,
) {
    match kind {
        AnimKind::FadeIn => {
            obj.opacity = lerp(0.0, 1.0, t);
        }
        AnimKind::FadeOut => {
            obj.opacity = lerp(initial.opacity, 0.0, t);
        }
        AnimKind::MoveTo(position) => {
            let (tx, ty) = position.resolve(canvas_w, canvas_h);
            obj.x = lerp(initial.x, tx, t);
            obj.y = lerp(initial.y, ty, t);
        }
        AnimKind::Rotate(degrees) => {
            obj.rotation = lerp(initial.rotation, initial.rotation + degrees, t);
        }
        AnimKind::Scale(factor) => {
            obj.scale = lerp(initial.scale, *factor, t);
        }
        AnimKind::ChangeColor(color) => {
            let target = color.to_rgba();
            obj.color = lerp_color(initial.color, target, t);
        }
        AnimKind::Grow(target_radius) => {
            obj.radius = lerp(initial.radius, *target_radius, t);
        }
        AnimKind::Shrink(target_radius) => {
            obj.radius = lerp(initial.radius, *target_radius, t);
        }
        AnimKind::Highlight(color) => {
            let target = color.to_rgba();
            let t2 = if t < 0.5 { t * 2.0 } else { 2.0 - t * 2.0 };
            obj.color = lerp_color(initial.color, target, t2);
        }
        AnimKind::WaveAnimate => {
            obj.opacity = 1.0;
        }

        // --- Manim-inspired animations ---

        AnimKind::GrowIn => {
            // GrowFromCenter: scale from 0 to 1, opacity from 0 to 1
            obj.scale = lerp(0.0, initial.scale, t);
            obj.opacity = lerp(0.0, 1.0, t.min(0.5) * 2.0); // fade in during first half
        }
        AnimKind::SpinIn => {
            // SpinInFromNothing: grow from 0 + spin 720°
            obj.scale = lerp(0.0, initial.scale, t);
            obj.opacity = lerp(0.0, 1.0, t.min(0.3) * (1.0 / 0.3));
            obj.rotation = initial.rotation + (1.0 - t) * 720.0; // spin fast then settle
        }
        AnimKind::DrawIn => {
            // DrawBorderThenFill: first half shows outline (wave_progress), second half fills
            if t < 0.6 {
                // Drawing phase — outline appears progressively
                obj.opacity = 0.7;
                obj.scale = initial.scale;
                obj.wave_progress = t / 0.6; // reuse wave_progress for draw progress
            } else {
                // Fill phase — full opacity
                obj.opacity = lerp(0.7, 1.0, (t - 0.6) / 0.4);
                obj.wave_progress = 1.0;
            }
        }
        AnimKind::Indicate => {
            // Brief yellow highlight + scale pulse (1.0 → 1.2 → 1.0)
            let yellow = [241, 196, 15, 255];
            let pulse = if t < 0.5 {
                t * 2.0 // 0 → 1
            } else {
                2.0 - t * 2.0 // 1 → 0
            };
            obj.scale = initial.scale * (1.0 + 0.2 * pulse);
            obj.color = lerp_color(initial.color, yellow, pulse * 0.6);
        }
        AnimKind::Flash(color) => {
            // Expanding ring burst — scale up while fading out
            let target = color.to_rgba();
            obj.color = target;
            obj.scale = initial.scale * (1.0 + t * 2.0); // expand to 3x
            obj.opacity = lerp(1.0, 0.0, t); // fade out as it expands
        }
        AnimKind::Wiggle => {
            // Rapid side-to-side shake with damping
            let frequency = 6.0 * std::f64::consts::PI;
            let damping = 1.0 - t; // dampen towards the end
            let amplitude = 15.0 * damping;
            obj.x = initial.x + amplitude * (t * frequency).sin();
            // Slight rotation wiggle too
            obj.rotation = initial.rotation + 5.0 * damping * (t * frequency).sin();
        }
        AnimKind::Circumscribe(color) => {
            // Draw highlight outline around shape — scale pulse with color
            let target = color.to_rgba();
            let pulse = if t < 0.5 {
                t * 2.0
            } else {
                2.0 - t * 2.0
            };
            obj.scale = initial.scale * (1.0 + 0.15 * pulse);
            obj.color = lerp_color(initial.color, target, pulse * 0.8);
        }
        AnimKind::SpiralIn => {
            // Spiral from off-screen to position
            let angle = (1.0 - t) * 4.0 * std::f64::consts::PI; // 2 full spirals
            let radius = (1.0 - t) * 400.0; // start 400px away
            obj.x = initial.x + radius * angle.cos();
            obj.y = initial.y + radius * angle.sin();
            obj.rotation = initial.rotation + (1.0 - t) * 360.0;
            obj.opacity = lerp(0.0, 1.0, t.min(0.3) * (1.0 / 0.3)); // fade in early
            obj.scale = lerp(0.3, initial.scale, t);
        }

        // --- New animations ---

        AnimKind::MorphInto(target_shape) => {
            // Morph: interpolate between shapes
            // We store the morph progress and target shape on the object
            obj.morph_progress = t;
            obj.morph_target = Some(target_shape.clone());
            // Subtle scale pulse during morph
            let pulse = if t < 0.5 { t * 2.0 } else { 2.0 - t * 2.0 };
            obj.scale = initial.scale * (1.0 + 0.1 * pulse);
            // Slight rotation during morph
            obj.rotation = initial.rotation + 15.0 * pulse;
        }
        AnimKind::ZoomTo(_factor) => {
            // Camera zoom is handled at the scene level in the renderer
            // The animation progress is used to interpolate camera_zoom
        }
        AnimKind::PanTo(_position) => {
            // Camera pan is handled at the scene level in the renderer
            // The animation progress is used to interpolate camera_x, camera_y
        }
    }
}
