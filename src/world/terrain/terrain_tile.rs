#[derive(Clone)]
pub struct TerrainTile {
    pub elevation: u32,
    pub temperature: f32,
}

impl TerrainTile {
    pub fn new(elevation: u32, temperature: f32) -> Self {
        Self {
            elevation,
            temperature,
        }
    }
}
