pub use camera::CameraComponent;
pub use geometry::*;
pub use lighting::*;
pub use material::{
    Material,
    material_presets::*,
    MaterialProperties,
    texture::*,
};
pub use opengl::*;
pub use renderer::{
    DefaultOpenGlRenderer,
    Renderer,
};
pub use shaders::{
    ShaderProgram,
    shaders_files::*,
    ShaderSource,
};
pub use shaders::*;

pub(crate) mod camera;
pub(crate) mod shaders;
pub(crate) mod renderer;
pub(crate) mod material;
pub(crate) mod geometry;
pub(crate) mod opengl;
pub(crate) mod lighting;

// todo : y'a pas un meilleur endroit ?

