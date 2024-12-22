pub extern crate nalgebra as na;

use na::Matrix3;

pub mod unconstrained_optimization;

pub fn run() {
    let decompose_matrix = Matrix3::new(1.0, 4.0, 5.0, 4.0, 18.0, 26.0, 3.0, 16.0, 30.0);
    let decomposed = decompose_matrix.cholesky().unwrap();
    println!("{:?}", decompose_matrix);
}
