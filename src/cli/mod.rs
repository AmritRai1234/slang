use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "slang")]
#[command(about = "Slang — a plain-English animation language")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Render a .sl file and play the video
    Run {
        /// Path to the .sl file
        file: String,
        /// Output quality: preview or hd
        #[arg(short, long, default_value = "preview")]
        quality: String,
    },
    /// Render a .sl file to MP4
    Render {
        /// Path to the .sl file
        file: String,
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        /// Output quality: preview or hd
        #[arg(short, long, default_value = "preview")]
        quality: String,
        /// Export as individual PNG frames instead
        #[arg(long)]
        frames: bool,
    },
    /// Create a new Slang project
    New {
        /// Project name
        name: String,
    },
    /// Launch the split-screen IDE
    Ide {
        /// Optional .sl file to open
        file: Option<String>,
        /// Port to run on
        #[arg(short, long, default_value = "3333")]
        port: u16,
    },
}
