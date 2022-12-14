use std::{array, f64};

use crate::linalg::Matrix;

pub fn vec_softmax(input: Vec<f64>) -> Vec<f64> {
    // Overflow prevention

    let mut max = input[0];

    for i in &input {
        if *i > max {
            max = *i;
        }
    }

    let mut new_input: Vec<f64> = Vec::new();

    for i in &input {
        new_input.push(i - max);
    }

    //------------------------------------

    let mut output: Vec<f64> = Vec::new();

    let mut sum = 0.0;

    for i in new_input {
        let e = f64::exp(i);
        sum += e;
        output.push(e);
    }

    for i in 0..output.len() {
        output[i] = output[i] / sum;
    }

    output
}

pub fn batch_softmax(mut input: Matrix) -> Matrix {
    //println!("{}", &input);

    let dim = input.dimension();

    for i in 0..dim.0 {
        let mut max = input[i][0];

        for j in 1..dim.1 {
            if input[i][j] > max {
                max = input[i][j];
            }
        }
        for j in 0..dim.1 {
            input[i][j] -= max;
        }
    }

    //println!("{}", &input);

    let mut output = Matrix::new(dim.0, dim.1);

    for i in 0..dim.0 {
        let mut sum = 0.0;

        for j in 0..dim.1 {
            let e = f64::exp(input[i][j] as f64);
            sum += e;
            output[i][j] = e;
        }

        for k in 0..dim.1 {
            output[i][k] = output[i][k] / sum;
        }
    }

    Matrix::from(output)
}

pub fn softmax_derivative(input: Vec<f64>) /*-> Vec<f64>*/
{
    let dimension = input.len();
    let mut jacobian: Vec<Vec<f64>> = vec![vec![0.0f64; dimension]; dimension];

    for i in 0..dimension {
        for j in 0..dimension {
            if i != j {
                jacobian[i][j] = &input[i] * &input[j] * -1.0;
            } else {
                jacobian[i][i] = &input[i] * (1.0 - &input[i]);
            }

            print!("{:?} ", jacobian[i][j]);
        }
        println!("");
    }

    println!("");
}

#[cfg(test)]
mod tests {
    use softmax::vec_softmax;

    use crate::{
        linalg::Matrix,
        output::softmax::{self, batch_softmax},
    };

    use super::softmax_derivative;

    #[test]
    fn it_works() {
        let result = vec![4.8, 1.21, 2.385];
        let res = vec_softmax(result);
        assert_eq!(
            vec![0.8952826639572619, 0.02470830678209937, 0.0800090292606387],
            res
        );
    }

    #[test]
    fn ndarray() {
        //let result = vec![0.08468093, 0.02426149, 0.6577931, 0.05399495, 0.17926953];
        let result = vec![0.7, 0.1, 0.2];
        softmax_derivative(result);
    }

    #[test]
    fn batchM() {
        let result = Matrix::from(vec![vec![4.8, 1.21, 2.385]]);
        let res = batch_softmax(result);
        assert_eq!(
            Matrix::from(vec![vec![
                0.8952826639572619,
                0.02470830678209937,
                0.0800090292606387
            ]]),
            res
        );
    }
}
