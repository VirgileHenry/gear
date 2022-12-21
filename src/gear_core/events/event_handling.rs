use glfw::{WindowEvent};
use foundry::*;

use crate::gear_core::engine::EngineMessage;

/// Any stuct implementing EventHandling can be used as a event handler. 
pub trait EventHandling {
    /// handle an event from the sdl2 lib. The components are passed so that 
    fn handle_event(&mut self, components: &mut ComponentTable, event: WindowEvent, engine_message_callback: &mut EngineMessage);
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
    fn handle_event(&mut self, components: &mut ComponentTable, event: WindowEvent, engine_message_callback: &mut EngineMessage) {
        match event {
            WindowEvent::Close { .. } => *engine_message_callback = EngineMessage::StopEngine, // close the window require engine stop


            _ => {},
        }
    }
}