mod lexer;
mod parser;
mod renderer;
mod cli;
mod ide;

use clap::Parser;
use cli::{Cli, Commands};
use std::path::Path;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file, quality } => {
            cmd_run(&file, &quality);
        }
        Commands::Render { file, output, quality, frames } => {
            cmd_render(&file, output.as_deref(), &quality, frames);
        }
        Commands::New { name } => {
            cmd_new(&name);
        }
        Commands::Ide { file, port } => {
            ide::start_ide(file.as_deref(), port);
        }
    }
}

fn cmd_run(file: &str, quality: &str) {
    let output = format!("{}.mp4", Path::new(file).file_stem().unwrap().to_string_lossy());
    cmd_render(file, Some(&output), quality, false);

    // Try to open the video with the system player
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(&output).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(&output).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd").args(["/c", "start", &output]).spawn();
    }
}

fn cmd_render(file: &str, output: Option<&str>, quality: &str, export_frames: bool) {
    // Read the source file
    let source = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Error reading '{}': {}", file, e);
            std::process::exit(1);
        }
    };

    println!("🎬 Slang Renderer v0.1.0");
    println!("  Source: {}", file);

    // Lex
    println!("  Lexing...");
    let mut lexer_inst = lexer::Lexer::new(&source);
    let tokens = match lexer_inst.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("❌ {}", e);
            std::process::exit(1);
        }
    };
    println!("  ✅ {} tokens", tokens.len());

    // Parse
    println!("  Parsing...");
    let mut parser_inst = parser::Parser::new(tokens);
    let program = match parser_inst.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ {}", e);
            std::process::exit(1);
        }
    };
    println!("  ✅ {} scene(s)", program.scenes.len());

    // Render
    println!("  Rendering...");
    let mut render = renderer::Renderer::new();
    let frames = render.render_program(&program);
    println!("  ✅ {} frames ({}x{} @ {} fps)",
        frames.len(), render.width, render.height, render.fps
    );

    let duration = frames.len() as f64 / render.fps as f64;
    println!("  Duration: {:.1}s", duration);

    // Export
    if export_frames {
        let out_dir = output.unwrap_or("frames");
        if let Err(e) = renderer::export::export_to_pngs(&frames, render.width, render.height, out_dir) {
            eprintln!("❌ {}", e);
            std::process::exit(1);
        }
    } else {
        let out_file = output.unwrap_or("output.mp4");
        if let Err(e) = renderer::export::export_to_mp4(
            &frames, render.width, render.height, render.fps, out_file, quality
        ) {
            eprintln!("❌ {}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_new(name: &str) {
    let dir = Path::new(name);
    if dir.exists() {
        eprintln!("❌ Directory '{}' already exists", name);
        std::process::exit(1);
    }

    std::fs::create_dir_all(dir).expect("Failed to create directory");

    let example = format!(r#"# My first Slang animation!
# Created with: slang new {}

scene "Hello, Slang!"

set background to dark blue
set resolution to 1920x1080
set fps to 30

# Draw a circle and fade it in
draw a circle at center with radius 150 color cyan
fade in the circle over 1.5 seconds

wait 0.5 seconds

# Add some text
write "Hello, Slang!" at top color white size 64 over 1.5 seconds

wait 1 second

# Move the circle
move the circle to right over 1 second

wait 0.5 seconds

# Draw more shapes
draw a square at left with size 120 color yellow
fade in the square over 1 second

draw a triangle at center with size 100 color pink
fade in the triangle over 0.5 seconds

wait 1 second

# Change colors
change color of the circle to orange over 1 second
change color of the square to purple over 1 second

wait 1 second

# Fade everything out
fade out everything over 1.5 seconds
"#, name);

    let main_file = dir.join("main.sl");
    std::fs::write(&main_file, example).expect("Failed to write main.sl");

    println!("🚀 Created new Slang project: {}/", name);
    println!("   📄 {}/main.sl", name);
    println!();
    println!("   To render your animation:");
    println!("   $ slang render {}/main.sl", name);
    println!();
    println!("   To render and play:");
    println!("   $ slang run {}/main.sl", name);
}
