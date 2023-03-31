extern crate glium;

use crate::{client::*, object::*, game::*};

mod game;
mod render;
mod utils;
mod client;
pub mod object; 

use render::*;
use utils::{matrix::ModelMat, app};
fn main() {
    experiment::environment()
    // let ap = app::App::new();
    // let mut _camera = Camera::new(&ap.screen);
    
    // let mut model_matrix = ModelMat::new();
    // model_matrix.scale(3.0, 3.0, 3.0);

    // let translation = [2., 3., 1.];
    // let t = [1., 4., 1.];
    // let u_li = (0.1, 0.1, 0.1);
    
    // let mut sphere_shaders = shader_data { tex_filename :  "./data/tex/tex.jpg".to_string(), vertex_shader : "data/glsl/vertex_shader.glsl".to_string(), fragment_shader : "data/glsl/fragment_shader.glsl".to_string()};
    // let mut sphere = RigidBody::load_obj(model_matrix, translation, &ap.screen ,"./data/obj/sphere.obj", sphere_shaders);

    // let mut terrain_shader = shader_data { tex_filename :  "./data/tex/tex.jpg".to_string(), vertex_shader : "data/glsl/vertex_shader.glsl".to_string(), fragment_shader : "data/glsl/fragment_shader.glsl".to_string()};
    // let mut terrain = RigidBody::load_obj(model_matrix, t, &ap.screen ,"./data/obj/ground.obj", terrain_shader);
    
    // // let mut terrain = terrain::Terrain::new(16, 5., 3, 1.5, 0.5,).get_mesh(&ap.screen, 2., "./data/glsl/terrain/fragment_shader.glsl", "./data/glsl/terrain/vertex_shader.glsl", "./data/tex/tex.jpg");
    // let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    // let mut children = vec![sphere, terrain];
    // let w = world::World::new(children, _camera, u_li);

    // world::process_input(w, ap.event_loop, ap.screen);
}