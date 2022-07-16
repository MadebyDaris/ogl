use std::borrow::Borrow;
use std::{fs::File, io::Read};
use std::io::*;

use glium::uniforms::{Uniforms, UniformsStorage};
use glium::{DrawError, uniforms, Surface, Frame, uniform};
use glium::program::Uniform;
use glium::texture::SrgbTexture2d;
use glium::{implement_vertex, vertex::VertexBufferAny, VertexBuffer, Display, Program};
use wfobj::*;

use crate::utils::matrix::ModelMat;

use super::camera::Camera;

#[derive(Clone, Copy)]
pub struct Vertex { position: [f32; 3], normal: [f32; 3], tex_coords: [f32; 2]} 
implement_vertex!(Vertex, position, normal, tex_coords);

pub struct ObjectData {
    vertex_data: Vec<Vertex>,
}

pub struct Mesh { 
    pub vert_buffer: VertexBufferAny,
    pub program: Program,
    pub mesh_transform: ModelMat,
    pub texture: SrgbTexture2d,
    pub translation_transform: [f32;3]
}

impl Mesh {
    pub fn new(model_matrix: ModelMat, transform: [f32;3], screen: &Display, object_data: ObjectData, 
        vertex_shader: &str, fragment_shader: &str, tex_filename: &str) -> Self{
            let vert_buffer = VertexBuffer::new(screen, &object_data.vertex_data).unwrap().into();
            let tex = Mesh::texture(screen, tex_filename);
            return Mesh { 
                vert_buffer, 
                program: Mesh::compile_program(screen, vertex_shader, fragment_shader),
                mesh_transform: model_matrix,
                texture: tex, 
                translation_transform: transform}
    }

    pub fn new_from_obj(model_matrix: ModelMat, transform: [f32;3], screen: &Display, filename: &str, 
        tex_filename: &str, vertex_shader: &str, fragment_shader: &str) -> Self {
            // PARSING THE FILE
            print!("{}", filename);
            let wrld = parse_file(filename).unwrap();
            let (v, n, t) = (wrld.vertices, wrld.normals, wrld.textures);
            let tex = Mesh::texture(screen, tex_filename);

            // New
            let mut vertex_data = Vec::new();
            for face in wrld.faces {
                for vert in face {
                // POSITION
                    let mut x: [f32;3] = [0.;3];
                    for i in 0..2 { let index_v = (vert[0] as usize) - 1;
                        x[i] = v[index_v][i]; }
                    let position = a4_2_a3(v[vert[0] as usize - 1]);
                // NORMALS
                    let normal = n[(vert[2]) as usize - 1];
                // TEXTURES
                    let mut tex_coords = [0.;2];
                    for i in 0..1 {
                        tex_coords[i] = t[(vert[1] as usize - 1)][i]};

                    vertex_data.push(Vertex {
                        position,
                        normal,
                        tex_coords: tex_coords,
                    })
                }
            }
            let vert_buffer = VertexBuffer::new(screen, &vertex_data).unwrap().into();

            return Mesh { 
                vert_buffer, 
                program: Mesh::compile_program(screen, vertex_shader, fragment_shader),
                mesh_transform: model_matrix, texture: tex,
                translation_transform: transform
            }
        }

    pub fn texture(display: &Display, path: &str) -> SrgbTexture2d {
        let image = image::load(BufReader::new(File::open(path).unwrap()),
                        image::ImageFormat::Jpeg).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
        return texture;
    }

    pub fn compile_program(screen: &Display, vertex_shader: &str, fragment_shader: &str) -> Program {
        let mut f_vrtx = File::open(vertex_shader).unwrap();
        let mut f_frgm = File::open(fragment_shader).unwrap();
        let (mut v_buffer, mut f_buffer) = (String::new(), String::new());
        let _vertex_shader_src = f_vrtx.read_to_string(&mut v_buffer).unwrap();
        let _fragment_shader_src = f_frgm.read_to_string(&mut f_buffer).unwrap();
    
        let program = Program::from_source(screen, &v_buffer, &f_buffer, None).unwrap();
        return program
    }
}
pub fn a4_2_a3(arr: [f32;4]) -> [f32;3] { 
    let mut x: [f32;3] = [0.;3];
    for i in 0..(x.len()) { 
        x[i] = arr[i] 
    } return x 
}