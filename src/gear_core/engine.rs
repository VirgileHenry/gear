use foundry::*;
use crate::gear_core::*;
use std::{time::{Instant, Duration}, any::Any};


pub struct Engine {
    world: World,
    main_timer: Duration,
    engine_state: EngineState,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            world: World::new(),
            main_timer: Duration::ZERO,
            engine_state: EngineState::Stopped,
        }
    }

    pub fn with_gl_window(mut self, event_handler: Option<Box<dyn EventHandling>>, renderer: Option<Box<dyn Renderer>>) -> Engine {
        // create the window system and add it
        match GlGameWindow::new(event_handler, renderer) {
            Ok(game_window) => {
                let window_system = System::new(Box::new(game_window), UpdateFrequency::PerFrame);
                self.world.register_system(window_system, 0);
            },
            Err(e) => println!("[GEAR ENGINE] => [GL WINDOW] => Unable to add a window to the engine : {:?}", e),
        };
        
        
        self
    }

    pub fn get_gl_window(&self) -> Option<&GlGameWindow> {
        match self.world.get_system(0) {
            Some(system) => (system.get_updatable() as &dyn Any).downcast_ref::<GlGameWindow>(),
            None => None,
        }
    }

    pub fn get_gl_window_mut(&mut self) -> Option<&mut GlGameWindow> {
        match self.world.get_system_mut(0) {
            Some(system) => system.get_updatable_mut().as_any_mut().downcast_mut::<GlGameWindow>(),
            None => None,
        }
    }

    pub fn main_loop(mut self) -> Engine {
        // set initial values
        self.main_timer = Duration::ZERO;
        self.engine_state = EngineState::Running;

        let mut last_instant = Instant::now();
        while self.engine_state == EngineState::Running {
            // record last instant, keep track of time
            let delta = last_instant.elapsed();
            self.main_timer += delta;
            last_instant = Instant::now();

            // update the engine
            let mut callback = EngineMessage::None;
            self.world.update(delta.as_secs_f32(), &mut callback);

            match callback {
                EngineMessage::None => {},
                _ => self.handle_message(callback),
            }
        }

        // end of main loop, state back to stopped
        self.engine_state = EngineState::Stopped;

        self
    }

    pub fn handle_message(&mut self, message: EngineMessage) {
        match message {
            EngineMessage::StopEngine => self.engine_state = EngineState::RequestingStop,

            _ => {}
        }
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }


}


#[derive(PartialEq)]
pub enum EngineState {
    Stopped,
    Running,
    RequestingStop,
}


pub enum EngineMessage {
    None,
    StopEngine,
}