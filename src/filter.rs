use std::ops::RangeInclusive;

use crate::tile::Tile;

pub struct TileFilter {
    pub skip_empty: bool,
    pub tile_range: Option<RangeInclusive<usize>>,
}

pub fn filter_tiles(tiles: Vec<Tile>, filter: &TileFilter) -> Vec<Tile> {
    let tiles = if let Some(range) = &filter.tile_range {
        tiles
            .into_iter()
            .enumerate()
            .filter(|(i, _)| range.contains(i))
            .map(|(_, t)| t)
            .collect()
    } else {
        tiles
    };

    if filter.skip_empty {
        tiles.into_iter().filter(|t| !t.is_empty()).collect()
    } else {
        tiles
    }
}
