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

    pub fn update_weights(self, mut neuron: StaticNeuron) -> () {

        //let x = neuron.weights.iter().map(|&x| x / len as f64).collect();
        

    }



    pub fn increment_iteration(mut self) -> (){

        self.iteration += 1;

    }

}