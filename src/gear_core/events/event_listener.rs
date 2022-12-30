use foundry::ComponentTable;

use super::engine_events::{EngineEvents, ENGINE_EVENT_SIZE};


pub struct EventListener {
    pub listener: [Option<Box<dyn Fn(EngineEvents, &mut ComponentTable)>>; ENGINE_EVENT_SIZE]
}

impl EventListener {
    pub fn new() -> EventListener {
        EventListener { listener: [None; ENGINE_EVENT_SIZE] }
    }

    pub fn listen(&mut self, event_type: EngineEvents, callback: Box<dyn Fn(EngineEvents, &mut ComponentTable)>) {
        self.listener[event_type.id()] = Some(callback);
    }
}