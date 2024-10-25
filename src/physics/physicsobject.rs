use crate::mesh::*;

use super::{physicsworld::PhysicsWorld, position_euclidean, Vector};

pub struct AstralBody {
    pub mesh: MeshObject,
    pub velocity: (f32, f32, f32),
    pub acc: (f32, f32, f32),
    pub mass: f32,
    pub r: f32,
    pub universe: PhysicsWorld
}
impl PartialEq for AstralBody {
    fn eq(&self, other: &Self) -> bool {
        // Compare fields that define equality for your struct
        self.mass == other.mass && 
        self.velocity == other.velocity && 
        self.acc == other.acc
        // Add more fields as necessary
    }
}
impl AstralBody {
    pub fn update_velocity(&mut self, delta_time: f32) {
        // Update velocity using the formula: v = v0 + a * t
        self.velocity.0 += self.acc.0 * delta_time;
        self.velocity.1 += self.acc.1 * delta_time;
        self.velocity.2 += self.acc.2 * delta_time;   
    }
    pub fn update_geometry(&mut self, delta_time: f32) {
        self.mesh.translate(self.velocity.0*delta_time, self.velocity.1*delta_time, self.velocity.2*delta_time);
    }
    pub fn universal_gravitation_force(&self, body: &mut AstralBody, g: f32) -> Vector {
        let direction = position_euclidean(&body.mesh) - position_euclidean(&self.mesh);
        let mu: f32 = g * body.mass * self.mass.clone();
        let distance_squared = direction.0.powi(2) + direction.1.powi(2) + direction.2.powi(2);
        if distance_squared == 0.0 {
            return Vector(0.0, 0.0, 0.0); // Avoid division by zero
        }
        return (direction * mu)/distance_squared;
    }
}