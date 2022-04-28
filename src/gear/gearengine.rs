extern crate sdl2;
use sdl2::event::Event;
use super::objects::scene::SceneManager;
use super::eventmanager::{
    EventManager,
    DefaultEventManager,
};

pub struct GearEngine {
    scene_manager: SceneManager,
    event_manager: Box<dyn EventManager>,
}

impl GearEngine {
    pub fn new() -> GearEngine {
        GearEngine {
            scene_manager: SceneManager::new(),
            event_manager: Box::new(DefaultEventManager::new()),
        }
    }

    pub fn assign_event_manager<E: EventManager + 'static>(&mut self, new_event_manager: E) {
        self.event_manager = Box::new(new_event_manager);
    }

    pub fn handle_events(&mut self, event: &Event) {
        // events are given to the event handler and to the scene if it exists
        self.event_manager.handle_events(&event);

        match self.scene_manager.main_scene {
            Some(ref mut scene) => scene.handle_events(&event),
            None => { },
        }
    }

    pub fn call_world_update(&mut self) {
        match self.scene_manager.main_scene {
            Some(ref mut scene) => scene.update_scene(),
            None => { },
        }
    }

    pub fn render_scene(&mut self) {
        match self.scene_manager.main_scene {
            Some(ref mut scene) => scene.render_scene(),
            None => { },
        }
    }
}