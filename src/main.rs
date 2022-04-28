// create the create tree for the compiler
mod gear;
use crate::gear::window;

fn main() {
    // initialize sdl lib
    let mut game_window: window::GameWindow = window::create_window();
    // start the main window loop
    game_window.main_loop();
}
