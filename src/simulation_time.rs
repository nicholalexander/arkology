pub struct SimulationTime {
  pub hour: u32, // Representing the hour of the day
}

impl SimulationTime {
  pub fn new() -> Self {
      Self { hour: 0 } // Start at midnight
  }

  pub fn advance(&mut self) {
      self.hour = (self.hour + 1) % 24; // Advance an hour and wrap around at 24
  }
}
