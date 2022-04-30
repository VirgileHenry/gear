extern crate sdl2;
extern crate gl;
use crate::gear::{
    gearengine::GearEngine,
    eventmanager::EventManager
};

pub struct GameWindow {
    _sdl: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    _gl_context: sdl2::video::GLContext,
    gear_engine: Option<GearEngine>,
}

pub fn create_window() -> GameWindow {
    // initialize the window
    let sdl_ = sdl2::init().unwrap();
    let video_subsystem_ = sdl_.video().unwrap();

    // open gl version and init
    let gl_attr = video_subsystem_.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window_ = video_subsystem_
        .window("Gear v0.0.1", 900, 700) // name and default size
        .opengl() // opengl flag so we can use opengl
        .resizable() // able to resize the window
        .build() // build the WindowBuilder into a window
        .unwrap();
    // create the event listener
    let event_pump_ = sdl_.event_pump().unwrap();
    // create the open gl context
    let gl_context_ = window_.gl_create_context().unwrap();
    // create the bindings to the gpu to allow rust to call methods on it
    let _gl = gl::load_with(|s| video_subsystem_.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // TEMP - set background color after
    unsafe {
        gl::Viewport(0, 0, 900, 700); // set viewport
        gl::ClearColor(0.741, 0.670, 0.854, 1.0);
    }

    return GameWindow {
        _sdl: sdl_,
        _video_subsystem: video_subsystem_,
        window: window_,
        event_pump: event_pump_,
        _gl_context: gl_context_,
        gear_engine: Some(GearEngine::new()),
    }
}

impl GameWindow {
    pub fn main_loop(&mut self) {
        
        'main: loop {

            unsafe {
                // clear the window
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            for event in self.event_pump.poll_iter() {
                match event {
                    // window events
                    // close the window
                    sdl2::event::Event::Quit {..} => break 'main,
                    // engine events
                    _ => {
                        match self.gear_engine {
                            Some(ref mut engine) => {
                                // handle events
                                engine.handle_events(&event);

                                // call update
                                //engine.call_world_update();
                            
                                // render engine to the window
                                //engine.render_scene();
                            }
                            None => {},
                        }
                    }
                }
            }

            // swap the rendered buffer with the one we just draw on
            self.window.gl_swap_window();

        }
    }

    pub fn use_event_manager<E: EventManager + 'static>(&mut self, new_event_manager: E) {
        match self.gear_engine {
            Some(ref mut engine) => {
                engine.assign_event_manager(new_event_manager);
            }
            None => {
                println!("WARNING -> Unable to assign event handler : no engine");
            }
        }
    }
}   


