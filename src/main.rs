// create the create tree for the compiler
mod window;
mod rendering;
mod objects;

fn main() {
    // initialize sdl lib
    let mut game_window: window::GameWindow = window::create_window();
    // start the main window loop
    window::main_loop(&mut game_window);
}
