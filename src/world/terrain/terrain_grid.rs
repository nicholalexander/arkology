// src/world/terrain/tile_grid.rs

use super::terrain_tile::TerrainTile;
use crate::simulation_time::SimulationTime;

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

    pub fn update_temperature(&mut self, time: &SimulationTime) {
        let base_temperature_change = match time.hour {
            6..=17 => 2.0, // Temperature rises during the day
            _ => -2.0,     // Temperature falls at night
        };

        let seasonal_modifier = time.month.temperature_modifier();

        for row in self.tiles.iter_mut() {
            for tile in row {
                tile.temperature += base_temperature_change + seasonal_modifier;
            }
        }
    }
}
