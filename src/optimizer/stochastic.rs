use crate::{neuron_layer::BasicLayer, prelude::*};

pub struct Stochastic {
    pub learning_rate: f64,
    pub current_learning_rate: f64,
    pub decay: f64,
    pub momentun: f64,
    pub iteration: u64,
}

impl Stochastic {
    pub fn new(learning_rate: f64, decay: f64, momentum: f64) -> Self {
        Stochastic {
            learning_rate: learning_rate,
            current_learning_rate: learning_rate,
            decay: decay,
            momentun: momentum,
            iteration: 0,
        }
    }

    pub fn update_learning_param(&mut self) -> () {
        self.current_learning_rate =
            self.current_learning_rate / (1.0 + self.decay * self.iteration as f64);
    }

    pub fn update_weights(&self, layer: &mut BasicLayer) -> () {
        let weight_updates = self.momentun * layer.weight_momentum.clone().unwrap()
            - self.current_learning_rate * layer.dweights.clone().unwrap();

        layer.weight_momentum = Some(weight_updates.clone());

        let bias_updates = self.momentun * layer.bias_momentum.clone().unwrap()
            - self.current_learning_rate * layer.dbias.clone().unwrap();

        layer.bias_momentum = Some(bias_updates.clone());

        layer.dweights = Some(layer.dweights.clone().unwrap() + weight_updates);
        layer.dbias = Some(layer.dbias.clone().unwrap() + bias_updates);
    }

    pub fn increment_iteration(&mut self) -> () {
        self.iteration += 1;
    }
}
