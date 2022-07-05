use std::{fs::File, io::Read};
use std::io::*;

use glium::texture::SrgbTexture2d;
use glium::{implement_vertex, vertex::VertexBufferAny, VertexBuffer, Display, Program};
use image::{ImageBuffer, Rgba};
use wfobj::*;

#[derive(Clone, Copy)]
pub struct Vertex { position: [f32; 3], normal: [f32; 3], tex_coords: [f32; 2]} 
implement_vertex!(Vertex, position, normal, tex_coords);

pub struct Entity { 
    pub vert_buffer: VertexBufferAny,
    pub program: Program
}

impl Entity {
    pub fn new(screen: &Display, filename: &str, vertex_shader: &str, fragment_shader: &str) -> Self {
        // PARSING THE FILE
        let wrld = parse_file(filename).unwrap();

        // New
        let mut vertex_data = Vec::new();
        for f in wrld.faces {
            for v in f {
                let position = Entity::a4_2_a3(wrld.vertices[(v[0] as usize) - 1]);
                let normal = wrld.normals[(v[2] as usize) - 1];
                let tex_coords = wrld.textures[(v[1] as usize) - 1];
                let mut t_coords_2d = [0.;2];
                for i in 0..1 {
                    t_coords_2d[i] = tex_coords[i]};

                vertex_data.push(Vertex {
                    position,
                    normal,
                    tex_coords: t_coords_2d
                })
            }
        }
        let vert_buffer = VertexBuffer::new(screen, &vertex_data).unwrap().into();
        return Entity { vert_buffer, program: Entity::compile_program(screen, vertex_shader, fragment_shader) }
    }

    pub fn tex(display: &Display, path: &str) -> SrgbTexture2d {
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
    
    // Getting an Array of 3 items from an array of 4 items brute Force temporary method
    pub fn a4_2_a3(arr: [f32;4]) -> [f32;3] { 
        let mut x: [f32;3] = [0.;3];
        for i in 0..(x.len()) { 
            x[i] = arr[i] 
        } return x 
    }
}
    