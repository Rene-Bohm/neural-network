use crate::prelude::*;
use f64;


pub fn batch_accuracy(actual: &Vec<usize>, expected: &Matrix) -> f64 {
    
    let dimension = expected.dimension();

    if actual.len() != dimension.0 || dimension.0 == 0 || dimension.1 == 0 {
        println!("actual.len(): {}, dimension: {:?}", actual.len(), dimension);

        panic!("Batch Accuracy cant be calculated")

    } else {

        let mut max_index = actual[0];

        for i in actual.iter(){

            if *i > max_index {
                max_index = *i;
            }
        }
        if(max_index >= dimension.1){

            panic!("Batch Accuracy cant be calculated, index is indexing value outside of the coloumn")

        }else{

            let samples_num = actual.len() as f64;

            let mut sum = 0.0;

            for i in 0..dimension.0{
                
                let mut row_max = expected[i][0];
                
                for i in expected[i].into_iter() {
                    if *i > row_max {
                        row_max = *i;
                    }
                }

                sum += if expected[i][actual[i]] != row_max {
                        1.0
                    } else {
                        0.0
                    };
            }
            sum / samples_num
        }
    }
}

pub fn accuracy(actual: usize, expected: Vec<f64>) -> f64 {
    let mut max = expected[0];

    for i in &expected {
        if *i > max {
            max = *i;
        }
    }

    if expected[actual] != max {
        1.0
    } else {
        0.0
    }
}

pub fn avg_batch_accuracy(actual: Vec<usize>, expected: Vec<Vec<f64>>) -> f64 {
    let len = expected.len() as f64;
    let mut sum = 0.0;
    for i in 0..expected.len() {
        sum += accuracy(actual[i], expected[i].clone());
    }

    sum / len
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn do_works() {
        let actual: usize = 1;
        let expected: Vec<f64> = vec![0.7, 0.1, 0.2];

        let loss = accuracy(actual, expected);

        assert_eq!(0.0, loss);

        let actual: usize = 0;
        let expected: Vec<f64> = vec![0.7, 0.1, 0.2];

        let loss = accuracy(actual, expected);
        assert_eq!(1.0, loss);
    }

    #[test]
    fn batch() {
        let actual: Vec<usize> = vec![0, 1, 1];
        let expected: Vec<Vec<f64>> = vec![
            vec![0.7, 0.0, 0.2],
            vec![0.1, 0.5, 0.4],
            vec![0.02, 0.9, 0.08],
        ];

        let loss = avg_batch_accuracy(actual, expected);

        assert_eq!(0.0, loss);

        let actual: Vec<usize> = vec![1];
        let expected: Vec<Vec<f64>> = vec![vec![0.7, 0.0, 0.2]];

        let loss = avg_batch_accuracy(actual, expected);

        assert_eq!(1.0, loss);
    }

    #[test]
    fn matrix_batch(){

        let actual: Vec<usize> = vec![0, 1, 1];
        let expected= Matrix::from(vec![
            vec![0.7, 0.0, 0.2],
            vec![0.1, 0.5, 0.4],
            vec![0.02, 0.9, 0.08],
        ]);

        let acc = batch_accuracy(&actual, &expected);

        assert_eq!(1.0, acc);

        let actual: Vec<usize> = vec![1];
        let expected = Matrix::from(vec![vec![0.7, 0.0, 0.2]]);

        let acc = batch_accuracy(&actual, &expected);

        assert_eq!(0.0, acc);

        let actual: Vec<usize> = vec![1, 2, 0];
        let expected= Matrix::from(vec![
            vec![0.7, 0.0, 0.2],
            vec![0.1, 0.5, 0.4],
            vec![0.02, 0.9, 0.08],
        ]);

        let acc = batch_accuracy(&actual, &expected);


        assert_eq!(0.0, acc);

    }
}
