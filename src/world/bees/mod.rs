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

    pub fn move_all_bees(bees: &mut Vec<Bee>) {
        for bee in bees.iter_mut() {
            let current_position = bee.get_position();

            //randomly move up or down or left or right by one square unless they are on the edge
            let mut x = current_position.0;
            let mut y = current_position.1;
            let mut rng = rand::thread_rng();
            let direction = rng.gen_range(0..=3);
            match direction {
                0 => {
                    if x > 0 {
                        x -= 1;
                    }
                }
                1 => {
                    if x < 9 {
                        x += 1;
                    }
                }
                2 => {
                    if y > 0 {
                        y -= 1;
                    }
                }
                3 => {
                    if y < 9 {
                        y += 1;
                    }
                }
                _ => {}
            }
            bee.move_position(x, y);
        }
    }
}

pub use bee::Bee;
