
extern crate sdl2;
extern crate gl;
extern crate foundry;
use foundry::iterate_over_component_mut;

use super::events::event_handling::{EventHandling, DefaultEventHandler};

pub struct GameWindow {
    sdl: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    gl_context: sdl2::video::GLContext, // move this in a open gl renderer ?
    event_handler: Box<dyn EventHandling>,
}

impl GameWindow {
    pub fn new(event_handler: Box<dyn EventHandling>) -> GameWindow {
        // initialize the window
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
    
        // open gl version and init
        let gl_attr = video_subsystem.gl_attr();
    
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);
    
        let window = video_subsystem
            .window("Gear Engine v0.1.0", 900, 700) // name and default size
            .opengl() // opengl flag so we can use opengl
            .resizable() // able to resize the window
            .build() // build the WindowBuilder into a window
            .unwrap();
        // create the event listener
        let event_pump = sdl.event_pump().unwrap();
        // create the open gl context
        let gl_context = window.gl_create_context().unwrap();
        // create the bindings to the gpu to allow rust to call methods on it
        let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    
        // TEMP - set background color after
        unsafe {
            gl::Viewport(0, 0, 900, 700); // set viewport
            gl::ClearColor(0.741, 0.670, 0.854, 1.0);
        }
    
        return GameWindow {
            sdl: sdl,
            video_subsystem: video_subsystem,
            window: window,
            event_pump: event_pump,
            gl_context: gl_context,
            event_handler: event_handler,
        }
    }
}

impl foundry::ecs::system::Updatable for GameWindow {
    fn update(&mut self, components: &mut foundry::ecs::component_table::ComponentTable, delta: f32) {
        unsafe {
            // clear the window
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // poll events
        for event in self.event_pump.poll_iter() {
            self.event_handler.handle_event(components, event);
        }

        // render the ecs in our context

        // swap the rendered buffer with the one we just draw on
        self.window.gl_swap_window();
    }
}