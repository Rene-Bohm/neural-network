use f64;

pub fn loss(actual: usize, expected: Vec<f64>) -> f64 {
    let loss = match expected[actual] {
        0.0 => -f64::ln(1e-7),
        _ => -f64::ln(expected[actual]),
    };
    loss
}

pub fn avg_batch_loss(actual: Vec<usize>, expected: Vec<Vec<f64>>) -> f64 {
    let len = expected.len() as f64;
    let mut sum = 0.0;
    for i in 0..expected.len() {
        sum = loss(actual[i], expected[i].clone());
    }

    sum / len
}

#[cfg(test)]
mod test {

    use crate::loss_calc::*;

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
            vec![0.7, 0.0, 0.2],
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
}
