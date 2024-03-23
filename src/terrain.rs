// Terrain is the structure intended to hold information about the grid of the world.
// Each cell of the grid will hold properties such as temperature, elevation, etc.

#[derive(Clone)]
pub struct TerrainTile {
  elevation: u32,
  temperature: f32,
  // Add more properties like moisture, vegetation type, etc.
}

pub struct TerrainGrid {
  width: usize,
  height: usize,
  tiles: Vec<Vec<TerrainTile>>,
}

impl TerrainGrid {
  pub fn new(width: usize, height: usize) -> TerrainGrid {
      let tiles = vec![vec![TerrainTile { elevation: 0, temperature: 0.0 }; width]; height];
      TerrainGrid { width, height, tiles }
  }

  // Method to update temperatures across the grid
  pub fn update_temperature(&mut self, hour: u32) {
      let temperature_change = match hour {
          6..=17 => 2.0, // Temperature rises during the day
          _ => -2.0, // Temperature falls at night
      };

      for row in self.tiles.iter_mut() {
          for tile in row {
              tile.temperature += temperature_change;
          }
      }
  }
}
