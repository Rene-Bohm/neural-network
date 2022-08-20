use crate::{output::OutputFunction, state::StateFunction};
use nalgebra as alg;


pub struct LeakyNeuron {
    pub z: f64,
    pub y: f64,
    pub output_function: Box<dyn OutputFunction>,
    pub state_function: Box<StateFunction>,
}
/* 
impl LeakyNeuron {
    pub fn new() {}

    pub fn calc(&mut self, weights: alg::DVector<f64>, inputs: alg::DVector<f64>) {
        let f = &self.state_function;
        self.z = f(weights, inputs);

        self.y = self.output_function.call(self.z)
    }
}
*/