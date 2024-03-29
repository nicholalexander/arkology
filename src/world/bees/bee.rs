use uuid::Uuid;

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
}

impl Bee {
    pub fn new(hunger: u32, x: usize, y: usize) -> Self {
        let uuid = Uuid::new_v4().to_string();
        let status = BeeStatus::Living;
        Self {
            hunger,
            x,
            y,
            uuid,
            status,
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
