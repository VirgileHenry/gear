extern crate core;

pub use cgmath::{
    Deg,
    Euler,
    Matrix4, Matrix3, Matrix2,
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

