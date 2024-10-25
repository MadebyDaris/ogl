extern crate glium;

use crate::simulation::*;

mod simulation;
mod render;
mod utils;
mod physics;
pub mod mesh; 

fn main() {
    example::example()
}