#[derive(Debug)]
pub struct GameTile {
    x: i32,
    y: i32,
    pub value: char,
}

impl GameTile {
    pub fn new(x: i32, y: i32, value: char) -> Self {
        Self { x, y, value }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_value(&self) -> char {
        self.value
    }

    pub fn set_value(&mut self, value: char) -> &mut Self {
        self.value = value;
        self
    }
}
