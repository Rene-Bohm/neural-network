use super::StateFunction;
use nalgebra as alg;

pub struct ScalarProd {}

impl StateFunction for ScalarProd {
    fn calc(&self, weight: alg::DVector<f64>, input: alg::DVector<f64>) -> f64 {
        let mut sum = 0.0;

        let shape = input.shape();
        let length = shape.0;

        for i in 0..length {
            sum += weight[i] * input[i]
        }

        sum
    }
}
