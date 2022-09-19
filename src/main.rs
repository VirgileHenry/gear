mod gear_core;
use foundry::create_entity;
use gear_core::engine::Engine;

use crate::gear_core::rendering::camera::CameraComponent;

fn main() {
    let mut engine = Engine::new() // creates the engine
        .with_window(None, None); // with a window


    engine.main_loop();

}
