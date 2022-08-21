use crate::{output::OutputFunction, state::StateFunction};

pub struct StaticNeuron {
    pub z: f64,
    pub y: f64,
    pub weights: Vec<f64>,
    pub output_function: Box<dyn OutputFunction>,
    pub state_function: Box<StateFunction>,
}

impl StaticNeuron{
    pub fn new() {}

    pub fn calc(&mut self, inputs: Vec<f64>) {

        let f = &self.state_function;

        self.z = f((self.weights).clone(), inputs);

        self.y = self.output_function.call(self.z)
    }
}
