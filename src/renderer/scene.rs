use crate::parser::ast::*;
use std::collections::HashMap;

/// A drawable object in the scene.
#[derive(Debug, Clone)]
pub struct SceneObject {
    pub id: usize,
    pub name: Option<String>,
    pub shape: ShapeKind,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub size: f64,
    pub color: [u8; 4],
    pub opacity: f64,
    pub rotation: f64,
    pub scale: f64,
    pub thickness: f64,
    pub end_x: Option<f64>,
    pub end_y: Option<f64>,
    pub visible_chars: Option<usize>,
    /// Wave-specific fields
    pub amplitude: f64,
    pub frequency: f64,
    pub wave_progress: f64,
}

impl SceneObject {
    pub fn effective_color(&self) -> [u8; 4] {
        let [r, g, b, _a] = self.color;
        let alpha = (self.opacity * 255.0).min(255.0).max(0.0) as u8;
        [r, g, b, alpha]
    }
}

pub struct SceneState {
    pub objects: Vec<SceneObject>,
    next_id: usize,
    canvas_w: f64,
    canvas_h: f64,
    styles: HashMap<String, Vec<StyleProp>>,
}

impl SceneState {
    pub fn new(w: f64, h: f64) -> Self {
        Self {
            objects: Vec::new(),
            next_id: 0,
            canvas_w: w,
            canvas_h: h,
            styles: HashMap::new(),
        }
    }

    pub fn define_style(&mut self, name: String, props: Vec<StyleProp>) {
        self.styles.insert(name, props);
    }

    pub fn add_shape(
        &mut self,
        name: Option<String>,
        mut shape: ShapeKind,
        position: Position,
        properties: Vec<ShapeProp>,
        end_position: Option<Position>,
    ) {
        let (x, y) = position.resolve(self.canvas_w, self.canvas_h);
        let (end_x, end_y) = if let Some(ref ep) = end_position {
            let (ex, ey) = ep.resolve(self.canvas_w, self.canvas_h);
            (Some(ex), Some(ey))
        } else {
            (None, None)
        };

        let mut color = [255, 255, 255, 255];
        let mut size = 100.0;
        let mut radius = 50.0;
        let mut thickness = 2.0;
        let mut amplitude = 50.0;
        let mut frequency = 2.0;

        for prop in &properties {
            match prop {
                ShapeProp::Color(c) => color = c.to_rgba(),
                ShapeProp::Size(s) => size = *s,
                ShapeProp::Radius(r) => radius = *r,
                ShapeProp::Thickness(t) => thickness = *t,
                ShapeProp::Amplitude(a) => amplitude = *a,
                ShapeProp::Frequency(f) => frequency = *f,
                ShapeProp::StyleRef(style_name) => {
                    if let Some(style_props) = self.styles.get(style_name).cloned() {
                        for sp in &style_props {
                            match sp.name.as_str() {
                                "color" => {
                                    if let Value::String(c) = &sp.value {
                                        color = ColorValue::Named(c.clone()).to_rgba();
                                    }
                                }
                                "size" => {
                                    if let Value::Number(n) = &sp.value {
                                        size = *n;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        // Update wave shape with parsed amplitude/frequency
        if let ShapeKind::Wave { amplitude: ref mut a, frequency: ref mut f } = shape {
            *a = amplitude;
            *f = frequency;
        }

        let obj = SceneObject {
            id: self.next_id,
            name,
            shape,
            x,
            y,
            radius,
            size,
            color,
            opacity: 1.0,
            rotation: 0.0,
            scale: 1.0,
            thickness,
            end_x,
            end_y,
            visible_chars: None,
            amplitude,
            frequency,
            wave_progress: 1.0,
        };

        self.next_id += 1;
        self.objects.push(obj);
    }

    pub fn resolve_target(&self, target: &Target) -> Vec<usize> {
        match target {
            Target::Everything => (0..self.objects.len()).collect(),
            Target::Named(name) => {
                self.objects
                    .iter()
                    .enumerate()
                    .filter(|(_, o)| o.name.as_deref() == Some(name.as_str()))
                    .map(|(i, _)| i)
                    .collect()
            }
            Target::LastShape(kind) => {
                let shape_name = match kind {
                    ShapeKind::Circle => "circle",
                    ShapeKind::Square => "square",
                    ShapeKind::Rectangle => "rectangle",
                    ShapeKind::Triangle => "triangle",
                    ShapeKind::Line => "line",
                    ShapeKind::Arrow => "arrow",
                    ShapeKind::Text(_) => "text",
                    ShapeKind::Math(_) => "math",
                    ShapeKind::Wave { .. } => "wave",
                    ShapeKind::Grid => "grid",
                    ShapeKind::Curve { .. } => "curve",
                    ShapeKind::FunctionPlot { .. } => "plot",
                    ShapeKind::NumberAxis { .. } => "axes",
                };
                self.objects
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, o)| {
                        let obj_shape = match &o.shape {
                            ShapeKind::Circle => "circle",
                            ShapeKind::Square => "square",
                            ShapeKind::Rectangle => "rectangle",
                            ShapeKind::Triangle => "triangle",
                            ShapeKind::Line => "line",
                            ShapeKind::Arrow => "arrow",
                            ShapeKind::Text(_) => "text",
                            ShapeKind::Math(_) => "math",
                            ShapeKind::Wave { .. } => "wave",
                            ShapeKind::Grid => "grid",
                            ShapeKind::Curve { .. } => "curve",
                            ShapeKind::FunctionPlot { .. } => "plot",
                            ShapeKind::NumberAxis { .. } => "axes",
                        };
                        obj_shape == shape_name
                    })
                    .map(|(i, _)| vec![i])
                    .unwrap_or_default()
            }
        }
    }
}
