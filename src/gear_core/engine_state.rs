#[derive(PartialEq)]
pub enum EngineState {
    Stopped,
    Running,
    RequestingStop,
}

/// This component is only on the system entity
/// It allow systems to give info to the game engine
pub struct EngineStateComponent {
    pub current_state: EngineState,
    pub window_definition: Option<(i32, i32)>,
}