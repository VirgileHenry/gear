use std::{any::Any, sync::mpsc::Receiver};
use crate::gear_core::*;
use foundry::*;
use gl::Disable;
use glfw::{Context, InitError, Window, WindowEvent, Glfw, WindowHint};

pub struct GlGameWindow {
    glfw: Glfw,
    window: Window,
    mouse_pos: (f64, f64),
    events: Receiver<(f64, WindowEvent)>,
    gl_renderer: Box<dyn Renderer>,
}

#[derive(Debug)]
pub enum GlWindowError {
    GlfwInitError(InitError),
    GlfwWindowCreationError,
    GlContextInitError,
}

impl GlGameWindow {
    pub fn new(renderer: Option<Box<dyn Renderer>>) -> Result<GlGameWindow, GlWindowError>  {
        // initialize glfw and the window
        let mut glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
            Ok(glfw) => glfw,
            Err(e) => return Err(GlWindowError::GlfwInitError(e)) 
        };

        // todo : change this to make it user friendly
        let width = 900;
        let height = 600;
        let title = "Gear Engine V0.1.0";
        let mode = glfw::WindowMode::Windowed;
        // Antialiasing x4
        glfw.window_hint(WindowHint::Samples(Some(4)));

        let (mut window, events) = match glfw.create_window(width, height, title, mode) {
            Some(result) => result,
            None => return Err(GlWindowError::GlfwWindowCreationError),
        };

        window.make_current();
        // poll every thing for now
        // todo : better way to chose what to poll
        window.set_all_polling(true);
        
        //window.set_cursor_mode(glfw::CursorMode::Hidden);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let renderer_system = match renderer {
            Some(renderer) => renderer,
            None => Box::new(DefaultOpenGlRenderer::new()),
        };

        // some open gl flags
        unsafe {
            // face are one sided
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::FRONT);
            // enable depth tets
            gl::Enable(gl::DEPTH_TEST);
        }
    
        return Ok(GlGameWindow {
            glfw,
            window,
            mouse_pos: (0., 0.),
            events,
            gl_renderer: renderer_system,
        })
    }

    #[allow(dead_code)]
    pub fn get_renderer(&self) -> &Box<dyn Renderer> {
        &self.gl_renderer
    }

    pub fn set_new_renderer(&mut self, renderer: Box<dyn Renderer>) {
        self.gl_renderer = renderer;
    }

    pub fn aspect_ratio(&self) -> f32 {
        let (w, h) = self.window.get_size();
        (w as f32) / (h as f32)
    }

    pub fn default_event_handling(&self, components: &mut ComponentTable, event: glfw::WindowEvent, engine_message: &mut EngineMessage) {
        match event {
            glfw::WindowEvent::Close => {
                *engine_message = EngineMessage::StopEngine;
            }

            glfw::WindowEvent::Size(width, height) => {
                // window got resized
                // todo : resize camera view frustrum

                // recompute ui positions !
                for ui_transform in iterate_over_component_mut!(components; UITransform) {
                    ui_transform.recompute_screen_pos(width, height);
                }
            }

            _ => {}
        }
    }

    pub fn handle_gl_messages(&mut self, message: &GlWindowMessage) {
        match message {
            GlWindowMessage::SetCursorMode(mode) => self.window.set_cursor_mode(*mode),
            GlWindowMessage::SetFullScreen(fullscreen) => println!("going full screen : {fullscreen}"),
        }
    }

}





impl Updatable for GlGameWindow {
    fn update(&mut self, components: &mut ComponentTable, _delta: f32, user_data: &mut dyn Any) {
        unsafe {
            // clear the window
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // poll events
        // get the engine message from the user data
        match user_data.downcast_mut::<EngineMessage>() {
            None => { // the user data was not an engine message : create a dummy callback to give to the event handler
                let mut dummy_callback = EngineMessage::None;
                self.glfw.poll_events();
                for (_, event) in glfw::flush_messages(&self.events) {
                    self.default_event_handling(components, event.clone(), &mut dummy_callback);
                    components.send_event(EngineEvents::WindowEvent(event), &mut dummy_callback);
                }
                let mouse_pos = self.window.get_cursor_pos();
                if mouse_pos != self.mouse_pos && self.window.is_focused() {
                    self.mouse_pos = mouse_pos;
                    components.send_event(EngineEvents::MousePosEvent(mouse_pos.0, mouse_pos.1), &mut dummy_callback);
                }
                match dummy_callback {
                    EngineMessage::GlWindowMessage(message) => self.handle_gl_messages(&message),
                    _ => {},
                }
            },
            Some(callback_message) => { // get the engine callback and pass it to the event handler
                self.glfw.poll_events();
                for (_, event) in glfw::flush_messages(&self.events) {
                    self.default_event_handling(components, event.clone(), callback_message);
                    components.send_event(EngineEvents::WindowEvent(event), callback_message);
                }
                let mouse_pos = self.window.get_cursor_pos();
                if mouse_pos != self.mouse_pos && self.window.is_focused() {
                    self.mouse_pos = mouse_pos;
                    components.send_event(EngineEvents::MousePosEvent(mouse_pos.0, mouse_pos.1), callback_message);
                }
                match callback_message {
                    EngineMessage::GlWindowMessage(message) => self.handle_gl_messages(message),
                    _ => {},
                }
            },
        };

        // render the ecs in our context
        self.gl_renderer.render(components);

        // swap the rendered buffer with the one we just draw on
        self.window.swap_buffers();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    } 

}