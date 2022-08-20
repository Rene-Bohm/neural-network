use nalgebra as alg;

pub fn Scalar(weight: alg::DVector<f64>, input: alg::DVector<f64>) -> f64 {
    let mut sum = 0.0;

    let shape = input.shape();
    let length = shape.0;

    for i in 0..length {
        sum += weight[i] * input[i]
    }

    sum
}