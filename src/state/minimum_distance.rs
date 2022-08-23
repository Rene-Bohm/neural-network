use std::f64;

pub fn Minimum(weight: Vec<f64>, input: Vec<f64>) -> f64 {
    let mut minimum = f64::MAX;

    let length = input.len();

    for i in 0..length {
        let tmp = (input[i] - weight[i]).abs();

        if tmp < minimum {
            minimum = tmp;
        }
    }

    minimum
}
