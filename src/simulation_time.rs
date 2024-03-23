pub struct SimulationTime {
    pub hour: u32, // Representing the hour of the day
    pub day: u32,  // Representing the day of the year
}

impl SimulationTime {
    pub fn new() -> Self {
        Self { hour: 0, day: 0 }
    }

    pub fn advance(&mut self) {
        self.hour = (self.hour + 1) % 24; // Advance an hour and wrap around at 24

        if self.hour == 0 {
            self.day = (self.day + 1) % 365; // Advance a day and wrap around at 365
        }
    }
}
