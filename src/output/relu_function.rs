use crate::linalg::Matrix;

use super::OutputFunction;
use std::f64;

#[derive(Clone)]
pub struct ZeroReLU;

impl OutputFunction for ZeroReLU {
    fn call(&self, state: f64) -> f64 {
        f64::max(state, 0.0)
    }

    fn derivative(&self, z: f64, y: f64) -> f64 {
        if z > 0.0 {
            1.0
        } else {
            0.0
        }
    }
}

#[derive(Clone)]
pub struct ReLU;

impl OutputFunction for ReLU {
    fn call(&self, state: f64) -> f64 {
        f64::max(state, state * 0.01)
    }

    fn derivative(&self, z: f64, y: f64) -> f64 {
        if z > 0.0 {
            1.0
        } else {
            0.01
        }
    }
}

pub struct relu {
    pub input: Option<Matrix>,
    pub dinput: Option<Matrix>,
    pub output: Option<Matrix>,
}

impl relu {
    pub fn new() -> Self {
        relu {
            input: None,
            dinput: None,
            output: None,
        }
    }

    pub fn forward(&mut self, input: &Matrix) -> () {
        self.input = Some(input.clone());

        self.output = Some(input.clone().map(|x| f64::max(x, 0.0)));
    }

    pub fn backwards(&mut self, derivative_value: &Matrix) {
        let mut dinput = derivative_value.clone();

        let relu_input = self.input.clone().unwrap();

        for i in 0..dinput.dimension().0 {
            for j in 0..dinput.dimension().1 {
                if (relu_input[i][j] <= 0.0) {
                    dinput[i][j] = 0.0;
                }
            }
        }

        self.dinput = Some(dinput);
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;
    use crate::prelude::*;

    #[test]
    fn do_test() {
        let input = vec![
            vec![-0.1, 0.03, 0.06],
            vec![0.03, -0.16, 0.13],
            vec![0.0, -0.03, 0.02],
        ];

        let input = Matrix::from(input);

        let entry = vec![
            vec![-0.1, 0.03, 0.06],
            vec![0.03, -0.16, 0.13],
            vec![0.0, -0.03, 0.02],
        ];

        let m0 = Matrix::from(entry);

        let entry1 = vec![
            vec![0.0, 0.03, 0.06],
            vec![0.03, 0.0, 0.13],
            vec![0.0, 0.0, 0.02],
        ];

        let m1 = Matrix::from(entry1);

        let mut relu = relu {
            input: Some(input),
            dinput: None,
            output: None,
        };

        relu.backwards(&m0);

        assert_eq!(m1, relu.dinput.unwrap());
    }
}
