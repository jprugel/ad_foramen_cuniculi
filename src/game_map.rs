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
            map.push(GameTile::new(x, y, '%'));
        }
        return Self {
            width,
            height,
            area,
            map,
        }
    }

    pub fn alter_tile_at_coordinate(&mut self, x: i32, y: i32, value: char) -> &mut Self {
        let index: usize = (y * self.width * x) as usize;
        self.map[index].set_value(value);
        self
    }

    pub fn alter_tile_at_index(&mut self, index: usize, value: char) -> &mut Self {
        self.map[index].set_value(value);
        self
    }
}
