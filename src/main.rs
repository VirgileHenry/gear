mod gear_core;
use foundry::create_entity;
use gear_core::engine::Engine;

use crate::gear_core::rendering::camera::CameraComponent;

fn main() {
    let mut engine = Engine::new() // creates the engine
        .with_window(None); // with a window

    let window_size = engine.get_window_size();

    let world_ref = engine.get_world();

    match window_size {
        None => {},
        Some((sx, sy)) => {
            let camera = create_entity!(world_ref.components; CameraComponent::new_perspective_camera(80.0, sx as f32 / sy as f32, 0.01, 100.0));
        }
    }

    engine.main_loop();

}
