pub struct Bee {
    hunger: u32,
    x: usize,
    y: usize,
}

impl Bee {
    pub fn new(hunger: u32, x: usize, y: usize) -> Self {
        Self { hunger, x, y }
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn eat(&mut self) {
        self.hunger -= 1;
    }

    pub fn fly_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
        self.hunger += 1;
    }

    pub fn hunger(&self) -> u32 {
        self.hunger
    }

    pub fn emoji(&self) -> char {
        '🐝'
    }
}
