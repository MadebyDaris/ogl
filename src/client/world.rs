use super::super::utils::app::*;
use super::super::render::*;
use crate::{utils::{app::App as app}, object::*};

use glium::{glutin::{*, self}, Surface, uniform};
pub struct World {
    children: Vec<RigidBody>,
    camera: Camera,
    u_light: (f32, f32, f32)
}
impl World {
    pub fn new( children: Vec<RigidBody>,
        camera: Camera, u_light: (f32, f32, f32)) -> Self {
            World { children, camera, u_light}
    }

    pub fn render(&mut self, screen: &glium::Display, cam: &camera::CameraMat, u_light: (f32, f32, f32)) {
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
        for mesh_object in &self.children {
            let uni = uniform!{
                mod_matrix: mesh_object.mesh.mesh_transform.matrix,
                transform: mesh_object.mesh.translation_transform,
                view_matrix: cam.view_mat,
                pers_mat: cam.pers_mat,
                u_light: u_light,
                tex: &mesh_object.mesh.texture
            };
            target.draw(&mesh_object.mesh.vert_buffer, &indices,&mesh_object.mesh.program, &uni, &params).unwrap();
        }
        target.finish().unwrap();
    }
}

pub fn process_input(mut world : World, el: event_loop::EventLoop<()>,  screen: glium::Display) {
    let camera_mat = CameraMat{ view_mat: world.camera.view_matrix(), pers_mat: world.camera.get_perspective() };
        app::update(el, move |events| {
        world.camera.update();
        world.render(&screen, &camera_mat, world.u_light);
        let mut action = Action::Continue;

        for event in events {
            match event {
                glutin::event::Event::DeviceEvent { event, .. } => match event {
                    ev => world.camera.look_at(&ev)
                },
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => action = Action::Stop,
                    ev => world.camera.input(&ev),
                },
                _ => (),
            }
        }
        action
    })
}