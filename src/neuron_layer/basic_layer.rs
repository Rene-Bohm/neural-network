use crate::get_rng;
use crate::output::OutputFunction;
use crate::prelude::*;
extern crate rand;
use rand::{distributions::Uniform, Rng};

pub struct BasicLayer {
    pub neurons: Matrix,        //abstract neuron set represented as a weight Matrix
    pub input: Option<Matrix>,  //Last input
    pub dinput: Option<Matrix>, //derivative with respect to input
    pub weight_momentum: Option<Matrix>,
    pub dweights: Option<Matrix>, //derivative with respect to input
    pub bias: Vec<f64>,
    pub bias_momentum: Option<Matrix>,
    pub dbias: Option<Matrix>, //derivative with respect to bias
    pub output: Option<Matrix>,
}

impl BasicLayer {
    //------------------------Constructor------------------------

    pub fn initialize(num_inputs: usize, num_neurons: usize) -> Self {
        let rng = get_rng();

        //------------------------------------------------------

        let mut neuron_row: Vec<Vec<f64>> = Vec::new();

        for _ in 0..num_inputs {
            let mut neuron_column: Vec<f64> = Vec::new();

            for _ in 0..num_neurons {
                neuron_column.push(0.0);
            }

            neuron_row.push(neuron_column);
        }

        let neuron_matrix = Matrix::from(neuron_row);

        //------------------------------------------------------

        BasicLayer {
            neurons: neuron_matrix, //abstract neuron set represented as a weight Matrix
            input: None,            //Last input
            dinput: None,           //derivative with respect to input
            weight_momentum: None,
            dweights: None, //derivative with respect to input
            bias: vec![0.0; num_neurons],
            bias_momentum: None,
            dbias: None, //derivative with respect to bias
            output: None,
        }
    }

    //------------------------Calculation------------------------

    pub fn call(&mut self, input: &Matrix) -> () {
        let weight_dim = self.neurons.dimension();
        let input_dim = &input.dimension();

        if (input_dim.1 == weight_dim.0) {
            self.input = Some(input.clone());

            let mut result = input.clone().dot(&self.neurons);

            result.clone().add_row(&self.bias);

            self.output = Some(result);
        } else {
            panic!(
                "Forward cant be calculated\n
                {}x{} != {}x{}",
                weight_dim.0, weight_dim.1, input_dim.0, input_dim.1
            );
        }
    }

    pub fn backwards(&mut self, derivative_value: &Matrix) {
        self.dweights = Some(
            self.input
                .as_ref()
                .unwrap()
                .transpose()
                .dot(derivative_value),
        );
        //println!("-");
        self.dbias = Some(derivative_value.clone().sum(Some(true)));
        //println!("--");
        self.dinput = Some(
            derivative_value
                .clone()
                .dot(&self.neurons.clone().transpose()),
        );
        //println!("---");
    }

    pub fn get_weights(&self) -> Matrix {
        self.neurons.clone()
    }

    pub fn get_bias(&self) -> Vec<f64> {
        self.bias.clone()
    }
}

#[cfg(test)]
mod test {

    #[test]
    pub fn add() {}
}
