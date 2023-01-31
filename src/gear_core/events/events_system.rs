use std::{
    collections::{BTreeMap}, 
};

use foundry::{ComponentTable, iterate_over_component_mut};

use crate::EngineMessage;

use super::{
    engine_events::EngineEvents,
    event_listener::EventListener,
};

pub trait EventCallable {
    fn send_event(&mut self, event: EngineEvents, engine_message: &mut EngineMessage);
}

impl EventCallable for ComponentTable {
    fn send_event(&mut self, event: EngineEvents, engine_message: &mut EngineMessage) {
        // create a map of all the events callbacks.
        let mut callbacks = BTreeMap::new();

        // take all callbacks from the component table that are interested in that event
        for (entity, event_listener) in iterate_over_component_mut!(self; EntityRef; EventListener) {
            // check if the listener is interested in this event (if it have a associated callback)
            match event_listener.listener.get_mut(event.id()) {
                Some(callback) => {callbacks.insert(entity, callback.take());},
                _ => {},
            }
        }

        // call the event callbacks passing in the event and component table
        for (entity, callback) in callbacks.iter_mut() {
            match &callback {
                Some(cb) => cb(event.clone(), *entity, self, engine_message),
                _ => {} // should never happen 
            }
        }

        // put back all the callbacks
        for (entity, event_listener) in iterate_over_component_mut!(self; EntityRef; EventListener) {
            // check if the entity ref is in the callbacks. As the callbacks are sorted by entity ref, it's easy
            match callbacks.get_mut(&entity) {
                Some(callback) => event_listener.listener[event.id()] = callback.take(),
                _ => {},
            }
        }
    }
}

