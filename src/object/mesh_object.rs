use std::{fs::File, io::Read};
use std::io::*;
use glium::texture::SrgbTexture2d;
use glium::{implement_vertex, vertex::VertexBufferAny, Display, Program};
use crate::utils::matrix::ModelMat;

use crate::{object::*};

#[derive(Clone, Copy)]
pub struct Vertex { pub(crate) position: [f32; 3], pub(crate) normal: [f32; 3], pub(crate) tex_coords: [f32; 2]}


implement_vertex!(Vertex, position, normal, tex_coords);

#[derive(Clone)]
pub struct MeshData {
    pub verts: Vec<Vertex> 
}
pub struct ShaderData {
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub tex_filename: String,
}


#[derive(Clone, Copy)]
pub struct MeshUniforms {
    pub mod_matrix: [[f32;4];4],
    pub transform: [f32; 3],
    pub u_light: (f32, f32, f32),
}

pub struct Mesh { 
    pub vert_buffer: VertexBufferAny,
    pub program: Program,
    pub texture: SrgbTexture2d,
    pub translation_transform: [f32;3],
}

impl Mesh {

    // Using another function specified in "../mod.rs" we get the data from an obj this is to make the
    // process more broad 
    pub fn new(transform: [f32;3], screen: &Display, object_data: &Vec<mesh_object::Vertex>, shader_data: ShaderData) -> Mesh {
        let (tex, vertex_s, fragment_s):(&str, &str, &str) = (shader_data.tex_filename.as_str(), shader_data.vertex_shader.as_str(), shader_data.fragment_shader.as_str());
        let tex = Mesh::texture(screen, &tex);
        // Get Mesh of Object
            let vert_buffer = VertexBuffer::new(screen, object_data).unwrap().into();

            let mesh =  Mesh { 
                vert_buffer, 
                program: Mesh::compile_program(screen, &vertex_s, &fragment_s),
                texture: tex, 
                translation_transform: transform,
            };
            return mesh
    }

    // from Shader Strings outputs a program used by another function which draws the Mesh
    pub fn compile_program(screen: &Display, vertex_shader: &str, fragment_shader: &str) -> Program {
            let mut f_vrtx = File::open(vertex_shader).unwrap();
            let mut f_frgm = File::open(fragment_shader).unwrap();
            let (mut v_buffer, mut f_buffer) = (String::new(), String::new());
            let _vertex_shader_src = f_vrtx.read_to_string(&mut v_buffer).unwrap();
            let _fragment_shader_src = f_frgm.read_to_string(&mut f_buffer).unwrap();
        
            let program = Program::from_source(screen, &v_buffer, &f_buffer, None).unwrap();
            return program
        }

    pub fn texture(display: &Display, path: &str) -> SrgbTexture2d {
        let image = image::load(BufReader::new(File::open(path).unwrap()), image::ImageFormat::Jpeg).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
        return texture;
    }
}


pub fn a4_2_a3(arr: [f32;4]) -> [f32;3] { 
    let mut x: [f32;3] = [0.;3];
    for i in 0..(x.len()) { 
        x[i] = arr[i] 
    } return x 
}