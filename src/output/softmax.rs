use std::f64;
pub fn softmax(input: Vec<f64>) -> Vec<f64> {
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

#[cfg(test)]
mod tests {
    use crate::output::softmax;

    #[test]
    fn it_works() {
        let result = vec![4.8, 1.21, 2.385];
        let res = softmax(result);
        assert_eq!(
            vec![0.8952826639572619, 0.02470830678209937, 0.0800090292606387],
            res
        );
    }
}
