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
    fn update_nectar(&mut self, _temperature: f32) {}

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

#[cfg(test)]
mod flower_tests {
    use super::*;

    #[test]
    fn blueberry_creation() {
        let blueberry = Blueberry::new(10, 1, 2);
        assert_eq!(blueberry.nectar_count(), 10);
        assert_eq!(blueberry.get_position(), (1, 2));
        assert_eq!(blueberry.name(), "Blueberry");
        assert_eq!(blueberry.flower_emoji(), "🫐");
    }

    // This tests the implementation of `update_nectar`. Since it does nothing,
    // ensure the nectar count remains the same regardless of temperature.
    #[test]
    fn blueberry_nectar_update_does_nothing() {
        let mut blueberry = Blueberry::new(10, 1, 2);
        blueberry.update_nectar(15.0);
        assert_eq!(blueberry.nectar_count(), 10);
    }

    // Test to ensure `give_nectar` behaves as expected. Currently, it always returns 0.
    #[test]
    fn blueberry_give_nectar() {
        let mut blueberry = Blueberry::new(10, 1, 2);
        assert_eq!(blueberry.give_nectar(), 0);
        // Even after giving nectar, the count should remain the same since give_nectar always returns 0.
        assert_eq!(blueberry.nectar_count(), 10);
    }

    // Add more tests for other behavior or edge cases...
}
