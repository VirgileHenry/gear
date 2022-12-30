use std::collections::VecDeque;

use foundry::{ComponentTable, iterate_over_component_mut};

use super::{
    engine_events::EngineEvents,
    event_listener::EventListener,
};

pub trait EventCallable {
    fn send_event(&mut self, event: EngineEvents);
}

impl EventCallable for ComponentTable {
    fn send_event(&mut self, event: EngineEvents) {
        // create a vec of all the events callbacks.
        let mut callbacks = VecDeque::new();

        for (entity, event_listener) in iterate_over_component_mut!(self; EntityRef; EventListener) {
            // check if the listener is interested in this event (if it have a associated callback)
            match event_listener.listener.get_mut(event.id()) {
                Some(callback) => {
                    callbacks.push_back((entity, callback.take()));
                }
                _ => {}
            }
        }

        // call the event callbacks passing in the event and component table
        for callback in callbacks.iter_mut() {
            match &callback.1 {
                Some(cb) => {
                    cb(event.clone(), self);
                }
                _ => {} // should never happen 
            }
        }

        // put back all the callbacks
        for (entity, event_listener) in iterate_over_component_mut!(self; EntityRef; EventListener) {
            // check if the entity ref is in the callbacks. As the callbacks are sorted by entity ref, it's easy
            match callbacks.pop_front() {
                Some((id, callback)) => {
                    if id == entity {
                        // put the callback back
                        event_listener.listener[event.id()] = callback;
                    }
                    // otherwise, keep going
                }
                _ => {
                    // no more callbacks to put back
                    break;
                }
            }
        }
    }
}

