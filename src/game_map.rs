#[derive(Debug)]
pub struct GameMap {
    width: i32,
    height: i32,
}
impl GameMap {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> i32 {
        self.width * self.height
    }

    pub fn to_vec(&self) -> Vec<char> {
        let vec_size = self.area();
        let mut result: Vec<char> = vec!['%'; vec_size as usize];
        for index in 0..result.len() as i32 {
            if index < self.width {
                result[index as usize] = '#';
            }
            if index >= (self.area() - self.width) {
                result[index as usize] = '#';
            }
            if (index + 1) % (self.width) == 0 || (index + 1) % (self.width) == 1 {
                result[index as usize] = '#';
            }
            result[index as usize] = '%'
        }
        result
    }

    pub fn print(&self) {
        let map = self.to_vec();
        for (index, value) in map.iter().enumerate() {
            if index == 0 {
                print!("{}", value);
            } else if (index + 1) % (self.width as usize) == 0 {
                println!("{}", value);
            } else {
                print!("{}", value);
            }
        }
    }
}
