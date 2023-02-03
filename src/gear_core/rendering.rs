pub(crate) mod camera;
pub(crate) mod shaders;
pub(crate) mod renderer;
pub(crate) mod material;
pub(crate) mod geometry;
pub(crate) mod opengl;
pub(crate) mod lighting;

pub use camera::CameraComponent; // todo : y'a pas un meilleur endroit ?

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
    MaterialProperties,
    material_presets::*,
    texture::*,
};

pub use geometry::*;
pub use opengl::*;
pub use lighting::*;
pub use shaders::*;
