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

    pub fn move_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn eat(&mut self, nectar: u32) {
        self.hunger -= nectar;
    }

    pub fn hunger(&self) -> u32 {
        self.hunger
    }

    pub fn emoji(&self) -> char {
        '🐝'
    }
}
