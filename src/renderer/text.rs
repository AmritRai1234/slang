/// Proper text rendering using ab_glyph for real font support.
use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use tiny_skia::*;

const FONT_DATA: &[u8] = include_bytes!("../../assets/fonts/Inter.ttf");

pub struct TextRenderer {
    font: FontRef<'static>,
}

impl TextRenderer {
    pub fn new() -> Self {
        let font = FontRef::try_from_slice(FONT_DATA).expect("Failed to load embedded font");
        Self { font }
    }

    /// Render text onto a pixmap at the given position.
    pub fn draw_text(
        &self,
        pixmap: &mut Pixmap,
        x: f32,
        y: f32,
        text: &str,
        font_size: f32,
        color: Color,
    ) {
        let scale = PxScale::from(font_size);
        let scaled_font = self.font.as_scaled(scale);

        // Calculate total width for centering
        let total_width: f32 = text.chars().map(|c| {
            let glyph_id = self.font.glyph_id(c);
            scaled_font.h_advance(glyph_id)
        }).sum();

        let mut cursor_x = x - total_width / 2.0;
        let ascent = scaled_font.ascent();
        let baseline_y = y + ascent / 2.0 - font_size * 0.1;

        let [r, g, b, a] = [
            (color.red() * 255.0) as u8,
            (color.green() * 255.0) as u8,
            (color.blue() * 255.0) as u8,
            (color.alpha() * 255.0) as u8,
        ];

        for ch in text.chars() {
            let glyph_id = self.font.glyph_id(ch);
            let advance = scaled_font.h_advance(glyph_id);

            if let Some(outlined) = self.font.outline_glyph(
                ab_glyph::Glyph {
                    id: glyph_id,
                    scale,
                    position: ab_glyph::point(cursor_x, baseline_y),
                }
            ) {
                let bounds = outlined.px_bounds();
                outlined.draw(|gx, gy, coverage| {
                    let px = bounds.min.x as i32 + gx as i32;
                    let py = bounds.min.y as i32 + gy as i32;

                    if px >= 0 && py >= 0 && px < pixmap.width() as i32 && py < pixmap.height() as i32 {
                        let alpha = (coverage * a as f32) as u8;
                        if alpha > 0 {
                            let idx = (py as u32 * pixmap.width() + px as u32) as usize * 4;
                            let pixels = pixmap.data_mut();
                            if idx + 3 < pixels.len() {
                                // Alpha blending
                                let src_a = alpha as f32 / 255.0;
                                let dst_a = pixels[idx + 3] as f32 / 255.0;
                                let out_a = src_a + dst_a * (1.0 - src_a);

                                if out_a > 0.0 {
                                    pixels[idx] = ((r as f32 * src_a + pixels[idx] as f32 * dst_a * (1.0 - src_a)) / out_a) as u8;
                                    pixels[idx + 1] = ((g as f32 * src_a + pixels[idx + 1] as f32 * dst_a * (1.0 - src_a)) / out_a) as u8;
                                    pixels[idx + 2] = ((b as f32 * src_a + pixels[idx + 2] as f32 * dst_a * (1.0 - src_a)) / out_a) as u8;
                                    pixels[idx + 3] = (out_a * 255.0) as u8;
                                }
                            }
                        }
                    }
                });
            }

            cursor_x += advance;
        }
    }

    /// Render math expression with superscripts, subscripts, and special symbols.
    /// Supports: ^{} for superscript, _{} for subscript, Greek letters (\pi, \alpha, etc.)
    pub fn draw_math(
        &self,
        pixmap: &mut Pixmap,
        x: f32,
        y: f32,
        math_text: &str,
        font_size: f32,
        color: Color,
    ) {
        let parsed = parse_math(math_text);
        let total_width = self.measure_math_tokens(&parsed, font_size);
        let mut cursor_x = x - total_width / 2.0;
        let baseline_y = y;

        for token in &parsed {
            match token {
                MathToken::Normal(text) => {
                    let w = self.measure_text(text, font_size);
                    let center_x = cursor_x + w / 2.0;
                    self.draw_text(pixmap, center_x, baseline_y, text, font_size, color);
                    cursor_x += w;
                }
                MathToken::Superscript(text) => {
                    let small_size = font_size * 0.6;
                    let w = self.measure_text(text, small_size);
                    let center_x = cursor_x + w / 2.0;
                    self.draw_text(pixmap, center_x, baseline_y - font_size * 0.35, text, small_size, color);
                    cursor_x += w;
                }
                MathToken::Subscript(text) => {
                    let small_size = font_size * 0.6;
                    let w = self.measure_text(text, small_size);
                    let center_x = cursor_x + w / 2.0;
                    self.draw_text(pixmap, center_x, baseline_y + font_size * 0.25, text, small_size, color);
                    cursor_x += w;
                }
                MathToken::Fraction(num, den) => {
                    let small_size = font_size * 0.7;
                    let num_w = self.measure_text(num, small_size);
                    let den_w = self.measure_text(den, small_size);
                    let frac_w = num_w.max(den_w) + font_size * 0.3;

                    // Numerator (above center)
                    let center_x = cursor_x + frac_w / 2.0;
                    self.draw_text(pixmap, center_x, baseline_y - font_size * 0.35, num, small_size, color);

                    // Fraction line
                    let [r, g, b, a] = [
                        (color.red() * 255.0) as u8,
                        (color.green() * 255.0) as u8,
                        (color.blue() * 255.0) as u8,
                        (color.alpha() * 255.0) as u8,
                    ];
                    let line_color = Color::from_rgba8(r, g, b, a);
                    let mut paint = Paint::default();
                    paint.set_color(line_color);
                    paint.anti_alias = true;
                    let mut pb = PathBuilder::new();
                    pb.move_to(cursor_x + font_size * 0.1, baseline_y);
                    pb.line_to(cursor_x + frac_w - font_size * 0.1, baseline_y);
                    if let Some(path) = pb.finish() {
                        let mut stroke = Stroke::default();
                        stroke.width = 2.0;
                        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
                    }

                    // Denominator (below center)
                    self.draw_text(pixmap, center_x, baseline_y + font_size * 0.35, den, small_size, color);

                    cursor_x += frac_w;
                }
                MathToken::Symbol(sym) => {
                    let symbol_char = match sym.as_str() {
                        "pi" => "π",
                        "alpha" => "α",
                        "beta" => "β",
                        "gamma" => "γ",
                        "delta" => "δ",
                        "theta" => "θ",
                        "lambda" => "λ",
                        "mu" => "μ",
                        "sigma" => "σ",
                        "omega" => "ω",
                        "sum" => "Σ",
                        "product" => "Π",
                        "infinity" | "inf" => "∞",
                        "sqrt" => "√",
                        "times" => "×",
                        "div" => "÷",
                        "pm" => "±",
                        "neq" => "≠",
                        "leq" => "≤",
                        "geq" => "≥",
                        "approx" => "≈",
                        "arrow" | "rightarrow" => "→",
                        "leftarrow" => "←",
                        _ => "?",
                    };
                    let w = self.measure_text(symbol_char, font_size);
                    let center_x = cursor_x + w / 2.0;
                    self.draw_text(pixmap, center_x, baseline_y, symbol_char, font_size, color);
                    cursor_x += w;
                }
            }
        }
    }

    fn measure_text(&self, text: &str, font_size: f32) -> f32 {
        let scale = PxScale::from(font_size);
        let scaled_font = self.font.as_scaled(scale);
        text.chars().map(|c| {
            let glyph_id = self.font.glyph_id(c);
            scaled_font.h_advance(glyph_id)
        }).sum()
    }

    fn measure_math_tokens(&self, tokens: &[MathToken], font_size: f32) -> f32 {
        let mut total = 0.0;
        for token in tokens {
            total += match token {
                MathToken::Normal(text) => self.measure_text(text, font_size),
                MathToken::Superscript(text) => self.measure_text(text, font_size * 0.6),
                MathToken::Subscript(text) => self.measure_text(text, font_size * 0.6),
                MathToken::Fraction(num, den) => {
                    let small = font_size * 0.7;
                    let nw = self.measure_text(num, small);
                    let dw = self.measure_text(den, small);
                    nw.max(dw) + font_size * 0.3
                }
                MathToken::Symbol(sym) => {
                    let ch = match sym.as_str() {
                        "pi" => "π", "alpha" => "α", "beta" => "β",
                        "gamma" => "γ", "delta" => "δ", "theta" => "θ",
                        "lambda" => "λ", "mu" => "μ", "sigma" => "σ",
                        "omega" => "ω", "sum" => "Σ", "infinity" | "inf" => "∞",
                        "sqrt" => "√", "times" => "×", "div" => "÷",
                        "pm" => "±", "neq" => "≠", "leq" => "≤",
                        "geq" => "≥", "approx" => "≈", "arrow" | "rightarrow" => "→",
                        "leftarrow" => "←", _ => "?",
                    };
                    self.measure_text(ch, font_size)
                }
            };
        }
        total
    }
}

/// Math expression tokens
#[derive(Debug)]
enum MathToken {
    Normal(String),
    Superscript(String),
    Subscript(String),
    Fraction(String, String),
    Symbol(String),
}

/// Parse a math string into tokens.
/// Syntax: ^{sup} _{sub} \frac{num}{den} \symbol
fn parse_math(input: &str) -> Vec<MathToken> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut pos = 0;
    let mut current = String::new();

    while pos < chars.len() {
        let ch = chars[pos];

        if ch == '^' {
            if !current.is_empty() {
                tokens.push(MathToken::Normal(std::mem::take(&mut current)));
            }
            pos += 1;
            let content = read_braced_or_char(&chars, &mut pos);
            tokens.push(MathToken::Superscript(content));
        } else if ch == '_' {
            if !current.is_empty() {
                tokens.push(MathToken::Normal(std::mem::take(&mut current)));
            }
            pos += 1;
            let content = read_braced_or_char(&chars, &mut pos);
            tokens.push(MathToken::Subscript(content));
        } else if ch == '\\' {
            if !current.is_empty() {
                tokens.push(MathToken::Normal(std::mem::take(&mut current)));
            }
            pos += 1;
            let mut cmd = String::new();
            while pos < chars.len() && chars[pos].is_alphanumeric() {
                cmd.push(chars[pos]);
                pos += 1;
            }

            if cmd == "frac" {
                let num = read_braced_or_char(&chars, &mut pos);
                let den = read_braced_or_char(&chars, &mut pos);
                tokens.push(MathToken::Fraction(num, den));
            } else {
                tokens.push(MathToken::Symbol(cmd));
            }
        } else {
            current.push(ch);
            pos += 1;
        }
    }

    if !current.is_empty() {
        tokens.push(MathToken::Normal(current));
    }

    tokens
}

/// Read content in braces {content} or a single character.
fn read_braced_or_char(chars: &[char], pos: &mut usize) -> String {
    if *pos >= chars.len() {
        return String::new();
    }

    if chars[*pos] == '{' {
        *pos += 1; // skip '{'
        let mut content = String::new();
        let mut depth = 1;
        while *pos < chars.len() && depth > 0 {
            if chars[*pos] == '{' { depth += 1; }
            if chars[*pos] == '}' { depth -= 1; }
            if depth > 0 {
                content.push(chars[*pos]);
            }
            *pos += 1;
        }
        content
    } else {
        let ch = chars[*pos];
        *pos += 1;
        ch.to_string()
    }
}
