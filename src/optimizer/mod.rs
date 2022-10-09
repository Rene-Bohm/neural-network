mod adagrad;
pub use adagrad::*;

mod adam;
pub use adam::*;

mod rmsprop;
pub use rmsprop::*;

mod stochastic;
pub use stochastic::*;

pub trait Optimizer{
    fn call(&self, state: f64) -> f64;
    fn derivative(&self, z: f64, y: f64) -> f64;
}