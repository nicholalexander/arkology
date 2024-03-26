pub mod chrysanthemum;
pub mod goldenrod;

pub trait Flower {
    fn update_nectar(&mut self);
    fn nectar_count(&self) -> u32;
    fn name(&self) -> &str;
    fn get_position(&self) -> (usize, usize);
}

pub use chrysanthemum::Chrysanthemum;
pub use goldenrod::Goldenrod;
