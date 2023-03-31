use crate::utils::matrix::ModelMat;
use super::super::utils::app::*;
use super::super::render::*;
use crate::{utils::{app::App as app}, object::*, client::*};

pub fn environment() {
    let ap = App::new();
    let mut _camera = Camera::new(&ap.screen);
    
    let mut model_matrix = ModelMat::new();
    model_matrix.scale(8.0, 8.0, 8.0);

    let translation = [2., 3., 1.];
    let t = [1., 4., 1.];
    let u_li = (0.1, 0.1, 0.1);
    
    let mut sphere_shaders = shader_data { tex_filename :  "./data/tex/tex.jpg".to_string(), vertex_shader : "data/glsl/vertex_shader.glsl".to_string(), fragment_shader : "data/glsl/fragment_shader.glsl".to_string()};
    let mut sphere = RigidBody::load_obj(model_matrix, t, &ap.screen ,"./data/obj/sphere.obj", sphere_shaders);

    let mut terrain_shader = shader_data { tex_filename :  "./data/tex/tex.jpg".to_string(), vertex_shader : "data/glsl/vertex_shader.glsl".to_string(), fragment_shader : "data/glsl/fragment_shader.glsl".to_string()};
    let mut terrain = RigidBody::load_obj(model_matrix, t, &ap.screen ,"./data/obj/ground.obj", terrain_shader);
    
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let mut children = vec![sphere, terrain];
    let w = world::World::new(children, _camera, u_li);

    let gltf_test = RigidBody::load_gltf(model_matrix, t, &ap.screen ,"./data/gltf/player.gltf", sphere_shader);
    
    world::process_input(w, ap.event_loop, ap.screen);  
    // {
    //     let a = sphere.collision;
    //     let b = &terrain.collision;
    //     if a.x.0 > b.x.0 { if a.x.0 < b.x.1{
    //         if a.y.0 > b.y.0 { if a.y.0 < b.y.1{
    //             if a.z.0 > b.z.0 { if a.z.0 < b.z.1{
    //                 sphere.mesh.mesh_transform.translate(0., 5., 0.);
    //             }}
    //         }}
    //     }}
    // }
}