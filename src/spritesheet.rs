use crate::palette::Color;
use crate::tile::Tile;

pub struct Spritesheet {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

const TILES_PER_ROW: usize = 16;
const TILE_SIZE: usize = 8;

pub fn assemble(tiles: &[Tile], palette: &[Color; 4], transparent_bg: bool) -> Spritesheet {
    if tiles.is_empty() {
        return Spritesheet {
            width: 0,
            height: 0,
            pixels: Vec::new(),
        };
    }

    let cols = TILES_PER_ROW;
    let rows = (tiles.len() + cols - 1) / cols;
    let width = (cols * TILE_SIZE) as u32;
    let height = (rows * TILE_SIZE) as u32;
    let mut pixels = vec![0u8; (width * height * 4) as usize];

    for (idx, tile) in tiles.iter().enumerate() {
        let tile_col = idx % cols;
        let tile_row = idx / cols;
        let base_x = tile_col * TILE_SIZE;
        let base_y = tile_row * TILE_SIZE;

        for y in 0..TILE_SIZE {
            for x in 0..TILE_SIZE {
                let px = base_x + x;
                let py = base_y + y;
                let offset = (py * width as usize + px) * 4;
                let color_idx = tile.pixels[y][x] as usize;
                let color = palette[color_idx];
                pixels[offset] = color[0];
                pixels[offset + 1] = color[1];
                pixels[offset + 2] = color[2];
                pixels[offset + 3] = if transparent_bg && color_idx == 0 {
                    0
                } else {
                    255
                };
            }
        }
    }

    Spritesheet {
        width,
        height,
        pixels,
    }
}
