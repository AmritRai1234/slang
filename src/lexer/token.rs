#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Commands
    Scene,
    Set,
    Draw,
    Fade,
    Move,
    Rotate,
    Scale,
    Change,
    Wait,
    Next,
    Grow,
    Shrink,
    Write,
    Highlight,
    Group,
    Style,
    Math,
    Plot,
    Indicate,
    Wiggle,
    Circumscribe,
    Spiral,
    Morph,
    Into,
    Zoom,
    Pan,
    Emit,
    Particles,
    Count,
    Let,
    Repeat,
    Times,
    If,
    Else,

    // Connectors
    To,
    At,
    With,
    From,
    Over,
    By,
    Of,
    The,
    A,
    In,
    Out,
    Is,
    And,

    // Shapes
    Circle,
    Square,
    Rectangle,
    Triangle,
    Line,
    Arrow,
    Text,
    Wave,
    Grid,
    Curve,
    Axes,

    // Properties
    Color,
    Size,
    Radius,
    Thickness,
    Background,
    ResolutionKw,
    Fps,
    Font,
    Bold,
    Amplitude,
    Frequency,
    Easing,

    // Positions
    Center,
    Top,
    Bottom,
    Left,
    Right,

    // Literals
    StringLit,
    NumberLit,
    NamedColor,
    Resolution,

    // Time
    Second,

    // Scale suffix
    XSuffix,

    // Special
    Everything,
    Identifier,

    // Punctuation
    LParen,
    RParen,
    Comma,
    Colon,
    Newline,
    Indent,

    // Operators
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    NotEqual,
    LBrace,
    RBrace,

    // End
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, col: usize) -> Self {
        Self {
            kind,
            value: None,
            line,
            col,
        }
    }

    pub fn with_value(kind: TokenKind, value: String, line: usize, col: usize) -> Self {
        Self {
            kind,
            value: Some(value),
            line,
            col,
        }
    }
}
