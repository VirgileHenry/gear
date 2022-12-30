
pub const ENGINE_EVENT_SIZE: usize = 1;

// T is the custom user data message enum
#[derive(Clone)]
pub enum EngineEvents {
    WindowEvent(glfw::WindowEvent),
}

impl EngineEvents {
    pub fn id(&self) -> usize {
        match &self {
            Self::WindowEvent(_) => 0,
        }
    }
}