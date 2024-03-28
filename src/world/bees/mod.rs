use crate::world::flowers::Flower;
use rand::Rng;

pub mod bee;

pub struct Bees;
impl Bees {
    pub fn build() -> Vec<Bee> {
        let mut rng = rand::thread_rng();
        let mut bees: Vec<Bee> = Vec::new();
        for _ in 0..5 {
            let x = rng.gen_range(0..=9);
            let y = rng.gen_range(0..=9);
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

            // Check for a flower with nectar at the current location before deciding to move
            if let Some(flower) = flowers.iter_mut().find(|f| {
                let pos = f.get_position();
                pos.0 == x && pos.1 == y
            }) {
                if flower.nectar_count() > 0 && bee.hunger() > 10 {
                    let nectar_taken = flower.give_nectar();
                    bee.eat(nectar_taken);
                } else {
                    // If there's no nectar or bee isn't hungry enough, consider moving
                    Self::move_bee(bee, &mut rng);
                }
            } else {
                Self::move_bee(bee, &mut rng);
            }
        }
    }

    fn move_bee(bee: &mut Bee, rng: &mut impl Rng) {
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
}

pub use bee::Bee;
