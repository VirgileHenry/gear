use foundry::iterate_over_component;
use crate::gear_core::{
    rendering::{
        mesh::MeshRenderer,
        camera::CameraComponent,
    },
    geometry::transform::Transform,
};


/// R is the renderer itself
pub trait Renderer {
    fn render(&self, components: &mut foundry::ecs::component_table::ComponentTable);
}

pub struct DefaultOpenGlRenderer {
     // move this in a open gl renderer ?
}

impl DefaultOpenGlRenderer {
    pub fn new() -> DefaultOpenGlRenderer {
        DefaultOpenGlRenderer {

        }
    }
}

impl Renderer for DefaultOpenGlRenderer {
    fn render(&self, components: &mut foundry::ecs::component_table::ComponentTable) {
        // found main camera
        for camera in iterate_over_component!(&components; CameraComponent) {
            if !camera.is_main() { continue; } // check we have the main camera
            for (transform, mesh) in iterate_over_component!(&components; Transform, MeshRenderer) {
                // todo !
                // this link explain basic rendering :
                // https://stackoverflow.com/questions/39923583/use-different-shader-programs-in-opengl
                println!("rendering given mesh with position !");
            }
            break; // render only once in case there are multiple main camera component
        }
    }
}

