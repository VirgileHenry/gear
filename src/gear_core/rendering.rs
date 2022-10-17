pub(crate) mod camera;
pub(crate) mod shaders;
pub(crate) mod renderer;
pub(crate) mod material;
pub(crate) mod geometry;
pub(crate) mod opengl;
pub(crate) mod lighting;

pub use camera::CameraComponent;

pub use shaders::{
    shaders_files::*,
    ShaderProgram,
};

pub use renderer::{
    DefaultOpenGlRenderer,
    Renderer,
};

pub use material::{
    Material,
    material_presets::*,
};

pub use geometry::*;
pub use opengl::*;
pub use lighting::*;
