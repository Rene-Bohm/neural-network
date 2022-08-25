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

    //Todo Bias
    pub fn error_backpropagation(&mut self, correction: Vec<f64>){

        let mut nth_derivative: Vec<f64> = Vec::new();

        for i in (0..self.layer.len()).rev(){


            //checks if the last layer is currently processed or not
            if i == self.layer.len(){

                //calculate Delta of layer L for each neuron
                for j in 0..correction.len(){

                    let neuro = self.layer[i].neurons[j].clone();

                    nth_derivative.push(-(correction[i] - neuro.y) * neuro.output_function.derivative(neuro.z, neuro.y)); 

                }

                nth_derivative.reverse();

                //change weights of neurons in the last layer 
                for j in 0..self.layer[i].neurons.len(){

                    for k in 0..self.layer[i].neurons[j].input.len(){

                        let delta =  nth_derivative[j] * self.layer[i].neurons[j].input[k] * (-self.layer[i].neurons[j].n);
                        
                        self.layer[i].neurons[j].input[k] = self.layer[i].neurons[j].input[k] + delta;
                        
                    }

                }

            }else{

                let mut tmp_derivative: Vec<f64> = Vec::new();

                //calculate Delta of the i'th layer for each neuron
                for j in 0..self.layer[i].neurons.len(){

                    let mut sum = 0.0; 
                    

                    for k in 0..self.layer[i+1].neurons.len(){

                        sum += self.layer[i+1].neurons[k].weights[j] * nth_derivative[k];


                    }

                   tmp_derivative.push(sum * self.layer[i].neurons[j].output_function.derivative(self.layer[i].neurons[j].z, self.layer[i].neurons[j].y))

                }

                tmp_derivative.reverse();
                nth_derivative = tmp_derivative;

                //change weights of neurons in the i'th layer 
                for j in 0..self.layer[i].neurons.len(){

                    for k in 0..self.layer[i].neurons[j].input.len(){

                        let delta =  nth_derivative[j] * self.layer[i].neurons[j].input[k] * (-self.layer[i].neurons[j].n);
                        
                        self.layer[i].neurons[j].input[k] = self.layer[i].neurons[j].input[k] + delta;
                        

                    }
                    
                }

            }

        }

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

