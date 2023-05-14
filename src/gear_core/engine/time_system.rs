use std::any::Any;

use foundry::{ComponentTable, Updatable};

pub struct GlobalTime {
    time: f32,
    delta: f32,
}

impl GlobalTime {
    pub fn new() -> GlobalTime {
        GlobalTime {
            time: 0.0,
            delta: 0.0,
        }
    }

    pub fn get_time(&self) -> f32 {
        self.time
    }

    pub fn get_delta(&self) -> f32 {
        self.delta
    }

    pub fn set_delta(&mut self, time: f32) {
        self.time = time;
    }

    pub fn add_delta_time(&mut self, delta: f32) {
        self.time+=delta;
        self.delta = delta;
    }
}

impl Updatable for GlobalTime {
    fn update(&mut self, components: &mut ComponentTable, delta: f32, user_data: &mut dyn Any) {
        self.time+=delta;
        self.delta = delta;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
