use super::Flower;

#[derive(Clone)]
pub struct Goldenrod {
    pub nectar: u32,
}

impl Goldenrod {
    pub fn new(nectar: u32) -> Self {
        Self { nectar }
    }
}

impl Flower for Goldenrod {
    fn update_nectar(&mut self) {
        if self.nectar < 100 {
            self.nectar += 2;
        }
    }

    fn nectar_count(&self) -> u32 {
      self.nectar
  }
}
