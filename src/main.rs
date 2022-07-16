extern crate glium;

use std::f32::consts::PI;
use client::scene::{World, process_input};
use glium::{glutin::{self}, Surface, uniform};

mod render;
mod utils;
mod client;

use render::*;
use mesh::*;
use utils::{matrix::ModelMat, app};

use crate::render::mesh::Mesh;

fn main() {
    let ap = app::app::new();
    let mut _camera = Camera::new(&ap.screen);
    
    let mut model_matrix = ModelMat::new();
    model_matrix.scale(2.0, 2.0, 2.0);

    let translation = [2., 3., 1.];
    let t = [1., 4., 1.];
    let u_li = (0.1, 0.1, 0.1);
    
    let mut cube = mesh::Mesh::new_from_obj(model_matrix, t, &ap.screen ,"./data/obj/box.obj", "./data/tex/tex.jpg", "data/glsl/vertex_shader.glsl", "data/glsl/fragment_shader.glsl");
    let mut monkey = mesh::Mesh::new_from_obj(model_matrix, translation, &ap.screen ,"./data/obj/monkey.obj", "./data/tex/tex.jpg", "data/glsl/monkey/vertex_shader.glsl", "data/glsl/monkey/fragment_shader.glsl");
    
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let w = World::new(vec![monkey, cube], _camera, u_li);
    
    process_input(w, ap.event_loop, ap.screen);
}