pub mod ast;

use crate::lexer::{Token, TokenKind};
use ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut scenes = Vec::new();
        let mut current_body: Vec<Statement> = Vec::new();
        let mut current_title = String::from("Untitled");
        let mut has_scene = false;

        self.skip_newlines();

        while !self.is_at_end() {
            match self.current().kind {
                TokenKind::Scene => {
                    if has_scene {
                        scenes.push(Scene {
                            title: current_title.clone(),
                            body: std::mem::take(&mut current_body),
                        });
                    }
                    self.advance(); // skip 'scene'
                    current_title = self.expect_string()?;
                    has_scene = true;
                    self.skip_newlines();
                }
                TokenKind::Next => {
                    self.advance(); // skip 'next'
                    self.expect(TokenKind::Scene)?;
                    if has_scene {
                        scenes.push(Scene {
                            title: current_title.clone(),
                            body: std::mem::take(&mut current_body),
                        });
                    }
                    current_title = String::from("Untitled");
                    has_scene = true;
                    self.skip_newlines();
                }
                TokenKind::Newline | TokenKind::Indent => {
                    self.skip_newlines();
                }
                _ => {
                    if !has_scene {
                        // Auto-create a default scene
                        has_scene = true;
                    }
                    let stmt = self.parse_statement()?;
                    current_body.push(stmt);
                    self.skip_newlines();
                }
            }
        }

        if has_scene {
            scenes.push(Scene {
                title: current_title,
                body: current_body,
            });
        }

        Ok(Program { scenes })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current().kind {
            TokenKind::Set => self.parse_set(),
            TokenKind::Draw => self.parse_draw(),
            TokenKind::Fade => self.parse_fade(),
            TokenKind::Move => self.parse_move(),
            TokenKind::Rotate => self.parse_rotate(),
            TokenKind::Scale => self.parse_scale(),
            TokenKind::Change => self.parse_change_color(),
            TokenKind::Wait => self.parse_wait(),
            TokenKind::Grow => self.parse_grow(),
            TokenKind::Shrink => self.parse_shrink(),
            TokenKind::Write => self.parse_write(),
            TokenKind::Highlight => self.parse_highlight(),
            TokenKind::Group => self.parse_group(),
            TokenKind::Style => self.parse_style(),
            TokenKind::Plot => self.parse_plot(),
            // Manim-inspired animations
            TokenKind::Indicate => self.parse_indicate(),
            TokenKind::Wiggle => self.parse_wiggle(),
            TokenKind::Circumscribe => self.parse_circumscribe(),
            TokenKind::Spiral => self.parse_spiral(),
            // New animations
            TokenKind::Morph => self.parse_morph(),
            TokenKind::Zoom => self.parse_zoom(),
            TokenKind::Pan => self.parse_pan(),
            TokenKind::Emit => self.parse_emit(),
            // "spin in" and "draw in" start with Identifier
            TokenKind::Identifier => {
                let word = self.current().value.clone().unwrap_or_default();
                match word.as_str() {
                    "spin" => self.parse_spin_in(),
                    "flash" => self.parse_flash(),
                    _ => {
                        let tok = self.current().clone();
                        Err(ParseError {
                            message: format!("Unexpected token: {:?} '{}'", tok.kind, tok.value.unwrap_or_default()),
                            line: tok.line,
                            col: tok.col,
                        })
                    }
                }
            }
            _ => {
                let tok = self.current().clone();
                Err(ParseError {
                    message: format!("Unexpected token: {:?} '{}'", tok.kind, tok.value.unwrap_or_default()),
                    line: tok.line,
                    col: tok.col,
                })
            }
        }
    }

    // set background to dark blue
    // set resolution to 1920x1080
    // set fps to 60
    fn parse_set(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'set'
        let property = match self.current().kind {
            TokenKind::Background => {
                self.advance();
                "background".to_string()
            }
            TokenKind::ResolutionKw => {
                self.advance();
                "resolution".to_string()
            }
            TokenKind::Fps => {
                self.advance();
                "fps".to_string()
            }
            _ => {
                let name = self.expect_identifier_or_value()?;
                name
            }
        };
        self.expect(TokenKind::To)?;

        let value = self.parse_value()?;

        Ok(Statement::Set { property, value })
    }

    // draw a circle at center with radius 100 color red
    // draw text "Hello" at top color white size 48
    // draw a line from (0,0) to (500,500) color white thickness 3
    fn parse_draw(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'draw'

        // "draw in the circle over 1 second" — Manim DrawBorderThenFill
        if !self.is_line_end() && self.current().kind == TokenKind::In {
            self.advance(); // skip 'in'
            self.skip_article();
            let target = self.parse_target()?;
            let (duration, easing) = self.parse_duration()?;
            return Ok(Statement::Animate {
                kind: AnimKind::DrawIn,
                target,
                duration,
                easing: easing.clone(),
            });
        }

        self.skip_article(); // skip optional 'a'/'an'

        let shape = self.parse_shape_kind()?;

        let mut name = None;
        let mut position = Position::Center;
        let mut properties = Vec::new();
        let mut from_pos = None;
        let mut to_pos = None;

        // Parse remaining tokens on this line
        while !self.is_line_end() {
            match self.current().kind {
                TokenKind::At => {
                    self.advance();
                    position = self.parse_position()?;
                }
                TokenKind::From => {
                    self.advance();
                    from_pos = Some(self.parse_position()?);
                }
                TokenKind::To => {
                    self.advance();
                    to_pos = Some(self.parse_position()?);
                }
                TokenKind::With => {
                    self.advance();
                    // optional 'style Name' or property
                    if self.current().kind == TokenKind::Style {
                        self.advance();
                        let style_name = self.expect_identifier_or_value()?;
                        properties.push(ShapeProp::StyleRef(style_name));
                    } else {
                        let prop = self.parse_shape_prop()?;
                        properties.push(prop);
                    }
                }
                TokenKind::Color => {
                    self.advance();
                    let color = self.parse_color_value()?;
                    properties.push(ShapeProp::Color(color));
                }
                TokenKind::Size => {
                    self.advance();
                    let val = self.expect_number()?;
                    properties.push(ShapeProp::Size(val));
                }
                TokenKind::Radius => {
                    self.advance();
                    let val = self.expect_number()?;
                    properties.push(ShapeProp::Radius(val));
                }
                TokenKind::Thickness => {
                    self.advance();
                    let val = self.expect_number()?;
                    properties.push(ShapeProp::Thickness(val));
                }
                TokenKind::Amplitude => {
                    self.advance();
                    let val = self.expect_number()?;
                    properties.push(ShapeProp::Amplitude(val));
                }
                TokenKind::Frequency => {
                    self.advance();
                    let val = self.expect_number()?;
                    properties.push(ShapeProp::Frequency(val));
                }
                TokenKind::Identifier => {
                    // Could be a name like "myCircle"
                    if name.is_none() {
                        name = Some(self.current().value.clone().unwrap_or_default());
                    }
                    self.advance();
                }
                _ => {
                    self.advance(); // skip unknown tokens on the line
                }
            }
        }

        // For line/arrow, use from/to positions
        if let (Some(from), Some(to)) = (from_pos.clone(), to_pos.clone()) {
            if matches!(shape, ShapeKind::Line | ShapeKind::Arrow) {
                return Ok(Statement::Draw {
                    name,
                    shape,
                    position: from,
                    properties,
                    end_position: Some(to),
                });
            }
        }

        // If 'to' was parsed (for line), use it as end_position
        Ok(Statement::Draw {
            name,
            shape,
            position: if let Some(f) = from_pos { f } else { position },
            properties,
            end_position: to_pos,
        })
    }

    // fade in the circle over 1 second
    // fade out everything over 1 second
    fn parse_fade(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'fade'
        let fade_in = match self.current().kind {
            TokenKind::In => { self.advance(); true }
            TokenKind::Out => { self.advance(); false }
            _ => true,
        };
        self.skip_article(); // skip 'the'

        let target = self.parse_target()?;
        let (duration, easing) = self.parse_duration()?;

        if fade_in {
            Ok(Statement::Animate {
                kind: AnimKind::FadeIn,
                target,
                duration,
                easing: easing.clone(),
            })
        } else {
            Ok(Statement::Animate {
                kind: AnimKind::FadeOut,
                target,
                duration,
                easing,
            })
        }
    }

    // move the square to (500, 500) over 2 seconds
    // move the square to center over 2 seconds
    fn parse_move(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'move'
        self.skip_article();

        let target = self.parse_target()?;

        self.expect(TokenKind::To)?;
        let to_pos = self.parse_position()?;
        let (duration, easing) = self.parse_duration()?;

        Ok(Statement::Animate {
            kind: AnimKind::MoveTo(to_pos),
            target,
            duration,
            easing: easing.clone(),
        })
    }

    // rotate the triangle by 90 over 1 second
    fn parse_rotate(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'rotate'
        self.skip_article();

        let target = self.parse_target()?;

        self.expect(TokenKind::By)?;
        let degrees = self.expect_number()?;
        let (duration, easing) = self.parse_duration()?;

        Ok(Statement::Animate {
            kind: AnimKind::Rotate(degrees),
            target,
            duration,
            easing: easing.clone(),
        })
    }

    // scale the circle to 2x over 0.5 seconds
    fn parse_scale(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'scale'
        self.skip_article();

        let target = self.parse_target()?;

        self.expect(TokenKind::To)?;
        let factor = self.expect_number()?;
        // skip optional 'x'
        if !self.is_line_end() && self.current().kind == TokenKind::XSuffix {
            self.advance();
        }
        let (duration, easing) = self.parse_duration()?;

        Ok(Statement::Animate {
            kind: AnimKind::Scale(factor),
            target,
            duration,
            easing: easing.clone(),
        })
    }

    // change color of the square to blue over 1 second
    fn parse_change_color(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'change'
        self.expect(TokenKind::Color)?;
        self.expect(TokenKind::Of)?;
        self.skip_article();

        let target = self.parse_target()?;

        self.expect(TokenKind::To)?;
        let color = self.parse_color_value()?;
        let (duration, easing) = self.parse_duration()?;

        Ok(Statement::Animate {
            kind: AnimKind::ChangeColor(color),
            target,
            duration,
            easing: easing.clone(),
        })
    }

    // wait 1 second
    // wait 0.5 seconds
    fn parse_wait(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'wait'
        let duration = self.expect_number()?;
        // skip optional 'second'/'seconds'
        if !self.is_line_end() && self.current().kind == TokenKind::Second {
            self.advance();
        }
        Ok(Statement::Wait { duration })
    }

    // grow the circle to radius 200 over 2 seconds
    fn parse_grow(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'grow'

        // "grow in the circle over 1 second" — Manim GrowFromCenter
        if !self.is_line_end() && self.current().kind == TokenKind::In {
            self.advance(); // skip 'in'
            self.skip_article();
            let target = self.parse_target()?;
            let (duration, easing) = self.parse_duration()?;
            return Ok(Statement::Animate {
                kind: AnimKind::GrowIn,
                target,
                duration,
                easing: easing.clone(),
            });
        }

        // "grow the circle to radius 200" — existing
        self.skip_article();
        let target = self.parse_target()?;
        self.expect(TokenKind::To)?;

        // optional 'radius' keyword
        if !self.is_line_end() && self.current().kind == TokenKind::Radius {
            self.advance();
        }
        let value = self.expect_number()?;
        let (duration, easing) = self.parse_duration()?;

        Ok(Statement::Animate {
            kind: AnimKind::Grow(value),
            target,
            duration,
            easing: easing.clone(),
        })
    }

    // shrink the circle to radius 50 over 1 second
    fn parse_shrink(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'shrink'
        self.skip_article();
        let target = self.parse_target()?;
        self.expect(TokenKind::To)?;

        if !self.is_line_end() && self.current().kind == TokenKind::Radius {
            self.advance();
        }
        let value = self.expect_number()?;
        let (duration, easing) = self.parse_duration()?;

        Ok(Statement::Animate {
            kind: AnimKind::Shrink(value),
            target,
            duration,
            easing: easing.clone(),
        })
    }

    // write "a² + b² = c²" at bottom color white size 36 over 1.5 seconds
    fn parse_write(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'write'
        let content = self.expect_string()?;

        let mut position = Position::Center;
        let mut color = ColorValue::Named("white".to_string());
        let mut size = 32.0;
        let mut duration = 1.0;

        while !self.is_line_end() {
            match self.current().kind {
                TokenKind::At => {
                    self.advance();
                    position = self.parse_position()?;
                }
                TokenKind::Color => {
                    self.advance();
                    color = self.parse_color_value()?;
                }
                TokenKind::Size => {
                    self.advance();
                    size = self.expect_number()?;
                }
                TokenKind::Over => {
                    self.advance();
                    duration = self.expect_number()?;
                    if !self.is_line_end() && self.current().kind == TokenKind::Second {
                        self.advance();
                    }
                }
                _ => { self.advance(); }
            }
        }

        Ok(Statement::Write {
            content,
            position,
            color,
            size,
            duration,
        })
    }

    // highlight the triangle color red over 1 second
    fn parse_highlight(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'highlight'
        self.skip_article();
        let target = self.parse_target()?;
        let mut color = ColorValue::Named("yellow".to_string());
        let mut duration = 1.0;
        let mut easing = "smooth".to_string();

        while !self.is_line_end() {
            match self.current().kind {
                TokenKind::Color => {
                    self.advance();
                    color = self.parse_color_value()?;
                }
                TokenKind::Over => {
                    self.advance();
                    duration = self.expect_number()?;
                    if !self.is_line_end() && self.current().kind == TokenKind::Second {
                        self.advance();
                    }
                }
                TokenKind::Easing => {
                    self.advance();
                    if !self.is_line_end() {
                        easing = self.current().value.clone().unwrap_or("smooth".into());
                        self.advance();
                    }
                }
                _ => { self.advance(); }
            }
        }

        Ok(Statement::Animate {
            kind: AnimKind::Highlight(color),
            target,
            duration,
            easing,
        })
    }

    // --- Manim-inspired animation parsers ---

    // spin in the circle over 1.5 seconds
    fn parse_spin_in(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'spin'
        if !self.is_line_end() && self.current().kind == TokenKind::In {
            self.advance(); // skip 'in'
        }
        self.skip_article();
        let target = self.parse_target()?;
        let (duration, easing) = self.parse_duration()?;
        Ok(Statement::Animate {
            kind: AnimKind::SpinIn,
            target,
            duration,
            easing: easing.clone(),
        })
    }

    // indicate the circle over 0.5 seconds
    fn parse_indicate(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'indicate'
        self.skip_article();
        let target = self.parse_target()?;
        let (duration, easing) = self.parse_duration()?;
        Ok(Statement::Animate {
            kind: AnimKind::Indicate,
            target,
            duration,
            easing: easing.clone(),
        })
    }

    // wiggle the circle over 0.5 seconds
    fn parse_wiggle(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'wiggle'
        self.skip_article();
        let target = self.parse_target()?;
        let (duration, easing) = self.parse_duration()?;
        Ok(Statement::Animate {
            kind: AnimKind::Wiggle,
            target,
            duration,
            easing: easing.clone(),
        })
    }

    // circumscribe the circle color yellow over 1 second
    fn parse_circumscribe(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'circumscribe'
        self.skip_article();
        let target = self.parse_target()?;
        let mut color = ColorValue::Named("yellow".to_string());
        let mut duration = 1.0;
        let mut easing = "smooth".to_string();

        while !self.is_line_end() {
            match self.current().kind {
                TokenKind::Color => {
                    self.advance();
                    color = self.parse_color_value()?;
                }
                TokenKind::Over => {
                    self.advance();
                    duration = self.expect_number()?;
                    if !self.is_line_end() && self.current().kind == TokenKind::Second {
                        self.advance();
                    }
                }
                TokenKind::Easing => {
                    self.advance();
                    if !self.is_line_end() {
                        easing = self.current().value.clone().unwrap_or("smooth".into());
                        self.advance();
                    }
                }
                _ => { self.advance(); }
            }
        }

        Ok(Statement::Animate {
            kind: AnimKind::Circumscribe(color),
            target,
            duration,
            easing,
        })
    }

    // spiral in the triangle over 1.5 seconds
    fn parse_spiral(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'spiral'
        if !self.is_line_end() && self.current().kind == TokenKind::In {
            self.advance(); // skip 'in'
        }
        self.skip_article();
        let target = self.parse_target()?;
        let (duration, easing) = self.parse_duration()?;
        Ok(Statement::Animate {
            kind: AnimKind::SpiralIn,
            target,
            duration,
            easing: easing.clone(),
        })
    }

    // flash at center color yellow over 0.5 seconds
    fn parse_flash(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'flash'
        self.skip_article();
        let target = self.parse_target()?;
        let mut color = ColorValue::Named("yellow".to_string());
        let mut duration = 0.5;
        let mut easing = "smooth".to_string();

        while !self.is_line_end() {
            match self.current().kind {
                TokenKind::Color => {
                    self.advance();
                    color = self.parse_color_value()?;
                }
                TokenKind::Over => {
                    self.advance();
                    duration = self.expect_number()?;
                    if !self.is_line_end() && self.current().kind == TokenKind::Second {
                        self.advance();
                    }
                }
                TokenKind::Easing => {
                    self.advance();
                    if !self.is_line_end() {
                        easing = self.current().value.clone().unwrap_or("smooth".into());
                        self.advance();
                    }
                }
                _ => { self.advance(); }
            }
        }

        Ok(Statement::Animate {
            kind: AnimKind::Flash(color),
            target,
            duration,
            easing,
        })
    }

    // morph the circle into a square over 2 seconds
    fn parse_morph(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'morph'
        self.skip_article();
        let target = self.parse_target()?;

        // expect 'into'
        if !self.is_line_end() && self.current().kind == TokenKind::Into {
            self.advance();
        }
        self.skip_article();

        let shape = self.parse_shape_kind()?;
        let (duration, easing) = self.parse_duration()?;

        Ok(Statement::Animate {
            kind: AnimKind::MorphInto(shape),
            target,
            duration,
            easing: easing.clone(),
        })
    }

    // zoom in to 2x over 1 second
    // zoom out to 1x over 1 second
    fn parse_zoom(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'zoom'
        // optional 'in' or 'out'
        if !self.is_line_end() && (self.current().kind == TokenKind::In || self.current().kind == TokenKind::Out) {
            self.advance();
        }
        self.expect(TokenKind::To)?;
        let factor = self.expect_number()?;
        // skip optional 'x'
        if !self.is_line_end() && self.current().kind == TokenKind::XSuffix {
            self.advance();
        }
        let (duration, easing) = self.parse_duration()?;

        Ok(Statement::Animate {
            kind: AnimKind::ZoomTo(factor),
            target: Target::Everything,
            duration,
            easing: easing.clone(),
        })
    }

    // pan to center over 1 second
    // pan to (500, 300) over 1.5 seconds
    fn parse_pan(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'pan'
        self.expect(TokenKind::To)?;
        let position = self.parse_position()?;
        let (duration, easing) = self.parse_duration()?;

        Ok(Statement::Animate {
            kind: AnimKind::PanTo(position),
            target: Target::Everything,
            duration,
            easing: easing.clone(),
        })
    }

    // emit particles at center color cyan count 50 over 2 seconds
    fn parse_emit(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'emit'
        // optional 'particles'
        if !self.is_line_end() && self.current().kind == TokenKind::Particles {
            self.advance();
        }

        let mut position = Position::Center;
        let mut color = ColorValue::Named("white".to_string());
        let mut count: usize = 30;
        let mut duration = 1.5;

        while !self.is_line_end() {
            match self.current().kind {
                TokenKind::At => {
                    self.advance();
                    position = self.parse_position()?;
                }
                TokenKind::Color => {
                    self.advance();
                    color = self.parse_color_value()?;
                }
                TokenKind::Count => {
                    self.advance();
                    count = self.expect_number()? as usize;
                }
                TokenKind::Over => {
                    self.advance();
                    duration = self.expect_number()?;
                    if !self.is_line_end() && self.current().kind == TokenKind::Second {
                        self.advance();
                    }
                }
                _ => { self.advance(); }
            }
        }

        Ok(Statement::Emit {
            position,
            color,
            count,
            duration,
        })
    }

    // group triangle_demo:
    //     draw a triangle at center color white
    fn parse_group(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'group'
        let name = self.expect_identifier_or_value()?;
        self.expect(TokenKind::Colon)?;
        self.skip_newlines();

        let mut body = Vec::new();
        // Parse indented statements
        while !self.is_at_end() && self.current().kind == TokenKind::Indent {
            self.advance(); // skip indent
            if self.is_at_end() || self.current().kind == TokenKind::Newline {
                self.skip_newlines();
                continue;
            }
            let stmt = self.parse_statement()?;
            body.push(stmt);
            self.skip_newlines();
        }

        Ok(Statement::Group { name, body })
    }

    // style hero:
    //     color is white
    //     size is 48
    fn parse_style(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'style'
        let name = self.expect_identifier_or_value()?;
        self.expect(TokenKind::Colon)?;
        self.skip_newlines();

        let mut properties = Vec::new();
        while !self.is_at_end() && self.current().kind == TokenKind::Indent {
            self.advance(); // skip indent
            if self.is_at_end() || self.current().kind == TokenKind::Newline {
                self.skip_newlines();
                continue;
            }
            let prop = self.parse_style_prop()?;
            properties.push(prop);
            self.skip_newlines();
        }

        Ok(Statement::StyleDef { name, properties })
    }

    // plot "sin(x)" from -3.14 to 3.14 color cyan thickness 2 over 2 seconds
    fn parse_plot(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // skip 'plot'

        // Expression string
        let expr = if !self.is_line_end() && self.current().kind == TokenKind::StringLit {
            self.expect_string()?
        } else {
            "sin(x)".to_string()
        };

        let mut x_min = -5.0;
        let mut x_max = 5.0;
        let mut y_min = -3.0;
        let mut y_max = 3.0;
        let mut color = ColorValue::Named("cyan".to_string());
        let mut thickness = 2.5;
        let mut duration = 1.5;

        while !self.is_line_end() {
            match self.current().kind {
                TokenKind::From => {
                    self.advance();
                    x_min = self.expect_number_allow_negative()?;
                }
                TokenKind::To => {
                    self.advance();
                    x_max = self.expect_number_allow_negative()?;
                }
                TokenKind::Color => {
                    self.advance();
                    color = self.parse_color_value()?;
                }
                TokenKind::Thickness => {
                    self.advance();
                    thickness = self.expect_number()?;
                }
                TokenKind::Over => {
                    self.advance();
                    duration = self.expect_number()?;
                    if !self.is_line_end() && self.current().kind == TokenKind::Second {
                        self.advance();
                    }
                }
                _ => { self.advance(); }
            }
        }

        Ok(Statement::Plot {
            expr,
            x_range: (x_min, x_max),
            y_range: (y_min, y_max),
            color,
            thickness,
            duration,
        })
    }

    fn parse_style_prop(&mut self) -> Result<StyleProp, ParseError> {
        let prop_name = match self.current().kind {
            TokenKind::Color => { self.advance(); "color".to_string() }
            TokenKind::Size => { self.advance(); "size".to_string() }
            TokenKind::Font => { self.advance(); "font".to_string() }
            TokenKind::Bold => { self.advance(); "bold".to_string() }
            TokenKind::Background => { self.advance(); "background".to_string() }
            _ => self.expect_identifier_or_value()?,
        };

        self.expect(TokenKind::Is)?;
        let value = self.parse_value()?;

        Ok(StyleProp {
            name: prop_name,
            value,
        })
    }

    // --- Helpers ---

    fn parse_shape_kind(&mut self) -> Result<ShapeKind, ParseError> {
        let kind = match self.current().kind {
            TokenKind::Circle => ShapeKind::Circle,
            TokenKind::Square => ShapeKind::Square,
            TokenKind::Rectangle => ShapeKind::Rectangle,
            TokenKind::Triangle => ShapeKind::Triangle,
            TokenKind::Line => ShapeKind::Line,
            TokenKind::Arrow => ShapeKind::Arrow,
            TokenKind::Text => {
                self.advance();
                let content = if !self.is_line_end() && self.current().kind == TokenKind::StringLit {
                    self.expect_string()?
                } else {
                    String::new()
                };
                return Ok(ShapeKind::Text(content));
            }
            TokenKind::Math => {
                self.advance();
                let content = if !self.is_line_end() && self.current().kind == TokenKind::StringLit {
                    self.expect_string()?
                } else {
                    String::new()
                };
                return Ok(ShapeKind::Math(content));
            }
            TokenKind::Wave => {
                self.advance();
                return Ok(ShapeKind::Wave { amplitude: 50.0, frequency: 2.0 });
            }
            TokenKind::Grid => {
                self.advance();
                return Ok(ShapeKind::Grid);
            }
            TokenKind::Curve => {
                self.advance();
                return Ok(ShapeKind::Curve { points: Vec::new() });
            }
            TokenKind::Axes => {
                self.advance();
                return Ok(ShapeKind::NumberAxis {
                    x_range: (-5.0, 5.0),
                    y_range: (-3.0, 3.0),
                });
            }
            _ => {
                return Err(ParseError {
                    message: format!("Expected shape, got {:?}", self.current().kind),
                    line: self.current().line,
                    col: self.current().col,
                });
            }
        };
        self.advance();
        Ok(kind)
    }

    fn parse_position(&mut self) -> Result<Position, ParseError> {
        match self.current().kind {
            TokenKind::Center => { self.advance(); Ok(Position::Center) }
            TokenKind::Top => { self.advance(); Ok(Position::Top) }
            TokenKind::Bottom => { self.advance(); Ok(Position::Bottom) }
            TokenKind::Left => { self.advance(); Ok(Position::Left) }
            TokenKind::Right => { self.advance(); Ok(Position::Right) }
            TokenKind::LParen => {
                self.advance(); // skip '('
                let x = self.expect_number()?;
                self.expect(TokenKind::Comma)?;
                let y = self.expect_number()?;
                self.expect(TokenKind::RParen)?;
                Ok(Position::Coords(x, y))
            }
            _ => {
                Err(ParseError {
                    message: format!("Expected position, got {:?}", self.current().kind),
                    line: self.current().line,
                    col: self.current().col,
                })
            }
        }
    }

    fn parse_target(&mut self) -> Result<Target, ParseError> {
        match self.current().kind {
            TokenKind::Everything => {
                self.advance();
                Ok(Target::Everything)
            }
            TokenKind::Circle => { self.advance(); Ok(Target::LastShape(ShapeKind::Circle)) }
            TokenKind::Square => { self.advance(); Ok(Target::LastShape(ShapeKind::Square)) }
            TokenKind::Rectangle => { self.advance(); Ok(Target::LastShape(ShapeKind::Rectangle)) }
            TokenKind::Triangle => { self.advance(); Ok(Target::LastShape(ShapeKind::Triangle)) }
            TokenKind::Line => { self.advance(); Ok(Target::LastShape(ShapeKind::Line)) }
            TokenKind::Arrow => { self.advance(); Ok(Target::LastShape(ShapeKind::Arrow)) }
            TokenKind::Text => { self.advance(); Ok(Target::LastShape(ShapeKind::Text(String::new()))) }
            TokenKind::Math => { self.advance(); Ok(Target::LastShape(ShapeKind::Math(String::new()))) }
            TokenKind::Wave => { self.advance(); Ok(Target::LastShape(ShapeKind::Wave { amplitude: 50.0, frequency: 2.0 })) }
            TokenKind::Grid => { self.advance(); Ok(Target::LastShape(ShapeKind::Grid)) }
            TokenKind::Curve => { self.advance(); Ok(Target::LastShape(ShapeKind::Curve { points: vec![] })) }
            TokenKind::Axes => { self.advance(); Ok(Target::LastShape(ShapeKind::NumberAxis { x_range: (-5.0, 5.0), y_range: (-3.0, 3.0) })) }
            TokenKind::Identifier => {
                let name = self.current().value.clone().unwrap_or_default();
                self.advance();
                Ok(Target::Named(name))
            }
            _ => {
                Err(ParseError {
                    message: format!("Expected target (shape name or 'everything'), got {:?}", self.current().kind),
                    line: self.current().line,
                    col: self.current().col,
                })
            }
        }
    }

    fn parse_color_value(&mut self) -> Result<ColorValue, ParseError> {
        match self.current().kind {
            TokenKind::NamedColor => {
                let mut color_name = self.current().value.clone().unwrap_or_default();
                self.advance();

                // Check for "gradient <color1> <color2>"
                if color_name == "gradient" {
                    let c1 = self.parse_color_value()?;
                    let c2 = self.parse_color_value()?;
                    return Ok(ColorValue::Gradient(Box::new(c1), Box::new(c2)));
                }

                // Handle compound colors like "dark blue", "light green"
                // Only combine if prefix is "dark" or "light"
                if (color_name == "dark" || color_name == "light")
                    && !self.is_line_end()
                    && self.current().kind == TokenKind::NamedColor
                {
                    let next_name = self.current().value.clone().unwrap_or_default();
                    color_name.push(' ');
                    color_name.push_str(&next_name);
                    self.advance();
                }
                Ok(ColorValue::Named(color_name))
            }
            TokenKind::Identifier => {
                let name = self.current().value.clone().unwrap_or_default();
                if name == "gradient" {
                    self.advance();
                    let c1 = self.parse_color_value()?;
                    let c2 = self.parse_color_value()?;
                    return Ok(ColorValue::Gradient(Box::new(c1), Box::new(c2)));
                }
                Err(ParseError {
                    message: format!("Expected color, got identifier '{}'", name),
                    line: self.current().line,
                    col: self.current().col,
                })
            }
            TokenKind::StringLit => {
                let hex = self.expect_string()?;
                Ok(ColorValue::Hex(hex))
            }
            _ => {
                Err(ParseError {
                    message: format!("Expected color, got {:?}", self.current().kind),
                    line: self.current().line,
                    col: self.current().col,
                })
            }
        }
    }

    fn parse_shape_prop(&mut self) -> Result<ShapeProp, ParseError> {
        match self.current().kind {
            TokenKind::Radius => {
                self.advance();
                let val = self.expect_number()?;
                Ok(ShapeProp::Radius(val))
            }
            TokenKind::Size => {
                self.advance();
                let val = self.expect_number()?;
                Ok(ShapeProp::Size(val))
            }
            TokenKind::Thickness => {
                self.advance();
                let val = self.expect_number()?;
                Ok(ShapeProp::Thickness(val))
            }
            TokenKind::Color => {
                self.advance();
                let color = self.parse_color_value()?;
                Ok(ShapeProp::Color(color))
            }
            _ => {
                Err(ParseError {
                    message: format!("Expected shape property, got {:?}", self.current().kind),
                    line: self.current().line,
                    col: self.current().col,
                })
            }
        }
    }

    fn parse_duration(&mut self) -> Result<(f64, String), ParseError> {
        let mut dur = 1.0;
        let mut easing = "smooth".to_string();

        while !self.is_line_end() {
            match self.current().kind {
                TokenKind::Over => {
                    self.advance();
                    dur = self.expect_number()?;
                    if !self.is_line_end() && self.current().kind == TokenKind::Second {
                        self.advance();
                    }
                }
                TokenKind::Easing => {
                    self.advance();
                    if !self.is_line_end() {
                        easing = self.current().value.clone().unwrap_or("smooth".into());
                        self.advance();
                    }
                }
                _ => break,
            }
        }

        Ok((dur, easing))
    }

    fn parse_value(&mut self) -> Result<Value, ParseError> {
        match self.current().kind {
            TokenKind::NumberLit => {
                let n = self.expect_number()?;
                Ok(Value::Number(n))
            }
            TokenKind::StringLit => {
                let s = self.expect_string()?;
                Ok(Value::String(s))
            }
            TokenKind::Resolution => {
                let res = self.current().value.clone().unwrap_or_default();
                self.advance();
                Ok(Value::String(res))
            }
            TokenKind::NamedColor => {
                let color = self.parse_color_value()?;
                match color {
                    ColorValue::Named(n) => Ok(Value::String(n)),
                    ColorValue::Hex(h) => Ok(Value::String(h)),
                    ColorValue::Gradient(a, b) => {
                        // Serialize as "gradient <color1> <color2>" for background
                        let c1 = match *a {
                            ColorValue::Named(ref n) => n.clone(),
                            ColorValue::Hex(ref h) => h.clone(),
                            _ => "white".to_string(),
                        };
                        let c2 = match *b {
                            ColorValue::Named(ref n) => n.clone(),
                            ColorValue::Hex(ref h) => h.clone(),
                            _ => "black".to_string(),
                        };
                        Ok(Value::String(format!("gradient {} {}", c1, c2)))
                    }
                }
            }
            TokenKind::Identifier => {
                let s = self.current().value.clone().unwrap_or_default();
                self.advance();
                Ok(Value::String(s))
            }
            TokenKind::Bold => {
                self.advance();
                Ok(Value::Bool(true))
            }
            _ => {
                Err(ParseError {
                    message: format!("Expected value, got {:?}", self.current().kind),
                    line: self.current().line,
                    col: self.current().col,
                })
            }
        }
    }

    // --- Token utilities ---

    fn current(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len() || self.current().kind == TokenKind::Eof
    }

    fn is_line_end(&self) -> bool {
        self.is_at_end() || matches!(self.current().kind, TokenKind::Newline | TokenKind::Eof)
    }

    fn skip_newlines(&mut self) {
        while !self.is_at_end() && matches!(self.current().kind, TokenKind::Newline | TokenKind::Indent) {
            self.advance();
        }
    }

    fn skip_article(&mut self) {
        if !self.is_at_end() && matches!(self.current().kind, TokenKind::A | TokenKind::The) {
            self.advance();
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        if self.current().kind == kind {
            self.advance();
            Ok(())
        } else {
            Err(ParseError {
                message: format!("Expected {:?}, got {:?}", kind, self.current().kind),
                line: self.current().line,
                col: self.current().col,
            })
        }
    }

    fn expect_string(&mut self) -> Result<String, ParseError> {
        if self.current().kind == TokenKind::StringLit {
            let val = self.current().value.clone().unwrap_or_default();
            self.advance();
            Ok(val)
        } else {
            Err(ParseError {
                message: format!("Expected string, got {:?}", self.current().kind),
                line: self.current().line,
                col: self.current().col,
            })
        }
    }

    fn expect_number(&mut self) -> Result<f64, ParseError> {
        if self.current().kind == TokenKind::NumberLit {
            let val = self.current().value.clone().unwrap_or_default();
            self.advance();
            val.parse::<f64>().map_err(|_| ParseError {
                message: format!("Invalid number: {}", val),
                line: self.current().line,
                col: self.current().col,
            })
        } else {
            Err(ParseError {
                message: format!("Expected number, got {:?}", self.current().kind),
                line: self.current().line,
                col: self.current().col,
            })
        }
    }

    /// Parse a number that may be negative (preceded by a minus sign identifier).
    fn expect_number_allow_negative(&mut self) -> Result<f64, ParseError> {
        let negative = if let Some(val) = &self.current().value {
            val == "-"
        } else {
            false
        };
        if negative {
            self.advance();
            let n = self.expect_number()?;
            Ok(-n)
        } else {
            self.expect_number()
        }
    }

    fn expect_identifier_or_value(&mut self) -> Result<String, ParseError> {
        let val = self.current().value.clone().unwrap_or_default();
        if !val.is_empty() {
            self.advance();
            Ok(val)
        } else {
            Err(ParseError {
                message: format!("Expected identifier, got {:?}", self.current().kind),
                line: self.current().line,
                col: self.current().col,
            })
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub col: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at {}:{}: {}", self.line, self.col, self.message)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use super::*;

    fn parse(input: &str) -> Program {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse().unwrap()
    }

    #[test]
    fn test_simple_scene() {
        let prog = parse(r#"scene "Hello"
set background to dark blue
draw a circle at center color red"#);
        assert_eq!(prog.scenes.len(), 1);
        assert_eq!(prog.scenes[0].title, "Hello");
        assert_eq!(prog.scenes[0].body.len(), 2);
    }

    #[test]
    fn test_draw_with_properties() {
        let prog = parse(r#"scene "Test"
draw a circle at center with radius 100 color red"#);
        match &prog.scenes[0].body[0] {
            Statement::Draw { shape, position, properties, .. } => {
                assert!(matches!(shape, ShapeKind::Circle));
                assert!(matches!(position, Position::Center));
                assert!(properties.len() >= 1);
            }
            _ => panic!("Expected Draw statement"),
        }
    }

    #[test]
    fn test_animation() {
        let prog = parse(r#"scene "Test"
fade in the circle over 2 seconds"#);
        match &prog.scenes[0].body[0] {
            Statement::Animate { kind, duration, .. } => {
                assert!(matches!(kind, AnimKind::FadeIn));
                assert_eq!(*duration, 2.0);
            }
            _ => panic!("Expected Animate statement"),
        }
    }

    #[test]
    fn test_wait() {
        let prog = parse(r#"scene "Test"
wait 1.5 seconds"#);
        match &prog.scenes[0].body[0] {
            Statement::Wait { duration } => {
                assert_eq!(*duration, 1.5);
            }
            _ => panic!("Expected Wait statement"),
        }
    }

    #[test]
    fn test_multiple_scenes() {
        let prog = parse(r#"scene "Scene 1"
draw a circle at center color red
next scene
draw a square at center color blue"#);
        assert_eq!(prog.scenes.len(), 2);
    }
}
