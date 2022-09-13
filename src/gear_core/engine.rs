extern crate foundry;
use foundry::{ecs::{
    world::World,
    system::System, entity::Entity,
    },  
    create_entity,
};
use crate::gear_core::{
    gear_window::GameWindow,
    events::event_handling::EventHandling,
    engine_state::{
        EngineStateComponent,
        EngineState,
    },
};
use std::time::{Instant, Duration};

use super::events::event_handling::{DefaultEventHandler};


pub struct Engine {
    world: World,
    system_entity: Entity,
    main_timer: Duration,
}

impl Engine {
    pub fn new() -> Engine {
        let mut world = World::new();
        let mut system_entity = create_entity!(world.components; EngineStateComponent{ 
            current_state: EngineState::Stopped,
            window_definition: None
        });
        Engine {
            world: world,
            system_entity: system_entity,
            main_timer: Duration::ZERO,
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

        // initializing the engine state to running
        match self.world.get_component_mut::<EngineStateComponent>(&self.system_entity) {
            Some(component) => component.current_state = EngineState::Running,
            None => println!("[GEAR ENGINE] -> Unable to start the engine : no engine state component on the system entity."), 
        }

        let mut last_instant = Instant::now();
        while match self.world.get_component::<EngineStateComponent>(&self.system_entity) {
            None => false, // missing the engine state on the system entity !
            Some(component) => component.current_state == EngineState::Running,
        } {
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

    pub fn get_window_size(&self) -> Option<(i32, i32)> {
        self.world.get_component::<EngineStateComponent>(&self.system_entity)?.window_definition
    }

}