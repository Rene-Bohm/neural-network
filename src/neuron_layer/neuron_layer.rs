use crate::{enums::*, get_rng, neurons::*, output::*, state::*};

extern crate rand;
use rand::{distributions::Uniform, Rng};

pub struct Layer {
    pub neurons: Vec<StaticNeuron>,
}

impl Layer {
    //------------------------Constructor------------------------

    pub fn new_vec(_layer_input: Vec<StaticNeuron>) -> Self {
        Layer {
            neurons: _layer_input,
        }
    }

    pub fn new_uniform(
        num_neurons: u32,
        num_of_weights: u32,
        state_function: StateType,
        output_function: OutputType,
        output_param: f64,
        learn_factor: f64,
    ) -> Self {
        let rng = get_rng();

        let mut neuron_layer: Vec<StaticNeuron> = Vec::new();

        //-----------------------------------------------------

        let State: Box<fn(Vec<f64>, Vec<f64>) -> f64> = match state_function {
            StateType::Euklid => Box::new(Euklid),
            StateType::Scalar => Box::new(Scalar),
            StateType::Min => Box::new(Minimum),
            StateType::Max => Box::new(Maximum),
            StateType::Manhatten => Box::new(Manhattan),
        };

        //------------------------------------------------------

        let Output: Box<dyn OutputFunction> = match output_function {
            OutputType::Id => Box::new(Id),
            OutputType::Step => Box::new(Step::new(output_param)),
            OutputType::Fermi => Box::new(Fermi::new(output_param)),
            OutputType::Tangens => Box::new(Tangens),
            OutputType::ReLU => Box::new(ReLU),
            OutputType::ZeroReLU => Box::new(ZeroReLU),
            OutputType::Gauss => Box::new(Gauss::new(output_param)),
        };

        //------------------------------------------------------

        for _ in 0..num_neurons {
            let mut weights: Vec<f64> = Vec::new();

            for _ in 0..num_of_weights {
                weights.push(rng.sample(Uniform::new(0.0, 1.0)));
            }

            neuron_layer.push(StaticNeuron {
                n: learn_factor,
                z: 0.0,
                y: 0.0,
                bias: rng.sample(Uniform::new(0.0, 1.0)),
                input: Vec::with_capacity(weights.len()),
                weights: weights,
                output_function: Output.clone(),
                state_function: State.clone(),
            });
        }

        //------------------------------------------------------

        Layer {
            neurons: neuron_layer,
        }
    }

    //------------------------Calculation------------------------

    pub fn call(&mut self, input: Vec<f64>) -> Vec<f64> {
        let mut output: Vec<f64> = Vec::new();

        for i in 0..self.neurons.len() {
            self.neurons[i].calc(input.clone());

            output.push(self.neurons[i].y);
        }

        output
    }

    //------------------------Visualization------------------------

    pub fn visualize(&self) {
        for i in 0..self.neurons.len() {
            println!(
                "This is neuron {}.\nThis is the current state \n{}",
                i + 1,
                self.neurons[i]
            );

            println!("--------<>-------\n");
        }
    }
}
