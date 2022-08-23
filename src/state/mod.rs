#![allow(clippy::non_snake_case)]

mod state_function;
pub use state_function::State;

mod scalar_product;
pub use scalar_product::Scalar;

mod euklid_distance;
pub use euklid_distance::Euklid;

mod maximum_distance;
pub use maximum_distance::Maximum;

mod minimum_distance;
pub use minimum_distance::Minimum;

pub mod manhattan_distance;
pub use manhattan_distance::Manhattan;

pub mod mahalanobis_distance; //todo

// pub trait StateFunction {
//     fn calc(&self, weight: alg::DVector<f64>, input: alg::DVector<f64>) -> f64;
// }

/// (weights, inputs) -> state
// pub type StateFunction = dyn Fn(Vec<f64>, Vec<f64>) -> f64;
pub type StateFunction = fn(Vec<f64>, Vec<f64>) -> f64;
