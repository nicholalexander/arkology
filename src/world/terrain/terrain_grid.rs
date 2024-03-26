// src/world/terrain/tile_grid.rs

use super::terrain_tile::TerrainTile;

pub struct TerrainGrid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<TerrainTile>>,
}

impl TerrainGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![TerrainTile::new(0, 0.0); width]; height];
        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn update_temperature(&mut self, hour: u32) {
        let temperature_change = match hour {
            6..=17 => 2.0, // Temperature rises during the day
            _ => -2.0,     // Temperature falls at night
        };

        for row in self.tiles.iter_mut() {
            for tile in row {
                tile.temperature += temperature_change;
            }
        }
    }
}
