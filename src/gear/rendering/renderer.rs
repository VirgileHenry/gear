

pub struct Renderer {

}
/*
impl Renderer {
    fn render(&self) {
        // render the world !

        // first, get a reference to all meshes
        let mut all_scene_render_objects: Vec<RenderObject> = Vec::new();

        for object in scene.objects.iter() {
            /*
            match object.to_render_objects() {
                RenderType::None => {},
                RenderType::Simple(render_object) => all_scene_render_objects.push(render_object),
            }
            */
        }

        // we managed to get all the scene meshes, but how do we set the unfiforms ? TODO

        // simple loop to check it works
        for render_object in all_scene_render_objects.iter() {
            
            render_object.mesh.use_shader_program();

            unsafe {
                Renderer::set_uniform_matrix4(render_object.world_tf, CString::new("modelWorldPos").unwrap(), render_object.mesh.get_shader_prog())
            }

            //scene.camera.set_camera_uniform(render_object.mesh.get_shader_prog());

            render_object.mesh.draw();
        }
    
        // scene.render(&camera);
    }

    unsafe fn set_uniform_matrix4(matrix: &cgmath::Matrix4::<f32>, loc_name: CString, shader_program: &ShaderProgram) {
        // admit correct shader program is in use
        // set the mat4 uniform at loc_name
        use cgmath::Matrix; // to use as_ptr() on the matrix
        let loc = gl::GetUniformLocation(
            shader_program.id(),
            loc_name.as_ptr() as *const gl::types::GLbyte
        );
        gl::UniformMatrix4fv(
            loc, // the data itself
            1 as gl::types::GLsizei, // the -number of element-
            gl::FALSE,
            matrix.as_ptr() as *const gl::types::GLfloat
        );
    }
}

*/
