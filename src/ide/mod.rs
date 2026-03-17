use std::io::Cursor;
use std::sync::{Arc, Mutex};
use tiny_http::{Server, Response, Header};

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::renderer::Renderer;

const IDE_HTML: &str = include_str!("../../assets/ide.html");

/// Start the split-screen IDE web server.
pub fn start_ide(file: Option<&str>, port: u16) {
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr).expect("Failed to start IDE server");

    let initial_code = if let Some(path) = file {
        std::fs::read_to_string(path).unwrap_or_else(|_| get_default_code())
    } else {
        get_default_code()
    };

    let shared_code = Arc::new(Mutex::new(initial_code));

    println!("🎨 Slang IDE running at http://localhost:{}", port);
    println!("   Open this URL in your browser!");
    println!();

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();
        let method = request.method().to_string();

        match (method.as_str(), url.as_str()) {
            ("GET", "/") => {
                let code = shared_code.lock().unwrap().clone();
                let html = IDE_HTML.replace("{{INITIAL_CODE}}", &code.replace('\\', "\\\\").replace('`', "\\`").replace("</", "<\\/"));
                let response = Response::from_string(html)
                    .with_header(Header::from_bytes("Content-Type", "text/html; charset=utf-8").unwrap());
                let _ = request.respond(response);
            }
            ("POST", "/render") => {
                // Read the source code from the request body
                let mut body = String::new();
                let mut reader = request.as_reader();
                let _ = std::io::Read::read_to_string(&mut reader, &mut body);

                // Parse JSON body
                let source = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                    json["code"].as_str().unwrap_or("").to_string()
                } else {
                    body
                };

                // Save the code
                *shared_code.lock().unwrap() = source.clone();

                // Render at preview quality (480p for speed)
                let result = render_preview(&source);

                let response = Response::from_string(result)
                    .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
                    .with_header(Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap());
                let _ = request.respond(response);
            }
            ("POST", "/render-frame") => {
                let mut body = String::new();
                let mut reader = request.as_reader();
                let _ = std::io::Read::read_to_string(&mut reader, &mut body);

                let json: serde_json::Value = serde_json::from_str(&body).unwrap_or_default();
                let source = json["code"].as_str().unwrap_or("").to_string();
                let frame_idx = json["frame"].as_u64().unwrap_or(0) as usize;

                let result = render_single_frame(&source, frame_idx);

                let response = Response::from_string(result)
                    .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
                    .with_header(Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap());
                let _ = request.respond(response);
            }
            ("POST", "/save") => {
                let mut body = String::new();
                let mut reader = request.as_reader();
                let _ = std::io::Read::read_to_string(&mut reader, &mut body);

                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                    if let (Some(path), Some(code)) = (json["path"].as_str(), json["code"].as_str()) {
                        let _ = std::fs::write(path, code);
                    }
                }

                let response = Response::from_string("{\"ok\":true}")
                    .with_header(Header::from_bytes("Content-Type", "application/json").unwrap());
                let _ = request.respond(response);
            }
            _ => {
                let response = Response::from_string("Not Found")
                    .with_status_code(404);
                let _ = request.respond(response);
            }
        }
    }
}

fn render_preview(source: &str) -> String {
    // Lex
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };

    // Parse
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };

    // Render at preview resolution (480p for speed)
    let mut renderer = Renderer::new();
    renderer.width = 854;
    renderer.height = 480;
    renderer.fps = 15; // low fps for fast preview
    let frames = renderer.render_program(&program);

    // Encode a few key frames as base64 PNGs for the preview
    let total_frames = frames.len();
    let num_preview_frames = 30.min(total_frames); // max 30 preview frames
    let step = if total_frames > 1 { (total_frames - 1) / (num_preview_frames - 1).max(1) } else { 1 };

    let mut preview_frames = Vec::new();
    let mut i = 0;
    while i < total_frames && preview_frames.len() < num_preview_frames {
        if let Some(png_data) = frame_to_png(&frames[i], renderer.width, renderer.height) {
            let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png_data);
            preview_frames.push(b64);
        }
        i += step;
    }

    let duration = total_frames as f64 / renderer.fps as f64;

    serde_json::json!({
        "frames": preview_frames,
        "totalFrames": total_frames,
        "width": renderer.width,
        "height": renderer.height,
        "fps": renderer.fps,
        "duration": duration,
        "scenes": program.scenes.len(),
    }).to_string()
}

fn render_single_frame(source: &str, frame_idx: usize) -> String {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };

    let mut renderer = Renderer::new();
    renderer.width = 854;
    renderer.height = 480;
    renderer.fps = 15;
    let frames = renderer.render_program(&program);

    let idx = frame_idx.min(frames.len().saturating_sub(1));
    if let Some(png_data) = frame_to_png(&frames[idx], renderer.width, renderer.height) {
        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png_data);
        serde_json::json!({"frame": b64, "index": idx, "total": frames.len()}).to_string()
    } else {
        serde_json::json!({"error": "Failed to encode frame"}).to_string()
    }
}

fn frame_to_png(rgba: &[u8], width: u32, height: u32) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    let cursor = Cursor::new(&mut buf);
    let encoder = image::codecs::png::PngEncoder::new(cursor);
    image::ImageEncoder::write_image(
        encoder,
        rgba,
        width,
        height,
        image::ExtendedColorType::Rgba8,
    ).ok()?;
    Some(buf)
}

fn get_default_code() -> String {
    r#"# Welcome to Slang IDE!
# Edit this code and click "Render" to see the preview.

scene "Hello Slang"

set background to dark blue
set fps to 15

draw a circle at center with radius 100 color cyan
fade in the circle over 1 second

wait 0.5 seconds

write "Hello, Slang!" at top color white size 48 over 1 second

wait 1 second

draw a square at left with size 80 color yellow
fade in the square over 0.5 seconds

draw a triangle at right with size 100 color pink
fade in the triangle over 0.5 seconds

wait 1 second

fade out everything over 1 second
"#.to_string()
}
