/// AST node definitions for the Slang language.

#[derive(Debug)]
pub struct Program {
    pub scenes: Vec<Scene>,
}

#[derive(Debug)]
pub struct Scene {
    pub title: String,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Set {
        property: String,
        value: Value,
    },
    Draw {
        name: Option<String>,
        shape: ShapeKind,
        position: Position,
        properties: Vec<ShapeProp>,
        end_position: Option<Position>,
    },
    Animate {
        kind: AnimKind,
        target: Target,
        duration: f64,
        easing: String,
    },
    Wait {
        duration: f64,
    },
    Write {
        content: String,
        position: Position,
        color: ColorValue,
        size: f64,
        duration: f64,
    },
    Group {
        name: String,
        body: Vec<Statement>,
    },
    StyleDef {
        name: String,
        properties: Vec<StyleProp>,
    },
    /// Plot a mathematical function
    Plot {
        expr: String,
        x_range: (f64, f64),
        y_range: (f64, f64),
        color: ColorValue,
        thickness: f64,
        duration: f64,
    },
    /// Emit particles
    Emit {
        position: Position,
        color: ColorValue,
        count: usize,
        duration: f64,
    },
}

#[derive(Debug, Clone)]
pub enum ShapeKind {
    Circle,
    Square,
    Rectangle,
    Triangle,
    Line,
    Arrow,
    Text(String),
    Math(String),
    /// Wave: amplitude, frequency, phase
    Wave { amplitude: f64, frequency: f64 },
    /// Grid / Axes
    Grid,
    /// Bezier Curve: control points
    Curve { points: Vec<(f64, f64)> },
    /// Function plot (expression stored as string)
    FunctionPlot { expr: String, x_range: (f64, f64), y_range: (f64, f64) },
    /// Number axis / labeled axes
    NumberAxis { x_range: (f64, f64), y_range: (f64, f64) },
}

#[derive(Debug, Clone)]
pub enum Position {
    Center,
    Top,
    Bottom,
    Left,
    Right,
    Coords(f64, f64),
}

impl Position {
    /// Resolve to absolute pixel coordinates given canvas width/height.
    pub fn resolve(&self, width: f64, height: f64) -> (f64, f64) {
        match self {
            Position::Center => (width / 2.0, height / 2.0),
            Position::Top => (width / 2.0, height * 0.15),
            Position::Bottom => (width / 2.0, height * 0.85),
            Position::Left => (width * 0.2, height / 2.0),
            Position::Right => (width * 0.8, height / 2.0),
            Position::Coords(x, y) => (*x, *y),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ShapeProp {
    Color(ColorValue),
    Size(f64),
    Radius(f64),
    Thickness(f64),
    Amplitude(f64),
    Frequency(f64),
    StyleRef(String),
}

#[derive(Debug, Clone)]
pub enum AnimKind {
    FadeIn,
    FadeOut,
    MoveTo(Position),
    Rotate(f64),
    Scale(f64),
    ChangeColor(ColorValue),
    Grow(f64),
    Shrink(f64),
    Highlight(ColorValue),
    /// Animate a wave drawing progressively
    WaveAnimate,
    // --- Manim-inspired animations ---
    /// GrowFromCenter — scale from 0 to 1 + fade in
    GrowIn,
    /// SpinInFromNothing — spin + grow from center
    SpinIn,
    /// DrawBorderThenFill — outline draws, then fill fades
    DrawIn,
    /// Indicate — brief yellow pulse + scale bump
    Indicate,
    /// Flash — expanding ring burst
    Flash(ColorValue),
    /// Wiggle — rapid side-to-side shake
    Wiggle,
    /// Circumscribe — draw highlight outline around shape
    Circumscribe(ColorValue),
    /// SpiralIn — spiral from off-screen to position
    SpiralIn,
    // --- New animations ---
    /// Morph one shape into another
    MorphInto(ShapeKind),
    /// Zoom camera to a scale factor
    ZoomTo(f64),
    /// Pan camera to a position
    PanTo(Position),
}

#[derive(Debug, Clone)]
pub enum Target {
    Everything,
    LastShape(ShapeKind),
    Named(String),
}

#[derive(Debug, Clone)]
pub enum ColorValue {
    Named(String),
    Hex(String),
    /// Gradient between two colors
    Gradient(Box<ColorValue>, Box<ColorValue>),
}

impl ColorValue {
    /// Convert to RGBA (0-255).
    pub fn to_rgba(&self) -> [u8; 4] {
        match self {
            ColorValue::Named(name) => match name.to_lowercase().as_str() {
                "red" => [231, 76, 60, 255],
                "green" => [46, 204, 113, 255],
                "blue" => [52, 152, 219, 255],
                "white" => [255, 255, 255, 255],
                "black" => [0, 0, 0, 255],
                "yellow" => [241, 196, 15, 255],
                "cyan" => [0, 255, 255, 255],
                "magenta" => [255, 0, 255, 255],
                "orange" => [243, 156, 18, 255],
                "purple" => [155, 89, 182, 255],
                "pink" => [255, 105, 180, 255],
                "grey" | "gray" => [149, 165, 166, 255],
                "dark blue" => [26, 26, 46, 255],
                "dark green" => [0, 100, 0, 255],
                "dark red" => [139, 0, 0, 255],
                "light blue" => [173, 216, 230, 255],
                "light green" => [144, 238, 144, 255],
                "light grey" | "light gray" => [211, 211, 211, 255],
                "dark grey" | "dark gray" => [64, 64, 64, 255],
                _ => [200, 200, 200, 255], // fallback
            },
            ColorValue::Hex(hex) => {
                let hex = hex.trim_start_matches('#');
                if hex.len() == 6 {
                    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(200);
                    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(200);
                    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(200);
                    [r, g, b, 255]
                } else {
                    [200, 200, 200, 255]
                }
            }
            ColorValue::Gradient(a, _b) => {
                // For flat contexts, return the first gradient color
                a.to_rgba()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct StyleProp {
    pub name: String,
    pub value: Value,
}
