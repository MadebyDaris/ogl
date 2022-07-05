extern crate glium;

mod object;
mod camera;
mod matrix_transform;
mod handler;

use glium::{glutin::{self}, Surface, uniform};

use crate::matrix_transform::AffineMatrix;
fn main() {
    println!("Hello, world!");
    let (el, wb, cb) = (
        glutin::event_loop::EventLoop::new(), 
        glutin::window::WindowBuilder::new().with_resizable(true),
        glutin::ContextBuilder::new());
    let screen = glium::Display::new(wb, cb, &el).unwrap();

    let cube = object::Entity::new(&screen, "./data/obj/torus.obj", "data/glsl/vertex_shader.glsl", "data/glsl/fragment_shader.glsl");
    let tex = object::Entity::tex(&screen, "./data/tex/tex.jpg");

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut _camera = camera::Camera::new(&screen);

    handler::handler(el, move |events| {
        _camera.update();
        let mut model_matrix = AffineMatrix::new();
        model_matrix.matrix = model_matrix.scale(4.0, 4.0, 4.0);
        
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
        
        let light = [-1.0, 0.4, 0.9f32];
        let uniform = uniform! { 
            matrix: model_matrix.matrix,
            pers_mat: _camera.get_perspective(),
            view_matrix: _camera.view_matrix(),
            u_light: light,
            tex: &tex
            };

        target.draw(&cube.vert_buffer, &indices, &cube.program, &uniform,
            &params).unwrap();
        target.finish().unwrap();

        let mut action = handler::Action::Continue;


        for event in events {
            match event {
                glutin::event::Event::DeviceEvent { event, .. } => match event {
                    ev => _camera.look_at(&ev)
                },
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => action = handler::Action::Stop,
                    ev => _camera.input(&ev),
                },
                _ => (),
            }
        }
        action
    });
}