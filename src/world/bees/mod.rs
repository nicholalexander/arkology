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

            // Randomly move up, down, left, or right by one square unless they are on the edge
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

            // Check for a flower with nectar at the intended location
            if let Some(flower) = flowers.iter_mut().find(|f| {
                let pos = f.get_position();
                pos.0 == x && pos.1 == y
            }) {
                if flower.nectar_count() > 0 && bee.hunger() > 10 {
                    // The bee eats some nectar from the flower, reducing its hunger.
                    // Adjust the amount of nectar taken based on your game's rules.
                    let nectar_taken = flower.give_nectar();
                    bee.eat(nectar_taken);
                    continue; // Skip moving the bee to this new location since it's eating
                }
            }

            // Move the bee only if it's not eating
            bee.fly_to(x, y);
        }
    }
}

pub use bee::Bee;
