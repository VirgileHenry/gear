use std::collections::BTreeMap;

use foundry::{iterate_over_component_mut};
use glfw::{WindowEvent, Action};

use crate::{EventListener, EngineEvents, ButtonState, EngineEventTypes};
use crate::{UITransform, Button};


pub fn ui_event_manager() -> EventListener {
    let mut result = EventListener::new();

    // create a callback on window events to trigger ui elements
    result.listen(EngineEventTypes::WindowEvent, Box::new(|event, entity, components| {
        match event {
            EngineEvents::WindowEvent(window_event) => {
                match window_event {
                    // clicks can trigger buttons !
                    WindowEvent::MouseButton(mouse_button, action, modifiers) => {
                        // only register clicks on the left mouse button
                        if mouse_button == glfw::MouseButtonLeft {
                            let mut on_selected_cb = BTreeMap::new();
                            let mut callbacks = BTreeMap::new();
                            for (entity, button) in iterate_over_component_mut!(components; EntityRef; Button) {
                                match action {
                                    Action::Press => {
                                        match button.state() {
                                            ButtonState::Hovered => {
                                                button.set_state(ButtonState::Pressed);
                                                on_selected_cb.insert(entity, (button.on_selected.take(), true));
                                            },
                                            _ => {},
                                        }
                                    }
                                    Action::Release => {
                                        match button.state() {
                                            ButtonState::PressedEscaped => {
                                                button.set_state(ButtonState::Idle);
                                            }
                                            ButtonState::Pressed => {
                                                button.set_state(ButtonState::Hovered);
                                                on_selected_cb.insert(entity, (button.on_selected.take(), false));
                                                callbacks.insert(entity, button.callback.take());
                                            }
                                            _ => {},
                                        }
                                    }
                                    _ => {},
                                }
                            }
                            for (entity, (on_press_callback, entering)) in on_selected_cb.iter() {
                                match &on_press_callback {
                                    Some(cb) => cb(components, *entity, *entering),
                                    _ => {},
                                }
                            }
                            for (entity, callback) in callbacks.iter() {
                                match &callback {
                                    Some(cb) => cb(components, *entity, modifiers),
                                    _ => {},
                                }
                            }
                            for (entity, button) in iterate_over_component_mut!(components; EntityRef; Button) {
                                match on_selected_cb.get_mut(&entity) {
                                    Some(cb) => button.on_selected = cb.0.take(),
                                    _ =>{},
                                }
                                match callbacks.get_mut(&entity) {
                                    Some(cb) => button.callback = cb.take(),
                                    _ =>{},
                                }
                            } 
                        }
                    }
                    _ => {},
                }
            },
            _ => {}
        }
    }));

    result.listen(EngineEventTypes::MousePosEvent, Box::new(|event, entity, components| {
        match event {
            EngineEvents::MousePosEvent(x, y) => {
                // buttons
                // the bool identify which callback type it is : true is on_enter
                let mut on_hover_cb = BTreeMap::new();
                let mut on_selected_cb = BTreeMap::new();
                for (entity, ui_transform, button) in iterate_over_component_mut!(components; EntityRef; UITransform, Button) {
                    // check the state of the button and position of the mouse relative to the button
                    if ui_transform.contains_point(cgmath::Vector2::new(x, y)) {
                        match button.state() {
                            ButtonState::Idle => {
                                button.set_state(ButtonState::Hovered);
                                on_hover_cb.insert(entity, (button.on_enter.take(), true));
                            },
                            ButtonState::PressedEscaped => {
                                button.set_state(ButtonState::Pressed);
                                on_hover_cb.insert(entity, (button.on_enter.take(), true));
                                on_selected_cb.insert(entity, (button.on_selected.take(), true));
                            },
                            _ => {},
                        }
                    }
                    else {
                        match button.state() {
                            ButtonState::Hovered => {
                                button.set_state(ButtonState::Idle);
                                on_hover_cb.insert(entity, (button.on_enter.take(), false));
                            }
                            ButtonState::Pressed => {
                                button.set_state(ButtonState::PressedEscaped);
                                on_hover_cb.insert(entity, (button.on_enter.take(), false));
                                on_selected_cb.insert(entity, (button.on_selected.take(), false));
                            }
                            _ => {},
                        }
                    }
                }

                // call the listener callbacks ! 
                for (entity, (on_hover, entering)) in on_hover_cb.iter() {
                    match &on_hover {
                        Some(cb) => cb(components, *entity, *entering),
                        _ => {},
                    }
                }
                for (entity, (on_selected, selecting)) in on_selected_cb.iter() {
                    match &on_selected {
                        Some(cb) => cb(components, *entity, *selecting),
                        _ => {},
                    }
                }

                // put them back
                for (entity, button) in iterate_over_component_mut!(components; EntityRef; Button) {
                    match on_hover_cb.get_mut(&entity) {
                        Some(cb) => button.on_enter = cb.0.take(),
                        _ => {},
                    }
                    match on_selected_cb.get_mut(&entity) {
                        Some(cb) => button.on_selected = cb.0.take(),
                        _ => {},
                    }
                }

            }
            _ => {},
        }
    }));


    result
}