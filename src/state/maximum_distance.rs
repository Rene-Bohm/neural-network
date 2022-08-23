use std::f64;

pub fn Maximum(weight: Vec<f64>, input: Vec<f64>) -> f64 {
    let mut maximum = f64::MIN;

    let length = input.len();

    for i in 0..length {
        let tmp = (input[i] - weight[i]).abs();

        if tmp > maximum {
            maximum = tmp;
        }
    }

    maximum
}
