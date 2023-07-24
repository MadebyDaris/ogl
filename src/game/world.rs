use super::super::render::*;
use crate::{object::mesh_object::{Mesh, MeshUniforms}};

use glium::{Surface, uniform};
pub struct World {
    pub children: Vec<Mesh>,
    pub camera: Camera,
    pub u_light: (f32, f32, f32)
}
impl World {
    pub fn new(children: Vec<Mesh>,
        camera: Camera, u_light: (f32, f32, f32)) -> Self {
            World { children, camera, u_light}
    }

    pub fn render(&mut self, object_render_data: Vec<MeshUniforms>, screen: &glium::Display, cam: &camera::CameraMat, u_light: (f32, f32, f32)) {
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
        
        for i in 0..self.children.len() {
            let mesh_object = &self.children[i];
            let mesh_uniform = object_render_data[i];

            let uni = uniform!{
                mod_matrix: mesh_uniform.mod_matrix,
                transform: mesh_uniform.transform,
                view_matrix: cam.view_mat,
                pers_mat: cam.pers_mat,
                u_light: u_light,
                tex: &mesh_object.texture
            };
            target.draw(&mesh_object.vert_buffer, &indices, &mesh_object.program, &uni, &params).unwrap();
        }
        target.finish().unwrap();
    }
}