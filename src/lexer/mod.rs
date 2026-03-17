mod token;

pub use token::*;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        self.skip_whitespace_no_newline();

        while self.pos < self.input.len() {
            let ch = self.input[self.pos];

            match ch {
                '\n' => {
                    tokens.push(Token::new(TokenKind::Newline, self.line, self.col));
                    self.advance();
                    // After newline, measure indentation
                    let indent = self.count_indent();
                    if indent > 0 {
                        tokens.push(Token::with_value(
                            TokenKind::Indent,
                            indent.to_string(),
                            self.line,
                            self.col,
                        ));
                    }
                }
                '#' => {
                    // Comment — skip until newline
                    while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                        self.advance();
                    }
                }
                '"' => {
                    let s = self.read_string()?;
                    tokens.push(Token::with_value(TokenKind::StringLit, s, self.line, self.col));
                }
                '(' => {
                    tokens.push(Token::new(TokenKind::LParen, self.line, self.col));
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::new(TokenKind::RParen, self.line, self.col));
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::new(TokenKind::Comma, self.line, self.col));
                    self.advance();
                }
                ':' => {
                    tokens.push(Token::new(TokenKind::Colon, self.line, self.col));
                    self.advance();
                }
                _ if ch.is_ascii_digit() || (ch == '-' && self.peek_next().map_or(false, |c| c.is_ascii_digit())) => {
                    let num = self.read_number();
                    // Check for resolution like 1920x1080
                    if self.pos < self.input.len() && self.input[self.pos] == 'x' {
                        let col_start = self.col;
                        self.advance(); // skip 'x'
                        let num2 = self.read_number();
                        tokens.push(Token::with_value(
                            TokenKind::Resolution,
                            format!("{}x{}", num, num2),
                            self.line,
                            col_start,
                        ));
                    } else {
                        tokens.push(Token::with_value(TokenKind::NumberLit, num, self.line, self.col));
                    }
                }
                _ if ch.is_alphabetic() || ch == '_' => {
                    let word = self.read_word();
                    let kind = Self::classify_word(&word);
                    tokens.push(Token::with_value(kind, word, self.line, self.col));
                }
                ' ' | '\t' | '\r' => {
                    self.skip_whitespace_no_newline();
                }
                _ => {
                    return Err(LexError {
                        message: format!("Unexpected character '{}'", ch),
                        line: self.line,
                        col: self.col,
                    });
                }
            }
        }

        tokens.push(Token::new(TokenKind::Eof, self.line, self.col));
        Ok(tokens)
    }

    fn advance(&mut self) {
        if self.pos < self.input.len() {
            if self.input[self.pos] == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            self.pos += 1;
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.pos + 1 < self.input.len() {
            Some(self.input[self.pos + 1])
        } else {
            None
        }
    }

    fn skip_whitespace_no_newline(&mut self) {
        while self.pos < self.input.len() && (self.input[self.pos] == ' ' || self.input[self.pos] == '\t' || self.input[self.pos] == '\r') {
            self.advance();
        }
    }

    fn count_indent(&mut self) -> usize {
        let mut spaces = 0;
        while self.pos < self.input.len() && self.input[self.pos] == ' ' {
            spaces += 1;
            self.advance();
        }
        // Also handle tabs (1 tab = 4 spaces)
        while self.pos < self.input.len() && self.input[self.pos] == '\t' {
            spaces += 4;
            self.advance();
        }
        spaces
    }

    fn read_string(&mut self) -> Result<String, LexError> {
        self.advance(); // skip opening "
        let mut s = String::new();
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            if self.input[self.pos] == '\\' {
                self.advance();
                if self.pos < self.input.len() {
                    match self.input[self.pos] {
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
                        '"' => s.push('"'),
                        '\\' => s.push('\\'),
                        c => {
                            s.push('\\');
                            s.push(c);
                        }
                    }
                }
            } else {
                s.push(self.input[self.pos]);
            }
            self.advance();
        }
        if self.pos >= self.input.len() {
            return Err(LexError {
                message: "Unterminated string".to_string(),
                line: self.line,
                col: self.col,
            });
        }
        self.advance(); // skip closing "
        Ok(s)
    }

    fn read_number(&mut self) -> String {
        let mut num = String::new();
        if self.pos < self.input.len() && self.input[self.pos] == '-' {
            num.push('-');
            self.advance();
        }
        while self.pos < self.input.len() && (self.input[self.pos].is_ascii_digit() || self.input[self.pos] == '.') {
            num.push(self.input[self.pos]);
            self.advance();
        }
        num
    }

    fn read_word(&mut self) -> String {
        let mut word = String::new();
        while self.pos < self.input.len() {
            let ch = self.input[self.pos];
            if ch.is_alphanumeric() || ch == '_' {
                word.push(ch);
                self.advance();
            } else if ch == '-' && self.pos + 1 < self.input.len() && self.input[self.pos + 1].is_alphabetic() {
                // Allow hyphens in compound words like "ease-out-expo", "ease-in-out-bounce"
                word.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        word
    }

    fn classify_word(word: &str) -> TokenKind {
        match word.to_lowercase().as_str() {
            // Commands
            "scene" => TokenKind::Scene,
            "set" => TokenKind::Set,
            "draw" => TokenKind::Draw,
            "fade" => TokenKind::Fade,
            "move" => TokenKind::Move,
            "rotate" => TokenKind::Rotate,
            "scale" => TokenKind::Scale,
            "change" => TokenKind::Change,
            "wait" => TokenKind::Wait,
            "next" => TokenKind::Next,
            "grow" => TokenKind::Grow,
            "write" => TokenKind::Write,
            "highlight" => TokenKind::Highlight,
            "group" => TokenKind::Group,
            "style" => TokenKind::Style,
            "plot" | "graph" => TokenKind::Plot,
            "shrink" => TokenKind::Shrink,
            "math" => TokenKind::Math,
            "indicate" => TokenKind::Indicate,
            "wiggle" => TokenKind::Wiggle,
            "circumscribe" => TokenKind::Circumscribe,
            "spiral" => TokenKind::Spiral,

            // Connectors
            "to" => TokenKind::To,
            "at" => TokenKind::At,
            "with" => TokenKind::With,
            "from" => TokenKind::From,
            "over" => TokenKind::Over,
            "by" => TokenKind::By,
            "of" => TokenKind::Of,
            "the" => TokenKind::The,
            "a" => TokenKind::A,
            "an" => TokenKind::A,
            "in" => TokenKind::In,
            "out" => TokenKind::Out,
            "is" => TokenKind::Is,
            "and" => TokenKind::And,

            // Shapes
            "circle" => TokenKind::Circle,
            "square" => TokenKind::Square,
            "rectangle" | "rect" => TokenKind::Rectangle,
            "triangle" => TokenKind::Triangle,
            "line" => TokenKind::Line,
            "arrow" => TokenKind::Arrow,
            "text" => TokenKind::Text,
            "wave" => TokenKind::Wave,
            "grid" => TokenKind::Grid,
            "axes" | "axis" => TokenKind::Axes,
            "curve" => TokenKind::Curve,

            // Properties
            "color" | "colour" => TokenKind::Color,
            "size" => TokenKind::Size,
            "radius" => TokenKind::Radius,
            "thickness" => TokenKind::Thickness,
            "background" => TokenKind::Background,
            "resolution" => TokenKind::ResolutionKw,
            "fps" => TokenKind::Fps,
            "font" => TokenKind::Font,
            "bold" => TokenKind::Bold,
            "amplitude" | "amp" => TokenKind::Amplitude,
            "frequency" | "freq" => TokenKind::Frequency,
            "easing" | "ease" => TokenKind::Easing,

            // Positions
            "center" | "centre" => TokenKind::Center,
            "top" => TokenKind::Top,
            "bottom" => TokenKind::Bottom,
            "left" => TokenKind::Left,
            "right" => TokenKind::Right,

            // Time
            "second" | "seconds" | "sec" | "secs" | "s" => TokenKind::Second,

            // Scale suffix
            "x" => TokenKind::XSuffix,

            // Special
            "everything" | "all" => TokenKind::Everything,

            // Named colors
            "red" | "green" | "blue" | "white" | "black" | "yellow"
            | "cyan" | "magenta" | "orange" | "purple" | "pink"
            | "dark" | "light" | "grey" | "gray"
            | "gradient" | "noise" | "radial" => TokenKind::NamedColor,

            _ => TokenKind::Identifier,
        }
    }
}

#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub col: usize,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lex error at {}:{}: {}", self.line, self.col, self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_scene() {
        let input = r#"scene "Hello World""#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Scene);
        assert_eq!(tokens[1].kind, TokenKind::StringLit);
        assert_eq!(tokens[1].value.as_deref(), Some("Hello World"));
    }

    #[test]
    fn test_draw_shape() {
        let input = "draw a circle at center color red";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Draw);
        assert_eq!(tokens[1].kind, TokenKind::A);
        assert_eq!(tokens[2].kind, TokenKind::Circle);
        assert_eq!(tokens[3].kind, TokenKind::At);
        assert_eq!(tokens[4].kind, TokenKind::Center);
        assert_eq!(tokens[5].kind, TokenKind::Color);
        assert_eq!(tokens[6].kind, TokenKind::NamedColor);
    }

    #[test]
    fn test_animation() {
        let input = "fade in the circle over 1 second";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Fade);
        assert_eq!(tokens[1].kind, TokenKind::In);
        assert_eq!(tokens[2].kind, TokenKind::The);
        assert_eq!(tokens[3].kind, TokenKind::Circle);
        assert_eq!(tokens[4].kind, TokenKind::Over);
        assert_eq!(tokens[5].kind, TokenKind::NumberLit);
        assert_eq!(tokens[6].kind, TokenKind::Second);
    }

    #[test]
    fn test_coordinates() {
        let input = "draw a circle at (100, 200)";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[3].kind, TokenKind::At);
        assert_eq!(tokens[4].kind, TokenKind::LParen);
        assert_eq!(tokens[5].kind, TokenKind::NumberLit);
        assert_eq!(tokens[6].kind, TokenKind::Comma);
        assert_eq!(tokens[7].kind, TokenKind::NumberLit);
        assert_eq!(tokens[8].kind, TokenKind::RParen);
    }

    #[test]
    fn test_resolution() {
        let input = "set resolution to 1920x1080";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Set);
        assert_eq!(tokens[1].kind, TokenKind::ResolutionKw);
        assert_eq!(tokens[2].kind, TokenKind::To);
        assert_eq!(tokens[3].kind, TokenKind::Resolution);
        assert_eq!(tokens[3].value.as_deref(), Some("1920x1080"));
    }

    #[test]
    fn test_hex_color_as_string() {
        let input = r##"color "#ff0000""##;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Color);
        assert_eq!(tokens[1].kind, TokenKind::StringLit);
        assert_eq!(tokens[1].value.as_deref(), Some("#ff0000"));
    }
}
