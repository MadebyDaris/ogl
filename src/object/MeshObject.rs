use std::{fs::File, io::Read};
use std::io::*;
use glium::texture::SrgbTexture2d;
use glium::{implement_vertex, vertex::VertexBufferAny, Display, Program};
use crate::utils::matrix::ModelMat;

#[derive(Clone, Copy)]
pub struct Vertex { pub(crate) position: [f32; 3], pub(crate) normal: [f32; 3], pub(crate) tex_coords: [f32; 2]}

implement_vertex!(Vertex, position, normal, tex_coords);

#[derive(Clone)]
pub struct MeshData {
    pub verts: Vec<Vertex> 
}

pub struct Mesh { 
    pub vert_buffer: VertexBufferAny,
    pub program: Program,
    pub mesh_transform: ModelMat,
    pub texture: SrgbTexture2d,
    pub translation_transform: [f32;3],
}

impl Mesh {
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
        let image = image::load(BufReader::new(File::open(path).unwrap()),
                        image::ImageFormat::Jpeg).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
        return texture;}
}


pub fn a4_2_a3(arr: [f32;4]) -> [f32;3] { 
    let mut x: [f32;3] = [0.;3];
    for i in 0..(x.len()) { 
        x[i] = arr[i] 
    } return x 
}