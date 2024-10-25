use crate::mesh::MeshObject;

pub mod physicsobject;
pub mod physicsworld;

use std::ops::{Add, AddAssign, Div, Mul, Sub};
pub struct Vector (f32, f32, f32);

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl Div for Vector {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vector(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl Div<f32> for Vector {
    type Output = Self;
    
    fn div(self, rhs: f32) -> Self::Output {
        Vector(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
impl Mul<f32> for Vector {
    type Output = Self;
    
    fn mul(self, rhs: f32) -> Self::Output {
        Vector(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl AddAssign for Vector {    
    fn add_assign(&mut self, rhs: Self){
        *self = Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Add for Vector {    
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

pub fn position_euclidean(body: &MeshObject) -> Vector {
    let matrix = body.uniforms.transform.matrix;
    return Vector(matrix[0][3],matrix[1][3],matrix[2][3]);
}