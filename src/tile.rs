pub struct Tile {
    pub pixels: [[u8; 8]; 8],
}

impl Tile {
    pub fn is_empty(&self) -> bool {
        let first = self.pixels[0][0];
        self.pixels
            .iter()
            .all(|row| row.iter().all(|&p| p == first))
    }
}

pub fn decode_tiles(chr_rom: &[u8]) -> Vec<Tile> {
    chr_rom
        .chunks_exact(16)
        .map(|chunk| {
            let plane0 = &chunk[0..8];
            let plane1 = &chunk[8..16];
            let mut pixels = [[0u8; 8]; 8];
            for y in 0..8 {
                for x in 0..8 {
                    let bit0 = (plane0[y] >> (7 - x)) & 1;
                    let bit1 = (plane1[y] >> (7 - x)) & 1;
                    pixels[y][x] = (bit1 << 1) | bit0;
                }
            }
            Tile { pixels }
        })
        .collect()
}
