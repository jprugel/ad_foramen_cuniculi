mod game_map;
mod game_tile;

use crate::game_map::GameMap;

fn main() {
    let mut map = GameMap::new(10, 10);
    map.initialize();
    map.render();
}

fn clear_terminal() {
    // ANSI escape code to clear the screen
    print!("\x1B[2J\x1B[1;1H"); // This code clears the screen and positions the cursor at (1, 1)

    // Rest of your program goes here
    println!("This is a cleared terminal screen!");
}
