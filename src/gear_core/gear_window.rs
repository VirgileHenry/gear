
extern crate sdl2;
extern crate gl;
extern crate foundry;
use std::any::Any;
use super::{
    events::event_handling::{
        EventHandling, 
        DefaultEventHandler
    }, engine::{
        EngineMessage,
        Engine
    },
    rendering::renderer::{
        Renderer,
        DefaultOpenGlRenderer
    },
};

pub struct GlGameWindow {
    sdl: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    gl_context: sdl2::video::GLContext,
    event_handler: Box<dyn EventHandling>,
    gl_renderer: Box<dyn Renderer>,
}

impl GlGameWindow {
    pub fn new(event_handler: Option<Box<dyn EventHandling>>, renderer: Option<Box<dyn Renderer>>) -> GlGameWindow {
        // initialize the window
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
    
        let window = video_subsystem
            .window("Gear Engine v0.1.0", 900, 700) // name and default size
            .opengl() // opengl flag so we can use opengl
            .resizable() // able to resize the window
            .build() // build the WindowBuilder into a window
            .unwrap();
        // create the event listener
        let event_pump = sdl.event_pump().unwrap();

        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        let mut event_handling_system = match event_handler {
            Some(handler) => handler,
            None => Box::new(DefaultEventHandler::new()),
        };

        let mut renderer_system = match renderer {
            Some(renderer) => renderer,
            None => Box::new(DefaultOpenGlRenderer::new()),
        };
    
        return GlGameWindow {
            sdl: sdl,
            video_subsystem: video_subsystem,
            window: window,
            event_pump: event_pump,
            gl_context: gl_context,
            event_handler: event_handling_system,
            gl_renderer: renderer_system,
        }
    }

}

impl foundry::ecs::system::Updatable for GlGameWindow {
    fn update(&mut self, components: &mut foundry::ecs::component_table::ComponentTable, _delta: f32, user_data: &mut dyn Any) {
        unsafe {
            // clear the window
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // poll events
        // get the engine message from the user data
        match user_data.downcast_mut::<EngineMessage>() {
            None => { // the user data was not an engine message : create a dummy callback to give to the event handler
                let mut dummy_callback = EngineMessage::None;
                for event in self.event_pump.poll_iter() {
                    self.event_handler.handle_event(components, event, &mut dummy_callback);
                }
            },
            Some(callback_message) => { // get the engine callback and pass it to the event handler
                for event in self.event_pump.poll_iter() {
                    self.event_handler.handle_event(components, event, callback_message);
                }
            },
        };


        // render the ecs in our context
        self.gl_renderer.render(components);

        // swap the rendered buffer with the one we just draw on
        self.window.gl_swap_window();
    }
}