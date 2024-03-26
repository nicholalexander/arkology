pub mod chrysanthemum;
pub mod goldenrod;

pub trait Flower {
    fn update_nectar(&mut self, temperature: f32);
    fn nectar_count(&self) -> u32;
    fn name(&self) -> &str;
    fn get_position(&self) -> (usize, usize);
}

pub struct Flowers;

impl Flowers {
    pub fn build() -> Vec<Box<dyn Flower>> {
        let mut flowers: Vec<Box<dyn Flower>> = Vec::new();
        for _ in 0..5 {
            flowers.push(Box::new(Goldenrod::new(0, 0, 0)));
        }

        for _ in 0..5 {
            flowers.push(Box::new(Chrysanthemum::new(0, 0, 1)));
        }

        flowers
    }
}

pub use chrysanthemum::Chrysanthemum;
pub use goldenrod::Goldenrod;
