use super::super::utils::app::*;
use super::super::render::*;
use crate::{utils::{app::App as app}, object::*};

use glium::{glutin::{*, self}, Surface, uniform};
pub struct World {
    pub children: Vec<RigidBody>,
    pub camera: Camera,
    pub u_light: (f32, f32, f32)
}
impl World {
    pub fn new(children: Vec<RigidBody>,
        camera: Camera, u_light: (f32, f32, f32)) -> Self {
            World { children, camera, u_light}
    }

    pub fn update(&mut self, kids: Vec<RigidBody>) -> Self {
        return World { children:kids, camera: self.camera, u_light: self.u_light}
    }

    pub fn render(&mut self, screen: &glium::Display, cam: &camera::CameraMat, u_light: (f32, f32, f32)) {
        let mut target = screen.draw();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        target.clear_color_and_depth((0.2, 0.4, 1.0, 1.0), 1.0);
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