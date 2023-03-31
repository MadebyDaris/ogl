pub mod MeshObject; pub use MeshObject::*;
pub mod PhysicsObject; pub use PhysicsObject::*;
use gltf;

use glium::{VertexBuffer, Display};
use wfobj::*;
use crate::utils::matrix::ModelMat;

pub struct RigidBody {
    pub mesh: Mesh,
    pub collision: Bounding
}

pub struct shader_data {
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub tex_filename: String
} 

impl RigidBody {
    pub fn rendering_data(vertex_shader: String, fragment_shader: String, tex_filename: String) -> shader_data {
        return shader_data { vertex_shader, fragment_shader, tex_filename}
    }
    pub fn new(model_matrix: ModelMat, transform: [f32;3], screen: &Display, object_data: Vec<MeshObject::Vertex>, shader_data: shader_data) -> RigidBody {
        let (tex, vertex_s, fragment_s):(&str, &str, &str) = (shader_data.tex_filename.as_str(), shader_data.vertex_shader.as_str(), shader_data.fragment_shader.as_str());
        let tex = Mesh::texture(screen, &tex);
        let collision = box_collision_object(&object_data);
        // Get Mesh of Object
            let vert_buffer = VertexBuffer::new(screen, &object_data).unwrap().into();
            let mesh =  Mesh { 
                vert_buffer, 
                program: Mesh::compile_program(screen, &vertex_s, &fragment_s),
                mesh_transform: model_matrix,
                texture: tex, 
                translation_transform: transform,
                data: object_data};
            return RigidBody {
                mesh,
                collision
                }
            }

// 
//  GLTF incoperation coming soon
// 
    pub fn load_gltf(model_matrix: ModelMat, transform: [f32;3], screen: &Display, filename: &str, 
        shader_data: shader_data) {
        
        let (tex, vertex_s, fragment_s):(&str, &str, &str) = (shader_data.tex_filename.as_str(), shader_data.vertex_shader.as_str(), shader_data.fragment_shader.as_str());
        let gltf = gltf::Gltf::open(filename).unwrap();
        for mesh in gltf.meshes() {
            for prim in mesh.primitives() {
                println!("{:?}", prim)
            } 
        }
    }

// 
// OBJ
// 
    pub fn load_obj(model_matrix: ModelMat, transform: [f32;3], screen: &Display, filename: &str, shader_data: shader_data) -> RigidBody {
        let (tex, vertex_s, fragment_s):(&str, &str, &str) = (shader_data.tex_filename.as_str(), shader_data.vertex_shader.as_str(), shader_data.fragment_shader.as_str());
        // PARSING THE FILE
                let wrld = parse_file(filename).unwrap();
                let (v, n, t) = (wrld.vertices, wrld.normals, wrld.textures);
                let tex = Mesh::texture(screen, &tex);

                let mut vertex_data = Vec::new();
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
                        })
                    }
                }
                
        // Get Mesh
            let vert_buffer = VertexBuffer::new(screen, &vertex_data).unwrap().into();
            let collision = box_collision_object( &vertex_data);
            let mesh = Mesh { 
                vert_buffer, 
                program: Mesh::compile_program(screen, &vertex_s, &fragment_s),
                mesh_transform: model_matrix, texture: tex,
                translation_transform: transform,
                data: vertex_data,
            };
        // Get Collision Object
        
            return RigidBody {mesh, collision}
        }

        pub fn get_collision(object_data: &Vec<MeshObject::Vertex>) -> PhysicsObject::PhysicsBody{
            let collision = PhysicsBody::box_collision_object(&object_data);
            return collision;
        }
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
                collision.x.0 = i.position[0]
            }
            if i.position[2] > collision.z.1 {
                collision.x.1 = i.position[0]
            }
        }
        print!("x, {:?}", collision.x);
        print!("y, {:?}", collision.y);
        print!("z, {:?}", collision.z);

        return collision
    }