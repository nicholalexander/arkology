use super::Flower;

#[derive(Clone)]
pub struct Blueberry {
    pub nectar: u32,
    pub x: usize,
    pub y: usize,
}

impl Blueberry {
    pub fn new(nectar: u32, x: usize, y: usize) -> Self {
        Self { nectar, x, y }
    }
}

impl Flower for Blueberry {
    fn update_nectar(&mut self, temperature: f32) {
        if temperature > 25.0 && self.nectar < 100 {
            self.nectar += 1;
        }
    }

    fn nectar_count(&self) -> u32 {
        self.nectar
    }

    fn name(&self) -> &str {
        "Blueberry"
    }

    fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn flower_emoji(&self) -> &str {
        "🫐"
    }

    fn give_nectar(&mut self) -> u32 {
        0
    }
}
