extern crate sdl2;
use sdl2::event::{Event, WindowEvent};

/// Any stuct implementing EventHandling can be used as a event handler. 
pub trait EventHandling {
    /// handle an event from the sdl2 lib. The components are passed so that 
    fn handle_event(&mut self, components: &mut foundry::ecs::component_table::ComponentTable, event: Event);
}

pub struct DefaultEventHandler {
    
}



impl DefaultEventHandler {
    pub fn new() -> DefaultEventHandler {
        DefaultEventHandler {
            
        }
    } 
}

impl EventHandling for DefaultEventHandler {
    fn handle_event(&mut self, components: &mut foundry::ecs::component_table::ComponentTable, event: Event) {
        match event {
            Event::Quit { .. } => println!("[GEAR ENGINE] -> close window required"),
            Event::Window { timestamp, window_id, win_event } => {
                match win_event {
                    WindowEvent::SizeChanged(sx, sy) => println!("[GEAR ENGINE] -> size changed {} {}", sx, sy),
                    // todo : get the camera components, resize any camera targetting the screen
                    _ => {},
                }
            }

            _ => {},
        }
    }
}