extern crate foundry;
use foundry::{
    ecs::{
        world::World,
        system::System, entity::Entity,
    },  
};
use crate::gear_core::{
    gear_window::GameWindow,
    events::event_handling::EventHandling,
};
use std::time::{Instant, Duration};

use super::events::event_handling::{DefaultEventHandler};


pub struct Engine {
    world: World,
    main_timer: Duration,
    engine_state: EngineState,
}

#[derive(PartialEq)]
pub enum EngineState {
    Stopped,
    Running,
    RequestingStop,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            world: World::new(),
            main_timer: Duration::ZERO,
            engine_state: EngineState::Stopped,
        }
    }

    pub fn with_window(mut self, event_handler: Option<Box<dyn EventHandling>>) -> Engine {
        // create the window system and add it
        let mut event_handling_system = match event_handler {
            Some(handler) => handler,
            None => Box::new(DefaultEventHandler::new()),
        };
        let game_window = GameWindow::new(event_handling_system);
        let window_system = System::new(Box::new(game_window), foundry::ecs::system::UpdateFrequency::PerFrame);
        self.world.register_system(window_system, 0);
        
        self
    }

    pub fn main_loop(mut self) -> Engine {
        // set initial values
        self.main_timer = Duration::ZERO;

        let mut last_instant = Instant::now();
        while self.engine_state == EngineState::Running {
            // record last instant, keep track of time
            let delta = last_instant.elapsed();
            self.main_timer += delta;
            last_instant = Instant::now();

            // update the engine
            self.world.update(delta.as_secs_f32());
        }

        self
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }


}