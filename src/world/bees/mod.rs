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
}

pub use bee::Bee;
