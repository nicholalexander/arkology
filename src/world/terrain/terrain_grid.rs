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

    pub fn update_temperature(&mut self, time: &mut SimulationTime) {
        let temperature = time.calculate_hourly_temperature();

        for row in self.tiles.iter_mut() {
            for tile in row {
                tile.temperature = temperature;
            }
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&TerrainTile> {
        self.tiles.get(y)?.get(x)
    }
}
