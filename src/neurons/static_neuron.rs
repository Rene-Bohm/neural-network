use std::clone;
use crate::{output::OutputFunction, state::StateFunction};
use nalgebra as alg;

pub struct StaticNeuron {
    pub z: f64,
    pub y: f64,
    pub weights: alg::DVector<f64>,
    pub output_function: Box<dyn OutputFunction>,
    pub state_function: Box<StateFunction>,
}

impl StaticNeuron{
    pub fn new() {}

    pub fn calc(&mut self, inputs: alg::DVector<f64>) {
        let f = &self.state_function;
        self.z = f((self.weights).clone(), inputs);

        self.y = self.output_function.call(self.z)
    }
}
