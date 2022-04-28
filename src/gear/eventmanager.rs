extern crate sdl2;
use sdl2::event::Event;

pub trait EventManager {
    fn handle_events(&mut self, events: &Event) {
        // general event handling for the window, not the game (go to full screen, resize)
    }
}

pub struct DefaultEventManager {

}

impl DefaultEventManager {
    pub fn new() -> DefaultEventManager {
        return DefaultEventManager {
            
        }
    }
}

impl EventManager for DefaultEventManager {
    fn handle_events(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
            } => {
                println!("Key pressed !");
            }
            _ => { },
        }
    }
}