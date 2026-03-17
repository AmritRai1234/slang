use std::io::Write;
use std::process::{Command, Stdio};

/// Export rendered frames to an MP4 video using FFmpeg.
pub fn export_to_mp4(
    frames: &[Vec<u8>],
    width: u32,
    height: u32,
    fps: u32,
    output_path: &str,
    quality: &str,
) -> Result<(), ExportError> {
    if frames.is_empty() {
        return Err(ExportError("No frames to export".to_string()));
    }

    // Determine quality settings
    let (crf, preset) = match quality {
        "preview" => ("28", "ultrafast"),
        "hd" => ("18", "medium"),
        _ => ("23", "fast"),
    };

    // Check if ffmpeg is available
    let ffmpeg_check = Command::new("ffmpeg").arg("-version").output();
    if ffmpeg_check.is_err() {
        return Err(ExportError(
            "FFmpeg not found. Please install FFmpeg: https://ffmpeg.org/download.html".to_string(),
        ));
    }

    println!("  Exporting {} frames to {} ({}p, {} fps, quality: {})...",
        frames.len(), output_path, height, fps, quality
    );

    // Pipe raw RGBA frames to ffmpeg
    let mut child = Command::new("ffmpeg")
        .args([
            "-y",                          // overwrite output
            "-f", "rawvideo",
            "-pixel_format", "rgba",
            "-video_size", &format!("{}x{}", width, height),
            "-framerate", &fps.to_string(),
            "-i", "-",                     // read from stdin
            "-c:v", "libx264",
            "-pix_fmt", "yuv420p",
            "-crf", crf,
            "-preset", preset,
            "-movflags", "+faststart",
            output_path,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| ExportError(format!("Failed to start FFmpeg: {}", e)))?;

    let expected_frame_size = (width * height * 4) as usize;

    {
        let stdin = child.stdin.as_mut()
            .ok_or_else(|| ExportError("Failed to open FFmpeg stdin".to_string()))?;

        for (i, frame) in frames.iter().enumerate() {
            if frame.len() != expected_frame_size {
                return Err(ExportError(format!(
                    "Frame {} has wrong size: expected {}, got {}",
                    i, expected_frame_size, frame.len()
                )));
            }
            stdin.write_all(frame)
                .map_err(|e| ExportError(format!("Failed to write frame {}: {}", i, e)))?;

            // Progress indicator
            if (i + 1) % (frames.len() / 10).max(1) == 0 {
                let pct = ((i + 1) as f64 / frames.len() as f64 * 100.0) as u32;
                print!("\r  Progress: {}%", pct);
                std::io::stdout().flush().ok();
            }
        }
    }

    let output = child.wait_with_output()
        .map_err(|e| ExportError(format!("FFmpeg process error: {}", e)))?;

    println!(); // newline after progress

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ExportError(format!("FFmpeg failed: {}", stderr)));
    }

    println!("  ✅ Video saved to: {}", output_path);
    Ok(())
}

/// Export frames as individual PNG images (for debugging).
pub fn export_to_pngs(
    frames: &[Vec<u8>],
    width: u32,
    height: u32,
    output_dir: &str,
) -> Result<(), ExportError> {
    std::fs::create_dir_all(output_dir)
        .map_err(|e| ExportError(format!("Failed to create output dir: {}", e)))?;

    for (i, frame) in frames.iter().enumerate() {
        let path = format!("{}/frame_{:05}.png", output_dir, i);
        image::save_buffer(
            &path,
            frame,
            width,
            height,
            image::ColorType::Rgba8,
        ).map_err(|e| ExportError(format!("Failed to save frame {}: {}", i, e)))?;
    }

    println!("  ✅ Saved {} frames to {}/", frames.len(), output_dir);
    Ok(())
}

#[derive(Debug)]
pub struct ExportError(pub String);

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Export error: {}", self.0)
    }
}
