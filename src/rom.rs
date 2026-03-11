use std::path::Path;

use anyhow::{Context, Result, bail};

pub struct INesHeader {
    pub prg_rom_size: usize,
    pub chr_rom_size: usize,
}

pub struct NesRom {
    pub header: INesHeader,
    pub chr_rom: Vec<u8>,
}

const INES_MAGIC: &[u8; 4] = b"NES\x1a";
const PRG_BANK_SIZE: usize = 16384;
const CHR_BANK_SIZE: usize = 8192;

pub fn parse_rom(path: &Path) -> Result<NesRom> {
    let data = std::fs::read(path)
        .with_context(|| format!("Failed to read ROM file: {}", path.display()))?;

    if data.len() < 16 {
        bail!("File too small to be a valid iNES ROM");
    }

    if &data[0..4] != INES_MAGIC {
        bail!("Not a valid iNES ROM (bad magic bytes)");
    }

    let prg_banks = data[4] as usize;
    let chr_banks = data[5] as usize;

    let prg_rom_size = prg_banks * PRG_BANK_SIZE;
    let chr_rom_size = chr_banks * CHR_BANK_SIZE;

    if chr_rom_size == 0 {
        bail!("ROM has no CHR ROM data (uses CHR RAM). No tiles to extract.");
    }

    let chr_start = 16 + prg_rom_size;
    let chr_end = chr_start + chr_rom_size;

    if data.len() < chr_end {
        bail!(
            "ROM file is truncated: expected {} bytes, got {}",
            chr_end,
            data.len()
        );
    }

    let chr_rom = data[chr_start..chr_end].to_vec();

    Ok(NesRom {
        header: INesHeader {
            prg_rom_size,
            chr_rom_size,
        },
        chr_rom,
    })
}
