extern crate glium;
use std::time::{Instant, Duration};

use glium::glutin::{self, event::{Event, StartCause}, event_loop::{EventLoop, ControlFlow}};

pub enum Action {
    Stop,
    Continue,
}

pub struct App {
    pub screen: glium::Display,
    pub event_loop: EventLoop<()>,
    // pub window: glutin::window::Window,
}

impl App {
    pub fn new() -> Self {
        let el = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new().with_resizable(true);
        let cb = glutin::ContextBuilder::new().with_vsync(true);
        let displ = glium::Display::new(wb, cb, &el).unwrap();

        return App { screen : displ, event_loop: el}
    }

    pub fn update<F>(event_loop: EventLoop<()>, mut callback: F) ->! where F: 'static + FnMut(&Vec<Event<'_, ()>>) -> Action {
            let mut buffer = Vec::new();
            event_loop.run(move |event, _, controlflow| {
                let mut next_frame_time = std::time::Instant::now();
            
                let run_callback = match event.to_static() {
                    Some(Event::NewEvents(cause)) => {
                        match cause {
                            StartCause::ResumeTimeReached { .. } | StartCause::Init => {
                                true
                            },
                            _ => false
                        }
                    },
                    Some(event) => {
                        buffer.push(event);
                        false
                    }
                    None => {
                        false 
                    }
                };
        
                let action = if run_callback {
                    let action = callback(&buffer);
                    next_frame_time = Instant::now() + Duration::from_nanos(16666667);
        
                    buffer.clear();
                    action
                } else {
                    Action::Continue
                };
        
                match action {
                    Action::Continue => {
                        *controlflow = ControlFlow::WaitUntil(next_frame_time);
                    },
                    Action::Stop => *controlflow = ControlFlow::Exit
                }
            })
    }
}