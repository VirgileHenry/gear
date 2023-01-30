use foundry::{ComponentTable, EntityRef};

use super::engine_events::{EngineEvents, EngineEventTypes, ENGINE_EVENT_SIZE};


pub struct EventListener {
    pub listener: [Option<Box<dyn Fn(EngineEvents, EntityRef, &mut ComponentTable)>>; ENGINE_EVENT_SIZE]
}

impl EventListener {
    pub fn new() -> EventListener {
        EventListener { listener: Default::default() }
    }

    pub fn listen(&mut self, event_type: EngineEventTypes, callback: Box<dyn Fn(EngineEvents, EntityRef, &mut ComponentTable)>) {
        self.listener[event_type.id()] = Some(callback);
    }
}

