
pub const ENGINE_EVENT_SIZE: usize = 2;

#[derive(Clone)]
pub enum EngineEvents {
    WindowEvent(glfw::WindowEvent),
    MousePosEvent(f64, f64),
}

// all the events of the engine. 
impl EngineEvents {
    pub fn id(&self) -> usize {
        match &self {
            Self::WindowEvent(_) => 0,
            Self::MousePosEvent(_, _) => 1,
        }
    }
}

// a way to identify the events by type
#[derive(Clone)]
pub enum EngineEventTypes {
    WindowEvent,
    MousePosEvent,
}

impl EngineEventTypes {
    pub fn id(&self) -> usize {
        match &self {
            Self::WindowEvent => 0,
            Self::MousePosEvent => 1,
        }
    }
}