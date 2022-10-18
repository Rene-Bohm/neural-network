use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_distr::Uniform;

use crate::{get_rng, set_rng};

pub fn linspace(start: f64, end: f64, samples: usize) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();

    if (start >= end) {
        panic!("Start value is bigger then end value for linspace");
    }

    if (samples > 1) {
        let diff = (end - start) / (samples as f64 - 1.0);

        result.push(start);

        for i in 1..samples {
            if (i == samples) {
                result.push(end);
            } else {
                result.push(result[i - 1] + diff);
            }
        }
    } else if (samples == 1) {
        result.push(start);
    }

    result
}

pub fn get_n<T>(input: &Vec<Vec<T>>, idx1: usize, idx2: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut new: Vec<Vec<T>> = vec![Vec::<T>::new(), Vec::<T>::new()];

    for i in idx1..idx2 {
        new[0].push(input[0][i].clone());
        new[1].push(input[1][i].clone());
    }

    new
}

//todo fix spiral
pub fn spiral(elements: usize, classes: usize) -> (Vec<Vec<f64>>, Vec<usize>) {
    let mut points = vec![vec![0.0; elements * classes], vec![0.0; elements * classes]];
    let mut respectiv_class = vec![0 as usize; elements * classes];

    set_rng(StdRng::seed_from_u64(15));
    let rng = get_rng();

    let mut round = 0 as usize;

    for class_num in 0..classes {
        let i1 = linspace(0.0, 1.0, elements);
        let mut i2 = linspace(
            (class_num * 4) as f64,
            ((class_num + 1) * 4) as f64,
            elements,
        );

        let i2: Vec<f64> = i2
            .iter()
            .map(|x| x + rng.sample(Uniform::new(0.0, 1.0)))
            .collect();

        for sample in round..round + elements {
            let minus_round = sample - round;
            points[0][sample] = i1[minus_round] * f64::sin(i2[minus_round] * 2.5);
            points[1][sample] = i1[minus_round] * f64::cos(i2[minus_round] * 2.5);
        }

        for class_index in round..round + elements {
            respectiv_class[class_index] = class_num;
        }

        round += elements;
    }

    (points, respectiv_class)
}

#[cfg(test)]
mod test {
    use crate::data::get_n;

    use super::{linspace, spiral};

    #[test]
    fn spiralo() {
        let elements = 3 as usize;

        let X = spiral(elements, 3);

        println!("{:?}", X.0);

        let n = get_n(&X.0, 0, 3);

        println!("{:?}", &n);
    }

    #[test]
    fn lin() {
        let lin = linspace(0.0, 1.0, 0);
        assert_eq!(lin, Vec::<f64>::new());

        let lin = linspace(5.0, 6.0, 1);
        assert_eq!(lin, vec![5.0]);

        let lin = linspace(5.0, 10.0, 5);
        assert_eq!(lin, vec![5.0, 6.25, 7.5, 8.75, 10.0]);

        let lin = linspace(5.0, 7.5, 5);
        assert_eq!(lin, vec![5.0, 5.625, 6.25, 6.875, 7.5]);
    }
}
