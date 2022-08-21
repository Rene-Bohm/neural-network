use std::fmt;

use crate::{output::OutputFunction, state::StateFunction};

pub struct StaticNeuron {
    pub n: f64,
    pub z: f64,
    pub y: f64,
    pub weights: Vec<f64>,
    pub output_function: Box<dyn OutputFunction>,
    pub state_function: Box<StateFunction>,
}

impl StaticNeuron{
    pub fn new() {}

    pub fn calc(&mut self, inputs: Vec<f64>) {

        //Calculate state of neuron and output 
        let f = &self.state_function;
        self.z = f((self.weights).clone(), inputs.clone());
        self.y = self.output_function.call(self.z);

        //Change weights 
        for i in 0..(self.weights).len(){

            self.weights[i] = self.weights[i] + (self.n * self.y * inputs[i]);

        }


    }
}

impl fmt::Display for StaticNeuron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(State: {},Last Output {}, current weights {:?})", self.z, self.y, self.weights)
    }
}
