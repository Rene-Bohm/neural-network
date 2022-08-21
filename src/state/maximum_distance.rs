use std::f64;

pub fn Maximum(weight: Vec<f64>, input: Vec<f64>) -> f64 {
   
    let mut maximum = f64::MIN;
    let mut tmp = 0.0;

    let length = input.len();

    for i in 0..length {

        tmp = (input[i]- weight[i]).abs();

        if tmp > maximum{

            maximum = tmp;

        }

    }

    maximum
}