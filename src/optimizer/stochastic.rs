use crate::prelude::*;

struct Stochastic{

    pub learning_rate: f64,
    pub current_learning_rate: f64,
    pub decay: f64,
    pub momentun: f64,
    pub iteration: u64


}

impl Stochastic{

    pub fn update_learning_param(mut self) -> (){

        self.current_learning_rate = self.current_learning_rate/(1.0 + self.decay*self.iteration as f64);


    }
    

    pub fn update_weights(self, layer: &mut Layer, weights: Matrix, biases: Matrix) -> () {

        let weight_updates =    
        self.momentun * layer.weight_momentum.clone() - 
        self.current_learning_rate * layer.dweights.clone();
        
        layer.weight_momentum = weight_updates.clone();
        
        let bias_updates =    
        self.momentun * layer.bias_momentum.clone() - 
        self.current_learning_rate * layer.dbias.clone();
        
        layer.bias_momentum = bias_updates.clone();

        layer.dweights = layer.dweights.clone() + weight_updates;
        layer.dbias = layer.dbias.clone() + bias_updates;
        
    }
 
    pub fn increment_iteration(mut self) -> (){

        self.iteration += 1;

    }

}