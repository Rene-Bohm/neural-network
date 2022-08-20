use nalgebra as alg;
use std::f64;

pub fn Minimum(weight: alg::DVector<f64>, input: alg::DVector<f64>) -> f64 {
   
    let mut minimum = f64::MAX;
    let mut tmp = 0.0;

    let shape = input.shape();
    let length = shape.0;

    for i in 0..length {

        tmp = (input[i]- weight[i]).abs();

        if tmp < minimum{

            minimum = tmp;

        }

    }

    minimum
}