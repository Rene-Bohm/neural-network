use crate::prelude::*;

pub struct Network{

    pub layer: Vec<Layer>,

}

impl Network{

    pub fn new(input_layer: Vec<Layer>) -> Self{

        Network { layer: (input_layer) }    

    }

    pub fn new_uniform( 
        layer_components: Vec<(u32, StateType, OutputType)>,
        output_parameter: f64, 
        learn_factor: f64,
        expected_input_size: u32) 
        -> Self
    {
        
        let mut layer: Vec<Layer> = Vec::new();
        let mut input_size = expected_input_size;

        for layer_index in 0..layer_components.len(){

            layer.push(Layer::new_uniform(
                layer_components[layer_index].0, 
                input_size,
                layer_components[layer_index].1,
                layer_components[layer_index].2, 
                output_parameter, 
                learn_factor)
            );

            input_size = layer_components[layer_index].0;

        }

        Network { layer: layer }

    }

    pub fn call(&mut self, input: Vec<f64>) -> Vec<f64>{

        //Get number of layers
        let layer_index = self.layer.len();

        //output vector for each layer
        let mut current_output = input;
        
        for i in 0..layer_index {
            
            current_output = self.layer[i].call(current_output);

        } 

        current_output

    }

    pub fn visualize (&self) {

        for i in 0..self.layer.len(){

            println!("This is layer {}", i + 1);
            println!("This layer has {} neurons and a Bias {}", self.layer[i].neurons.len(), self.layer[i].bias);
            println!("----------------\n");

            self.layer[i].visualize();

        }

    }

}

