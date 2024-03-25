
pub mod goldenrod;
pub mod chrysanthemum;

pub trait Flower {
    fn update_nectar(&mut self);
    fn nectar_count(&self) -> u32;
  }


pub use goldenrod::Goldenrod;
pub use chrysanthemum::Chrysanthemum;
