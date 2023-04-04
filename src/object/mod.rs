pub mod MeshObject; pub use MeshObject::*;

use glium::{VertexBuffer};
use wfobj::*;
use crate::utils::{matrix::ModelMat};

#[derive(Clone, Copy)]
pub struct Bounding {  
    pub x: (f32, f32),
    pub y: (f32, f32),
    pub z: (f32, f32)}

pub struct ShaderData {
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub tex_filename: String,
    pub transform_data: ModelMat
}

pub fn load_obj_file(filename: &str) -> MeshData {
    // PARSING THE FILE
        let wrld = parse_file(filename).unwrap();
        let (v, n, t) = (wrld.vertices, wrld.normals, wrld.textures);

        let mut vertex_data: Vec<MeshObject::Vertex> = Vec::new();
        for face in wrld.faces {
            for vert in face { let mut x: [f32;3] = [0.;3];
            
                // POSITION
                for i in 0..2 { let index_v = (vert[0] as usize) - 1; x[i] = v[index_v][i]; }
                let position = a4_2_a3(v[vert[0] as usize - 1]);

                // NORMALS
                let normal = n[(vert[2]) as usize - 1];

                // TEXTURES
                let mut tex_coords = [0.;2];
                for i in 0..1 { tex_coords[i] = t[(vert[1] as usize - 1)][i]};

                vertex_data.push(Vertex {
                    position,
                    normal,
                    tex_coords,
                });
            }
        }
        return MeshData { verts: vertex_data};
}


pub fn box_collision_object(mesh: &Vec<Vertex>) -> Bounding {
    let mut collision = Bounding { x : (0.0,0.0), y : (0.0,0.0), z : (0.0,0.0) };
    for i in mesh {
    // X Axis
        if i.position[0] < collision.x.0 {
            collision.x.0 = i.position[0]
        }
        if i.position[0] > collision.x.1 {
            collision.x.1 = i.position[0]
        }
    
    // Y Axis
        if i.position[1] < collision.y.0 {
            collision.y.0 = i.position[1]
        }
        if i.position[1] > collision.y.1 {
            collision.y.1 = i.position[1]
        }

    // X Axis
        if i.position[2] < collision.z.0 {
            collision.z.0 = i.position[2]
        }
        if i.position[2] > collision.z.1 {
            collision.z.1 = i.position[2]
        }
    }
    // print!("x, {:?}", collision.x);
    // print!("y, {:?}", collision.y);
    // print!("z, {:?}", collision.z);

    return collision
}