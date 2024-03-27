use rand::Rng; // Import the Rng trait to use random number generation

pub mod chrysanthemum;
pub mod goldenrod;

pub trait Flower {
    fn update_nectar(&mut self, temperature: f32);
    fn nectar_count(&self) -> u32;
    fn name(&self) -> &str;
    fn get_position(&self) -> (usize, usize);

    fn flower_emoji(&self) -> &str {
        "🌼"
    }
    fn give_nectar(&mut self) -> u32;
}

pub struct Flowers;
impl Flowers {
    pub fn build() -> Vec<Box<dyn Flower>> {
        let mut rng = rand::thread_rng();
        let mut flowers: Vec<Box<dyn Flower>> = Vec::new();
        for _ in 0..20 {
            let x = rng.gen_range(0..=9);
            let y = rng.gen_range(0..=9);
            flowers.push(Box::new(Goldenrod::new(0, x, y)));
        }

        for _ in 0..10 {
            let x = rng.gen_range(0..=9);
            let y = rng.gen_range(0..=9);
            flowers.push(Box::new(Chrysanthemum::new(0, x, y)));
        }

        flowers
    }
}

pub use chrysanthemum::Chrysanthemum;
pub use goldenrod::Goldenrod;
