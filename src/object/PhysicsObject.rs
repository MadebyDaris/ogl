use crate::{object::*};

#[derive(Clone, Copy)]
pub struct Bounding {  
    pub x: (f32, f32),
    pub y: (f32, f32),
    pub z: (f32, f32)}

#[derive(Clone, Copy)]
pub struct PhysicsBody {
    pub BoundingBox: Bounding,
}


impl PhysicsBody {
    pub fn box_collision_object(mesh: &Vec<MeshObject::Vertex>) -> PhysicsBody {
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


        return PhysicsBody {
            BoundingBox: collision
        }
    }
}