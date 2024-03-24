#[derive(Clone)]
pub struct Goldenrod {
  pub nectar: u32,
}

impl Goldenrod {
  pub fn new(nectar: u32) -> Self {
    Self { nectar }
  }

  pub fn update_nectar(&mut self) {
    if self.nectar < 100 {
      self.nectar += 1;
    }
  }
}
