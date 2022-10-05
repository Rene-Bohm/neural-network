use crate::{prelude::*, state};

pub struct ReluNetwork {
    pub layer: Vec<Layer>,
}

impl ReluNetwork {
    pub fn new(layer_components: Vec<u32>, learn_factor: f64, expected_input_size: u32) -> Self {
        let mut layer: Vec<Layer> = Vec::new();
        let mut input_size = expected_input_size;

        for layer_index in 0..layer_components.len() {
            layer.push(Layer::new_uniform(
                layer_components[layer_index],
                input_size,
                StateType::Scalar,
                OutputType::ReLU,
                0.0,
                learn_factor,
            ));

            input_size = layer_components[layer_index];
        }

        ReluNetwork { layer: layer }
    }

    pub fn call(&mut self, input: Vec<f64>) -> Vec<f64> {
        //Get number of layers
        let layer_index = self.layer.len();

        //output vector for each layer
        let mut current_output = input;

        for i in 0..layer_index {
            current_output = self.layer[i].call(current_output);
        }

        softmax(current_output)
    }

    //Todo Bias
    pub fn error_backpropagation(&mut self, input: Vec<f64>, correction: usize) {
        let forward_output = self.call(input.clone());

        let loss = softmax_cross_derivative(forward_output, correction);
    }

    pub fn visualize(&self) {
        for i in 0..self.layer.len() {
            println!("This is layer {}", i + 1);
            println!("This layer has {} neurons", self.layer[i].neurons.len());
            println!("----------------\n");

            self.layer[i].visualize();
        }
    }
}
