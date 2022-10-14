use crate::prelude::*;
use f64;

pub fn batch_loss(actual: &Vec<usize>, expected: &Matrix) -> f64 {
    
    let dimension = expected.dimension();

    if actual.len() != dimension.0 || dimension.0 == 0 || dimension.1 == 0 {
        println!("actual.len(): {}, dimension: {:?}", actual.len(), dimension);

        panic!("Batch loss cant be calculated")

    } else {

        let mut max_index = actual[0];

        for i in actual.iter(){

            if *i > max_index {
                max_index = *i;
            }
        }
        if(max_index >= dimension.1){

            panic!("Batch loss cant be calculated, index is indexing value outside of the coloumn")

        }else{

            let samples_num = actual.len() as f64;

            let mut sum = 0.0;

            for i in 0..dimension.0{
                
                sum += if expected[i][actual[i]] == 0.0{
                        -f64::ln(1e-7)
                    } else {
                        -f64::ln(expected[i][actual[i]] - 1e-7)
                    };
            }
            sum / samples_num
        }
    }
}


pub fn loss(actual: usize, expected: Vec<f64>) -> f64 {
    if expected[actual] == 0.0 {
        -f64::ln(1e-7)
    } else {
        -f64::ln(expected[actual] - 1e-7)
    }
}

pub fn avg_batch_loss(actual: Vec<usize>, expected: Vec<Vec<f64>>) -> f64 {
    let len = expected.len() as f64;
    let mut sum = 0.0;
    for i in 0..expected.len() {
        sum = loss(actual[i], expected[i].clone());
    }

    sum / len
}

//---------------------------------------------------------------------------

pub fn batch_derivative(softmax_output: &mut Matrix, expected_class_index: &Vec<usize>) -> Matrix{

    let dimension = softmax_output.dimension();

    if expected_class_index.len() != dimension.0 || dimension.0 == 0 || dimension.1 == 0 {
        println!("expected_class_index.len(): {}, dimension: {:?}", expected_class_index.len(), dimension);

        panic!("Batch derivative cant be calculated")

    } else {

        let mut max_index = expected_class_index[0];

        for i in expected_class_index.iter(){

            if *i > max_index {
                max_index = *i;
            }
        }
        if(max_index >= dimension.1){

            panic!("Batch derivative cant be calculated, index is indexing value outside of the coloumn")

        }else{

            let samples_num = expected_class_index.len() as f64;

            for i in 0..dimension.0{
                
                softmax_output[i][expected_class_index[i]] -= 1.0;

            }

            let m1 = softmax_output.clone().map(|x| x / dimension.0 as f64);
            
            m1
        }
    }

}

pub fn softmax_cross_derivative(
    mut softmax_out: Vec<f64>,
    expected_class_index: usize,
) -> Vec<f64> {
    //
    let len = softmax_out.len();

    softmax_out[expected_class_index] -= 1.0;

    softmax_out.iter().map(|&x| x / len as f64).collect()
}

pub fn batch_softmax_cross_derivative(
    mut softmax_out: Vec<Vec<f64>>,
    expected_class_index: Vec<usize>,
) -> Vec<Vec<f64>> {
    for i in 0..softmax_out.len() {
        softmax_out[i] = softmax_cross_derivative(softmax_out[i].clone(), expected_class_index[i]);
    }
    softmax_out
}

#[cfg(test)]
mod test {

    use loss::batch_derivative;

    use crate::{loss_calc::*, linalg::Matrix};

    #[test]
    fn do_works() {
        let actual: usize = 0;
        let expected: Vec<f64> = vec![0.7, 0.1, 0.2];

        let loss = loss(actual, expected);

        println!("{}", loss);

        assert_eq!(0.35667494393873245, loss);
    }

    #[test]
    fn zero() {
        let actual: usize = 1;
        let expected: Vec<f64> = vec![0.7, 0.0, 0.2];

        let loss = loss(actual, expected);

        println!("{}", loss);

        assert_eq!(16.11809565095832, loss);
    }

    #[test]
    fn batch() {
        let actual: Vec<usize> = vec![0, 1, 1];
        let expected: Vec<Vec<f64>> = vec![
            vec![0.7, 0.1, 0.2],
            vec![0.1, 0.5, 0.4],
            vec![0.02, 0.9, 0.08],
        ];

        let loss = avg_batch_loss(actual, expected);

        assert_eq!(0.035120171885942096, loss);

        let actual: Vec<usize> = vec![1];
        let expected: Vec<Vec<f64>> = vec![vec![0.7, 0.0, 0.2]];

        let loss = avg_batch_loss(actual, expected);

        assert_eq!(16.11809565095832, loss);
    }

    #[test]
    fn matrix_batch(){

        let actual: Vec<usize> = vec![0, 1, 1];
        let expected= Matrix::from(vec![
            vec![0.7, 0.1, 0.2],
            vec![0.1, 0.5, 0.4],
            vec![0.02, 0.9, 0.08],
        ]);

        let loss = batch_loss(&actual, &expected);
        
        assert_eq!(0.3850610313749314, loss);

        let actual: Vec<usize> = vec![1];
        let expected = Matrix::from(vec![vec![0.7, 0.0, 0.2]]);

        let loss = batch_loss(&actual, &expected);

        assert_eq!(16.11809565095832, loss);

        let actual: Vec<usize> = vec![1, 2, 0];
        let expected= Matrix::from(vec![
            vec![0.7, 0.0, 0.2],
            vec![0.1, 0.5, 0.4],
            vec![0.02, 0.9, 0.08],
        ]);

        let loss = batch_loss(&actual, &expected);
        assert_eq!(6.982138212757718, loss);

    }

    #[test]
    fn soft_cross_derivative() {
        let actual: usize = 0;
        let vector: Vec<f64> = vec![0.7, 0.1, 0.2];
        let result = softmax_cross_derivative(vector, actual);

        println!("{:?}", &result);
        assert_eq!(
            vec![
                -0.10000000000000002,
                0.03333333333333333,
                0.06666666666666667
            ],
            result
        );
    }

    #[test]
    fn batch_soft_cross_derivative() {
        let actual: Vec<usize> = vec![0, 1, 1];
        let vector: Vec<Vec<f64>> = vec![
            vec![0.7, 0.1, 0.2],
            vec![0.1, 0.5, 0.4],
            vec![0.02, 0.9, 0.08],
        ];
        let result = batch_softmax_cross_derivative(vector, actual);

        println!("{:?}", &result);

        assert_eq!(
            result,
            vec![
                vec![
                    -0.10000000000000002,
                    0.03333333333333333,
                    0.06666666666666667
                ],
                vec![
                    0.03333333333333333,
                    -0.16666666666666667,
                    0.13333333333333333
                ],
                vec![
                    0.006666666666666667,
                    -0.033333333333333326,
                    0.02666666666666667
                ]
            ]
        );
    }

    #[test]
    fn batch_derivatives() {

        let actual: Vec<usize> = vec![0, 1, 1];
        let mut m1 = Matrix::from(vec![
            vec![0.7, 0.1, 0.2],
            vec![0.1, 0.5, 0.4],
            vec![0.02, 0.9, 0.08],
        ]);

        let result = batch_derivative(&mut m1, &actual);

        println!("{:?}", &result);

        assert_eq!(
            result,
            Matrix::from(vec![
                vec![
                    -0.10000000000000002,
                    0.03333333333333333,
                    0.06666666666666667
                ],
                vec![
                    0.03333333333333333,
                    -0.16666666666666667,
                    0.13333333333333333
                ],
                vec![
                    0.006666666666666667,
                    -0.033333333333333326,
                    0.02666666666666667
                ]
            ]
        ));
    }

}
