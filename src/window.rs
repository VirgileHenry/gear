extern crate sdl2;
extern crate gl;
use crate::rendering;
use std::time::Instant;
use std::thread::sleep;

pub struct GameWindow {
    _sdl: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    _gl_context: sdl2::video::GLContext,
    pub game_context: rendering::renderer::GameContext,
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
        game_context: rendering::renderer::GameContext::initialize(),
    }
}

pub fn main_loop(window: &mut GameWindow) {
    
    let mut last_instant = Instant::now();
    
    'main: loop {
        for event in window.event_pump.poll_iter() {
            match event {
                // call event manager from here
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }
    
        // update game state
        let deltatime = (Instant::now() - last_instant).as_nanos() as f32 / 1_000_000_000.0; // delta time in seconds
        last_instant = Instant::now();

        window.game_context.update(deltatime);

        // println!("fps: {}", 1.0 / deltatime);

        // render window contents here
        unsafe {
            // clear the window
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // call the renderer
        window.game_context.render_game();

        // swap the rendered buffer with the one we just draw on
        window.window.gl_swap_window();

    }
}


