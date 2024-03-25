
pub mod goldenrod;
pub mod chrysanthemum;

pub trait Flower {
    fn update_nectar(&mut self);
}

pub use goldenrod::Goldenrod;
pub use chrysanthemum::Chrysanthemum;
