extern crate core;
#[macro_use]
extern crate lazy_static;

pub use cgmath::{
    Deg,
    Euler,
    Matrix2, Matrix3, Matrix4,
    Quaternion,
    Rad,
    Vector2, Vector3, Vector4,
};
pub use foundry::*;
pub use foundry;
pub use glfw::{
    Action,
    Key,
    WindowEvent,
};

pub use gear_core::*;
pub use modules::*;

mod gear_core;
mod modules;

