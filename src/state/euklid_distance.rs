use std::f64;

pub fn Euklid(weight: Vec<f64>, input: Vec<f64>) -> f64 {
   
    let mut sum = 0.0;

    let length = input.len();
    
    for i in 0..length {

        sum += (input[i]- weight[i]).powi(2);
    }

    sum.sqrt()
}