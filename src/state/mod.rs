use nalgebra as alg;

mod scalar_prod;
pub use scalar_prod::*;

// pub trait StateFunction {
//     fn calc(&self, weight: alg::DVector<f64>, input: alg::DVector<f64>) -> f64;
// }

/// (weights, inputs) -> state
pub type StateFunction = dyn Fn(alg::DVector<f64>, alg::DVector<f64>) -> f64;

pub struct EuklidDist {}

pub struct MaxDist {}

pub struct MinDist {}

pub struct ManhattanDist {}

pub struct MahalanobisDist {}
