use nalgebra as alg;

pub mod Scalar_Product;

pub mod Euklid_Distance;

pub mod Max_Distance;

pub mod Min_Distance;

pub mod Manhattan_Distance;

pub mod Mahalanobis_Distance;

// pub trait StateFunction {
//     fn calc(&self, weight: alg::DVector<f64>, input: alg::DVector<f64>) -> f64;
// }

/// (weights, inputs) -> state
pub type StateFunction = dyn Fn(alg::DVector<f64>, alg::DVector<f64>) -> f64;


