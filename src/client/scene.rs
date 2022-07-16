use super::super::utils::app::*;
use super::super::render::*;
use glium::{glutin::{*, self}, uniforms, Surface, uniform, Display};
pub struct World {
    Meshes: Vec<Mesh>,
    Camera: Camera,
    u_light: (f32, f32, f32)
}
impl World {
    pub fn new( Meshes: Vec<Mesh>,
        Camera: Camera, u_light: (f32, f32, f32)) -> Self {
            World { Meshes, Camera, u_light}
    }

    pub fn render(&mut self, screen: &glium::Display, cam: &camera::camera_mat, u_light: (f32, f32, f32)) {
        let mut target = screen.draw();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        target.clear_color_and_depth((0.5, 0.4, 1.0, 1.0), 1.0);
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        for mesh_object in &self.Meshes {
            let uni = uniform!{
                mod_matrix: mesh_object.mesh_transform.matrix,
                transform: mesh_object.translation_transform,
                view_matrix: cam.view_mat,
                pers_mat: cam.pers_mat,
                u_light: u_light,
                tex: &mesh_object.texture
            };
            target.draw(&mesh_object.vert_buffer, &indices,&mesh_object.program, &uni, &params).unwrap();
        }
        target.finish().unwrap();
    }
}
    
pub fn process_input(mut world : World, el: event_loop::EventLoop<()>,  screen: glium::Display) {
    app::update(el, move |events| {
        world.Camera.update();

        let camera_mat = camera_mat{ view_mat: world.Camera.view_matrix(), pers_mat: world.Camera.get_perspective() };

        world.render(&screen, &camera_mat, world.u_light);
        let mut action = Action::Continue;

        for event in events {
            match event {
                glutin::event::Event::DeviceEvent { event, .. } => match event {
                    ev => world.Camera.look_at(&ev)
                },
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => action = Action::Stop,
                    ev => world.Camera.input(&ev),
                },
                _ => (),
            }
        }
        action
    })
}