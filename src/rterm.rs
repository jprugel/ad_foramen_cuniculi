pub fn clear_terminal() {
    // ANSI escape code to clear the screen
    print!("\x1B[2J\x1B[1;1H"); // This code clears the screen and positions the cursor at (1, 1)
}
