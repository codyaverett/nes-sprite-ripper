mod export;
mod filter;
mod palette;
mod rom;
mod spritesheet;
mod tile;
mod tui;

use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::Parser;

use filter::TileFilter;

#[derive(Parser)]
#[command(name = "nes-sprite-ripper")]
#[command(about = "Extract sprite tiles from NES ROM files as PNG spritesheets")]
struct Cli {
    /// Path to the .nes ROM file
    rom: PathBuf,

    /// Output PNG file path
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Show interactive TUI preview before exporting
    #[arg(long)]
    preview: bool,

    /// Make background color (palette index 0) transparent
    #[arg(long)]
    transparent_bg: bool,

    /// Skip empty/blank tiles (all pixels same value)
    #[arg(long)]
    skip_empty: bool,

    /// Select tile range, e.g., 0-255 (inclusive)
    #[arg(long)]
    tiles: Option<String>,
}

fn parse_tile_range(s: &str) -> Result<std::ops::RangeInclusive<usize>> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 2 {
        bail!("Invalid tile range format. Use START-END, e.g., 0-255");
    }
    let start: usize = parts[0].parse().map_err(|_| anyhow::anyhow!("Invalid range start"))?;
    let end: usize = parts[1].parse().map_err(|_| anyhow::anyhow!("Invalid range end"))?;
    Ok(start..=end)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let nes_rom = rom::parse_rom(&cli.rom)?;
    eprintln!(
        "ROM loaded: PRG={} KB, CHR={} KB",
        nes_rom.header.prg_rom_size / 1024,
        nes_rom.header.chr_rom_size / 1024
    );

    let tiles = tile::decode_tiles(&nes_rom.chr_rom);
    eprintln!("Decoded {} tiles", tiles.len());

    let tile_range = cli.tiles.as_deref().map(parse_tile_range).transpose()?;
    let filter = TileFilter {
        skip_empty: cli.skip_empty,
        tile_range,
    };
    let tiles = filter::filter_tiles(tiles, &filter);
    eprintln!("After filtering: {} tiles", tiles.len());

    if tiles.is_empty() {
        bail!("No tiles remaining after filtering");
    }

    let pal = &palette::DEFAULT_PALETTE;

    if cli.preview {
        let should_export = tui::run_tui(&tiles, pal)?;
        if !should_export {
            eprintln!("Preview closed without exporting.");
            return Ok(());
        }
    }

    let sheet = spritesheet::assemble(&tiles, pal, cli.transparent_bg);
    eprintln!(
        "Spritesheet: {}x{} pixels",
        sheet.width, sheet.height
    );

    let output_path = cli.output.unwrap_or_else(|| {
        let stem = cli.rom.file_stem().unwrap_or_default().to_string_lossy();
        PathBuf::from(format!("{}_sprites.png", stem))
    });

    export::write_png(&output_path, &sheet)?;
    eprintln!("Saved to {}", output_path.display());

    Ok(())
}
