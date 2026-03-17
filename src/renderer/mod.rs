pub mod scene;
pub mod shapes;
pub mod animation;
pub mod text;
pub mod export;
pub mod plotting;
pub mod backgrounds;

use crate::parser::ast::*;
use scene::SceneState;
use text::TextRenderer;

/// The main renderer. Takes a parsed Program and renders it to frames.
pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub background: backgrounds::BackgroundKind,
    text_renderer: TextRenderer,
    frame_index: usize,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fps: 60,
            background: backgrounds::BackgroundKind::Solid(
                ColorValue::Named("dark blue".to_string()).to_rgba()
            ),
            text_renderer: TextRenderer::new(),
            frame_index: 0,
        }
    }

    /// Render a full program to a list of RGBA frame buffers.
    pub fn render_program(&mut self, program: &Program) -> Vec<Vec<u8>> {
        let mut all_frames = Vec::new();

        for scene in &program.scenes {
            let frames = self.render_scene(scene);
            all_frames.extend(frames);
        }

        all_frames
    }

    fn render_scene(&mut self, scene: &Scene) -> Vec<Vec<u8>> {
        let mut state = SceneState::new(self.width as f64, self.height as f64);
        let mut frames: Vec<Vec<u8>> = Vec::new();

        // First pass: apply settings
        for stmt in &scene.body {
            if let Statement::Set { property, value } = stmt {
                self.apply_setting(property, value);
            }
            if let Statement::StyleDef { name, properties } = stmt {
                state.define_style(name.clone(), properties.clone());
            }
        }

        // Second pass: process draw/animate/wait
        for stmt in &scene.body {
            match stmt {
                Statement::Set { .. } | Statement::StyleDef { .. } => {
                    // Already handled
                }
                Statement::Draw { name, shape, position, properties, end_position } => {
                    state.add_shape(name.clone(), shape.clone(), position.clone(), properties.clone(), end_position.clone());
                    frames.push(self.render_frame(&state));
                }
                Statement::Animate { kind, target, duration, easing } => {
                    let anim_frames = self.render_animation(&mut state, kind, target, *duration, easing);
                    frames.extend(anim_frames);
                }
                Statement::Wait { duration } => {
                    let frame_count = (*duration * self.fps as f64) as usize;
                    let frame = self.render_frame(&state);
                    for _ in 0..frame_count {
                        frames.push(frame.clone());
                    }
                }
                Statement::Write { content, position, color, size, duration } => {
                    let anim_frames = self.render_write(&mut state, content, position, color, *size, *duration);
                    frames.extend(anim_frames);
                }
                Statement::Group { name, body } => {
                    for inner in body {
                        if let Statement::Draw { name: n, shape, position, properties, end_position } = inner {
                            state.add_shape(n.clone(), shape.clone(), position.clone(), properties.clone(), end_position.clone());
                        }
                    }
                    let _ = name;
                    frames.push(self.render_frame(&state));
                }
                Statement::Plot { expr, x_range, y_range, color, thickness, duration } => {
                    let plot_frames = self.render_plot(
                        &mut state, expr, *x_range, *y_range, color, *thickness, *duration,
                    );
                    frames.extend(plot_frames);
                }
            }
        }

        if frames.is_empty() {
            frames.push(self.render_frame(&state));
        }

        frames
    }

    fn apply_setting(&mut self, property: &str, value: &Value) {
        match property {
            "background" => {
                if let Value::String(color_str) = value {
                    let lower = color_str.to_lowercase();

                    if lower.starts_with("gradient ") {
                        // "gradient blue purple"
                        let parts: Vec<&str> = lower.splitn(3, ' ').collect();
                        if parts.len() >= 3 {
                            let c1 = ColorValue::Named(parts[1].to_string()).to_rgba();
                            let c2 = ColorValue::Named(parts[2].to_string()).to_rgba();
                            self.background = backgrounds::BackgroundKind::Gradient(c1, c2);
                        }
                    } else if lower.starts_with("radial ") {
                        // "radial dark blue black"
                        let rest = &color_str[7..];
                        let parts: Vec<&str> = rest.splitn(2, ' ').collect();
                        let c1 = ColorValue::Named(parts.get(0).unwrap_or(&"dark blue").to_string()).to_rgba();
                        let c2 = ColorValue::Named(parts.get(1).unwrap_or(&"black").to_string()).to_rgba();
                        self.background = backgrounds::BackgroundKind::Radial(c1, c2);
                    } else if lower.starts_with("noise ") {
                        // "noise dark blue"
                        let rest = &color_str[6..];
                        let base = ColorValue::Named(rest.to_string()).to_rgba();
                        self.background = backgrounds::BackgroundKind::Noise(base);
                    } else {
                        let color = ColorValue::Named(color_str.clone());
                        self.background = backgrounds::BackgroundKind::Solid(color.to_rgba());
                    }
                }
            }
            "resolution" => {
                if let Value::String(res) = value {
                    if let Some((w, h)) = res.split_once('x') {
                        self.width = w.parse().unwrap_or(1920);
                        self.height = h.parse().unwrap_or(1080);
                    }
                }
            }
            "fps" => {
                if let Value::Number(n) = value {
                    self.fps = *n as u32;
                }
            }
            _ => {}
        }
    }

    fn render_frame(&self, state: &SceneState) -> Vec<u8> {
        let w = self.width;
        let h = self.height;
        let mut pixmap = tiny_skia::Pixmap::new(w, h).unwrap();

        // Draw background (solid, gradient, noise, or radial)
        backgrounds::draw_background(&mut pixmap, &self.background, self.frame_index);

        for obj in &state.objects {
            shapes::draw_object(&mut pixmap, obj, w as f64, h as f64, &self.text_renderer);
        }

        pixmap.data().to_vec()
    }

    fn render_animation(
        &self,
        state: &mut SceneState,
        kind: &AnimKind,
        target: &Target,
        duration: f64,
        easing_name: &str,
    ) -> Vec<Vec<u8>> {
        let frame_count = (duration * self.fps as f64) as usize;
        let frame_count = frame_count.max(1);
        let mut frames = Vec::with_capacity(frame_count);

        let target_indices = state.resolve_target(target);
        let initial_states: Vec<_> = target_indices
            .iter()
            .map(|&idx| state.objects[idx].clone())
            .collect();

        for frame_i in 0..frame_count {
            let raw_t = (frame_i as f64 + 1.0) / frame_count as f64;
            let t = animation::apply_easing(raw_t, easing_name);

            for (i, &idx) in target_indices.iter().enumerate() {
                let initial = &initial_states[i];
                animation::apply_animation(&mut state.objects[idx], initial, kind, t, self.width as f64, self.height as f64);

                // For wave animations, animate the wave_progress
                if matches!(kind, AnimKind::WaveAnimate) {
                    state.objects[idx].wave_progress = raw_t;
                }
            }

            frames.push(self.render_frame(state));
        }

        frames
    }

    fn render_write(
        &self,
        state: &mut SceneState,
        content: &str,
        position: &Position,
        color: &ColorValue,
        size: f64,
        duration: f64,
    ) -> Vec<Vec<u8>> {
        let frame_count = (duration * self.fps as f64) as usize;
        let frame_count = frame_count.max(1);
        let mut frames = Vec::with_capacity(frame_count);

        let (px, py) = position.resolve(self.width as f64, self.height as f64);

        state.add_shape(
            None,
            ShapeKind::Text(content.to_string()),
            Position::Coords(px, py),
            vec![
                ShapeProp::Color(color.clone()),
                ShapeProp::Size(size),
            ],
            None,
        );
        let text_idx = state.objects.len() - 1;
        state.objects[text_idx].opacity = 0.0;

        let total_chars = content.len();
        for frame_i in 0..frame_count {
            let t = (frame_i as f64 + 1.0) / frame_count as f64;
            // Use smooth easing for character reveal
            let eased = animation::ease_in_out(t);
            let chars_to_show = ((eased * total_chars as f64).ceil() as usize).min(total_chars);

            state.objects[text_idx].visible_chars = Some(chars_to_show);
            state.objects[text_idx].opacity = 1.0;

            frames.push(self.render_frame(state));
        }

        state.objects[text_idx].visible_chars = None;
        state.objects[text_idx].opacity = 1.0;

        frames
    }

    fn render_plot(
        &self,
        state: &mut SceneState,
        expr: &str,
        x_range: (f64, f64),
        y_range: (f64, f64),
        color: &ColorValue,
        thickness: f64,
        duration: f64,
    ) -> Vec<Vec<u8>> {
        let frame_count = (duration * self.fps as f64) as usize;
        let frame_count = frame_count.max(1);
        let mut frames = Vec::with_capacity(frame_count);

        // Add the function plot shape
        state.add_shape(
            None,
            ShapeKind::FunctionPlot {
                expr: expr.to_string(),
                x_range,
                y_range,
            },
            Position::Center,
            vec![
                ShapeProp::Color(color.clone()),
                ShapeProp::Size(600.0),
                ShapeProp::Thickness(thickness),
            ],
            None,
        );
        let plot_idx = state.objects.len() - 1;
        state.objects[plot_idx].wave_progress = 0.0;

        // Animate the curve drawing
        for frame_i in 0..frame_count {
            let t = (frame_i as f64 + 1.0) / frame_count as f64;
            let eased = animation::ease_in_out(t);
            state.objects[plot_idx].wave_progress = eased;
            frames.push(self.render_frame(state));
        }

        state.objects[plot_idx].wave_progress = 1.0;

        frames
    }
}
