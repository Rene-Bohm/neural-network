use std::fmt;

use crate::{output::OutputFunction, state::StateFunction};

#[derive(Clone)]
pub struct StaticNeuron {

    pub n: f64,
    pub z: f64,
    pub y: f64,
    pub bias: f64,
    pub input: Vec<f64>,
    pub weights: Vec<f64>,
    pub output_function: Box<dyn OutputFunction>,
    pub state_function: Box<StateFunction>,
}

impl StaticNeuron {
    pub fn new() {}

    pub fn calc(&mut self, inputs: Vec<f64>) {
        self.input = inputs.clone();

        //Calculate state of neuron and output with bias
        let f = &self.state_function;
        self.z = f((self.weights).clone(), inputs.clone()) + self.bias;
        self.y = self.output_function.call(self.z);
    }

    pub fn change_weight(&mut self, delta: Vec<f64>) {
        for i in 0..self.weights.len() {
            self.weights[i] = self.weights[i] + delta[i];
        }
    }

    pub fn get_weights(&self) -> Vec<f64> {
        self.weights.clone()
    }
    
}

impl fmt::Display for StaticNeuron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(State: {},Last Output {}, current weights {:?})",
            self.z, self.y, self.weights
        )
    }
}
