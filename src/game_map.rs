use crate::game_tile::GameTile;

#[derive(Debug)]
pub struct GameMap {
    width: i32,
    height: i32,
    area: i32,
    map: Vec<GameTile>,
}
impl GameMap {
    pub fn new(width: i32, height: i32) -> Self {
        let area = width * height;
        let mut map = Vec::new();
        for index in 0..area {
            let x: i32 = index % width;
            let y: i32 = index / width;
            map.push(GameTile::new(x, y, '.'));
        }
        Self {
            width,
            height,
            area,
            map,
        }
    }

    pub fn alter_tile_at_coordinate(&mut self, x: i32, y: i32, value: char) -> &mut Self {
        let index: usize = (y * self.width + x) as usize;
        self.map[index].set_value(value);
        self
    }

    pub fn alter_tile_at_index(&mut self, index: usize, value: char) -> &mut Self {
        self.map[index].set_value(value);
        self
    }

    pub fn initialize(&mut self) {
        for (index, tile) in self.map.iter_mut().enumerate() {
            if index < self.width as usize {
                tile.set_value('#');
            }
            if (index + 1) % (self.width as usize) <= 1 {
                tile.set_value('#');
            }
            if index > (self.area - self.width) as usize {
                tile.set_value('#');
            }
        }
    }

    pub fn render(&self) {
        for (index, tile) in self.map.iter().enumerate() {
            let remainder = index % (self.width as usize);
            let target = (self.width as usize) - 1;
            if remainder == target {
                print!("{}", tile.get_value());
                println!();
            } else {
                print!("{}", tile.get_value());
            }
        }
    }
}
