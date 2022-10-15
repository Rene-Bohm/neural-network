use crate::get_rng;
use crate::output::OutputFunction;
use crate::prelude::*;
extern crate rand;
use rand::{distributions::Uniform, Rng};

pub struct Layer {
    pub neurons: Vec<StaticNeuron>,
    pub weight_momentum: Matrix,
    pub bias_momentum: Matrix,
    pub dweights: Matrix,
    pub dbias: Matrix,

}

impl Layer {
    //------------------------Constructor------------------------

    pub fn new_vec(_layer_input: Vec<StaticNeuron>) -> Self {

        Layer {
            neurons: _layer_input.clone(),
            weight_momentum: Matrix::from(vec![vec![0.0; _layer_input.len()]]),
            bias_momentum: Matrix::from(vec![vec![0.0; _layer_input.len()]]),
            dweights: Matrix::from(vec![vec![0.0]]), //only initial
            dbias: Matrix::from(vec![vec![0.0]]), //only initial
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

        let state: Box<fn(Vec<f64>, Vec<f64>) -> f64> = match state_function {
            StateType::Euklid => Box::new(Euklid),
            StateType::Scalar => Box::new(Scalar),
            StateType::Min => Box::new(Minimum),
            StateType::Max => Box::new(Maximum),
            StateType::Manhatten => Box::new(Manhattan),
        };

        //------------------------------------------------------

        let output: Box<dyn OutputFunction> = match output_function {
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
                output_function: output.clone(),
                state_function: state.clone(),
            });
        }

        //------------------------------------------------------

        let len = neuron_layer.len();

        Layer {
            neurons: neuron_layer,
            weight_momentum: Matrix::from(vec![vec![0.0; len]]),
            bias_momentum: Matrix::from(vec![vec![0.0; len]]),
            dweights: Matrix::from(vec![vec![0.0]]), //only initial
            dbias: Matrix::from(vec![vec![0.0]]),
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

    pub fn get_weights(&self) -> Matrix{

        let mut tmp_weights: Vec<Vec<f64>> = Vec::new();

        for i in 0..self.neurons.len(){

            tmp_weights.push(self.neurons[i].get_weights());

        }

        Matrix::from(tmp_weights)

    }

    pub fn get_bias(&self) -> Matrix{

        let mut tmp_bias: Vec<f64> = Vec::new();

        for i in 0..self.neurons.len(){

            tmp_bias.push(self.neurons[i].get_bias());

        }

        Matrix::from(vec![tmp_bias])

    }

}

#[cfg(test)]
mod test{
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    use crate::prelude::{StateType, OutputType};
    use crate::{get_rng, set_rng};
    use super::Layer;
    use crate::prelude::*;


    #[test]
    fn getting_weights(){

        set_rng(StdRng::seed_from_u64(12));

        

        let l1 = Layer::new_uniform(
            3,
            2,
            StateType::Scalar,
            OutputType::ReLU,
            0.0,
            1.0,
        );

        let m1 = l1.get_weights();

        println!("{:?}\n{:?}\n{:?}\n", l1.neurons[0].weights,l1.neurons[1].weights,l1.neurons[2].weights);

        println!("{}", m1);

    }

    #[test]
    fn getting_bias(){

        set_rng(StdRng::seed_from_u64(12));

        

        let l1 = Layer::new_uniform(
            3,
            2,
            StateType::Scalar,
            OutputType::ReLU,
            0.0,
            1.0,
        );

        let m1 = l1.get_bias();

        println!("{:?}\n{:?}\n{:?}\n", l1.neurons[0].bias,l1.neurons[1].bias,l1.neurons[2].bias);

        println!("{}", m1);

    }


}