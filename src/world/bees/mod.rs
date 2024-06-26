use crate::simulation_time::Month;
use crate::world::flowers::Flower;
use rand::Rng;

pub mod bee;

pub struct Bees;
impl Bees {
    pub fn build() -> Vec<Bee> {
        // let mut rng = rand::thread_rng();
        let mut bees: Vec<Bee> = Vec::new();
        for _ in 0..3 {
            // let x = rng.gen_range(0..=9);
            // let y = rng.gen_range(0..=9);
            let x = 3;
            let y = 3;
            bees.push(Bee::new(0, x, y));
        }

        bees
    }

    pub fn move_all_bees(bees: &mut Vec<Bee>, flowers: &mut Vec<Box<dyn Flower>>) {
        let mut rng = rand::thread_rng();

        for bee in bees.iter_mut() {
            let current_position = bee.get_position();
            let x = current_position.0;
            let y = current_position.1;

            if bee.get_status() == "Sleeping" {
                continue;
            }
            // Check for a flower with nectar at the current location before deciding to move
            if let Some(flower) = flowers.iter_mut().find(|f| {
                let pos = f.get_position();
                pos.0 == x && pos.1 == y
            }) {
                if flower.nectar_count() > 0 && bee.hunger() > 10 {
                    let nectar_taken = flower.give_nectar();
                    if nectar_taken > 0 {
                        bee.eat(nectar_taken);
                    } else {
                        Self::move_bee(bee, &mut rng);
                    }
                } else {
                    Self::move_bee(bee, &mut rng);
                }
            } else {
                Self::move_bee(bee, &mut rng);
            }
        }

        let mut new_bees = Vec::new();

        for bee in bees.iter() {
            if bee.get_status() == "Sleeping" {
                continue;
            }

            let current_position = bee.get_position();
            let x = current_position.0;
            let y = current_position.1;

            if bees.iter().any(|b| {
                let pos = b.get_position();
                b.get_uuid() != bee.get_uuid()
                    && pos.0 == x
                    && pos.1 == y
                    && b.hunger() > 20
                    && bee.hunger() < 30
                    && b.get_status() == "Living"
            }) {
                // Note: We are not directly mutating `bees` here.
                new_bees.push(Bee::new(60, x, y));
            }
        }

        // Now, add the newly spawned bees to the main collection
        bees.extend(new_bees);
    }

    fn move_bee(bee: &mut Bee, rng: &mut impl Rng) {
        if bee.get_status() == "Sleeping" {
            return;
        }

        let current_position = bee.get_position();
        let mut x = current_position.0;
        let mut y = current_position.1;

        let direction = rng.gen_range(0..=3);
        match direction {
            0 if x > 0 => x -= 1,
            1 if x < 9 => x += 1,
            2 if y > 0 => y -= 1,
            3 if y < 9 => y += 1,
            _ => {}
        }

        bee.fly_to(x, y);
    }

    pub fn sweep_dead_bees(bees: &mut Vec<Bee>) {
        bees.retain(|bee| bee.get_status() == "Living" || bee.get_status() == "Sleeping");
    }

    pub fn sleeping_state(bees: &mut Vec<Bee>, month: &Month) {
        match month {
            Month::December | Month::January | Month::February => {
                for bee in bees.iter_mut() {
                    bee.sleep();
                }
            }
            _ => {
                for bee in bees.iter_mut() {
                    bee.wake_up();
                }
            }
        }
    }

    pub fn increment_age(bees: &mut Vec<Bee>) {
        for bee in bees.iter_mut() {
            bee.increment_age();
        }
    }
}


pub use bee::Bee;
