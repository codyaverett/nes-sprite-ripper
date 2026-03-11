use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use anyhow::{Context, Result};

use crate::spritesheet::Spritesheet;

pub fn write_png(path: &Path, sheet: &Spritesheet) -> Result<()> {
    let file = File::create(path)
        .with_context(|| format!("Failed to create output file: {}", path.display()))?;
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, sheet.width, sheet.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder
        .write_header()
        .context("Failed to write PNG header")?;

    writer
        .write_image_data(&sheet.pixels)
        .context("Failed to write PNG data")?;

    Ok(())
}
