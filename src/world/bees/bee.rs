use uuid::Uuid;
use rand::Rng;

pub enum BeeStatus {
    Living,
    Dead,
    Sleeping,
}
pub struct Bee {
    hunger: u32,
    x: usize,
    y: usize,
    uuid: String,
    status: BeeStatus,
    age: u32,
}

impl Bee {
    pub fn new(hunger: u32, x: usize, y: usize) -> Self {
        let uuid = Uuid::new_v4().to_string();
        let status = BeeStatus::Living;
        let age = 0;

        Self {
            hunger,
            x,
            y,
            uuid,
            status,
            age
        }
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    pub fn get_status(&self) -> String {
        match self.status {
            BeeStatus::Living => "Living".to_string(),
            BeeStatus::Dead => "Dead".to_string(),
            BeeStatus::Sleeping => "Sleeping".to_string(),
        }
    }

    pub fn eat(&mut self, nectar_taken: u32) {
        self.hunger -= nectar_taken;
    }

    pub fn fly_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
        self.hunger += 1;
        if self.hunger > 100 {
            self.status = BeeStatus::Dead;
        }
    }

    pub fn hunger(&self) -> u32 {
        self.hunger
    }

    pub fn emoji(&self) -> char {
        '🐝'
    }

    pub fn sleep(&mut self) {
        self.status = BeeStatus::Sleeping;
    }

    pub fn wake_up(&mut self) {
        self.status = BeeStatus::Living;
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn increment_age(&mut self) {
        let mut rng = rand::thread_rng();

        self.age += 1;

        let death_chance = Self::calculate_death_chance(self.age);

        if rng.gen_bool(death_chance) {
            self.die();
        }
    }


    pub fn die(&mut self) {
        self.status = BeeStatus::Dead;
    }

    pub fn calculate_death_chance(age_in_hours: u32) -> f64 {
        let age_in_days = age_in_hours as f64 / 24.0;

        // Example: Start with a base death chance and increase it as the bee gets older
        let base_death_chance = 0.001; // 1% daily death chance
        let additional_risk = (age_in_days / 45.0) * 0.01; // Increase risk after 45 days

        (base_death_chance + additional_risk).min(0.1) // Cap the death chance to a maximum of 10%
    }

}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module

    #[test]
    fn bee_creation() {
        let bee = Bee::new(10, 5, 5);
        assert_eq!(bee.hunger(), 10);
        assert_eq!(bee.get_position(), (5, 5));
        assert_eq!(bee.get_uuid().len(), 36);
    }

    #[test]
    fn bee_eating_reduces_hunger() {
        let mut bee = Bee::new(10, 5, 5);
        bee.eat(5);
        assert_eq!(bee.hunger(), 5);
    }

    #[test]
    fn bee_flying_increases_hunger() {
        let mut bee = Bee::new(10, 5, 5);
        bee.fly_to(6, 6);
        assert_eq!(bee.hunger(), 11);
        assert_eq!(bee.get_position(), (6, 6));
    }

    #[test]
    fn bee_with_hunger_over_100_dies() {
        let mut bee = Bee::new(100, 5, 5);
        bee.fly_to(6, 6);
        assert_eq!(bee.get_status(), "Dead");
    }

    #[test]
    fn bee_has_correct_emoji() {
        let bee = Bee::new(10, 5, 5);
        assert_eq!(bee.emoji(), '🐝');
    }
}
