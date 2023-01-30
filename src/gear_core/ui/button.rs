
use foundry::{ComponentTable, EntityRef};
use glfw::Modifiers;

#[derive(Clone, Copy)]
pub enum ButtonState {
    /// The button is idle.
    Idle,
    /// The mouse is currently over the button
    Hovered,
    /// The button have been pushed
    Pressed,
    /// The button have been pressed and the mouse exited it without releasing
    PressedEscaped,
}

pub struct Button {
    state: ButtonState,
    pub on_enter: Option<Box<dyn Fn(&mut ComponentTable, EntityRef, bool)>>,
    pub on_selected: Option<Box<dyn Fn(&mut ComponentTable, EntityRef, bool)>>,
    pub callback: Option<Box<dyn Fn(&mut ComponentTable, EntityRef, Modifiers)>>,
}

impl Button {
    pub fn new() -> Button {
        Button {
            state: ButtonState::Idle,
            on_enter: None,
            on_selected: None,
            callback: None,
        }
    }

    pub fn state(&self) -> ButtonState {
        self.state
    }

    pub fn set_state(&mut self, new_state: ButtonState) {
        self.state = new_state;
    }
}