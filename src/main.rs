mod game_map;
mod game_tile;
mod player;

use crate::game_map::GameMap;
use crate::player::Player;

use std::io;

fn main() {
    let mut map = GameMap::new(30, 10);
    let mut player: Player = Player::new('@', (3, 3));
    map.initialize();
    map.render();
    map.alter_tile_at_coordinate(
        player.get_position().0,
        player.get_position().1,
        player.get_icon(),
    );
    clear_terminal();
    map.render();
}

fn clear_terminal() {
    // ANSI escape code to clear the screen
    print!("\x1B[2J\x1B[1;1H"); // This code clears the screen and positions the cursor at (1, 1)
}
