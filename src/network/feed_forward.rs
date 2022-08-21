use crate::{
    neurons::*};

pub struct Network{

    pub layer: Vec<Vec<StaticNeuron>>,

}

impl Network{

    pub fn new(input_layer: Vec<Vec<StaticNeuron>>) -> Self{

        Network { layer: (input_layer) }    

    }

    pub fn call(&mut self, input: Vec<f64>) -> Vec<f64>{

        //Get number of layers
        let layer_dimension = self.layer.len();

        //output vector for each layer
        let mut tmp_output = input;
        
        for i in 0..layer_dimension {
            
            let num_of_neurons = self.layer[i].len();
            let mut output_of_current_layer: Vec<f64> = Vec::new();
            
            for j in 0..num_of_neurons{
                
                self.layer[i][j].calc(tmp_output.clone());

                output_of_current_layer.push(self.layer[i][j].y); 

            }

            tmp_output = output_of_current_layer;

        } 

        tmp_output

    }

    pub fn visualize (&self) {

        for i in 0..self.layer.len(){

            println!("This is layer {}", i + 1);
            println!("----------------\n");

            for j in 0..self.layer[i].len(){

                println!(   "This is neuron {}. This is the current state {}"
                            , j + 1, self.layer[i][j]);

                println!("--------<>-------\n");

            }

        }

    }

}

