#![allow(dead_code)]
use std::time::{Duration, Instant};
use glium::{self, Display};
use glium::vertex::VertexBufferAny;
use glium::glutin::event_loop::{EventLoop, ControlFlow};
use glium::glutin::event::{Event, StartCause};

#[path = "camera.rs"] pub mod camera;

pub enum Action {
    Stop,
    Continue,
}

pub fn handler<F>(event_loop: EventLoop<()>, mut callback: F) ->! where F: 'static + FnMut(&Vec<Event<'_, ()>>) -> Action {
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