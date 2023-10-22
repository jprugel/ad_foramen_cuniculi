pub struct Player {
    icon: char,
    position: (i32, i32),
}

impl Player {
    pub fn new(icon: char, position: (i32, i32)) -> Self {
        Self { icon, position }
    }
    pub fn get_icon(&self) -> char {
        self.icon
    }

    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }
}
