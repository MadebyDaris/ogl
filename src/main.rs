extern crate glium;

mod object;
mod camera;
mod matrix_transform;

use glium::{glutin::{self, event::{self, Event, WindowEvent}}, Surface, uniform};

use crate::matrix_transform::affine_matrix;
fn main() {
    println!("Hello, world!");
    let (el, wb, cb) = (glutin::event_loop::EventLoop::new(), glutin::window::WindowBuilder::new().with_resizable(true),
    glutin::ContextBuilder::new());
    let screen = glium::Display::new(wb, cb, &el).unwrap();

    let cube = object::Entity::new(&screen, "./data/monkey.obj", "data/vertex_shader.glsl", "data/fragment_shader.glsl");
    
    // INDICES / FACES
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut t = 0.;
    el.run(move |event, _, controlflow| {
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *controlflow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, ..} => match event{
                WindowEvent::CloseRequested => {*controlflow = glutin::event_loop::ControlFlow::Exit; return;}
                _ => return
            },

            Event::NewEvents(cause) => match cause {
                event::StartCause::ResumeTimeReached { start: _, requested_resume: _ } => (),
                event::StartCause::Init => (),
                _ => return,
            }
            _ => ()
        }

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        let mut target = screen.draw();
        target.clear_color_and_depth((0.5, 0.4, 1.0, 1.0), 1.0);
        

        let base_matrix = affine_matrix::new();
        let matrix = base_matrix.scale(0.5, 0.5);

        let rot_mat = affine_matrix::new();
        let rot = rot_mat.rotate(180.0 + t/100.);
        
        let _camera = camera::Camera::new();
        t += 0.04;

        let light = [-1.0, 0.4, 0.9f32];
        let uniform = uniform! { matrix:matrix, rot_mat:rot, pers_mat: _camera.get_perspective(), u_light:light};
        target.draw(&cube.vert_buffer, &indices, &cube.program, &uniform,
            &params).unwrap();
        target.finish().unwrap()
    });
}
