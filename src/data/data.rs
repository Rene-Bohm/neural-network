use rand::{rngs::StdRng, SeedableRng, Rng};
use rand_distr::Uniform;

use crate::{get_rng, set_rng};


pub fn linspace(start: f64, end: f64, samples: usize) -> Vec<f64>{

    let mut result:Vec<f64> = Vec::new();

    if(start >= end){

        panic!("Start value is bigger then end value for linspace");

    }

    if (samples>1){

        let diff = (end - start)/(samples as f64 - 1.0);

        result.push(start);

        for i in 1..samples{

            if(i == samples){

                result.push(end);

            }else{

                result.push(result[i-1] + diff);

            }
        }

    }else if(samples == 1){

        result.push(start);

    }

    result


}

pub fn spiral(samples: usize, classes: usize) -> (Vec<(f64,f64)>, Vec<usize>){

    let mut points = vec![(0.0, 0.0); samples*classes];
    let mut respectiv_class = vec![0 as usize; samples*classes];

    set_rng(StdRng::seed_from_u64(12));
    let rng = get_rng();

    let mut round = 0 as usize;

    for class_num in 0..classes{

        let i1 = linspace(0.0, 1.0, samples);
        let mut i2 = linspace((class_num*4) as f64, ((class_num+1)*4) as f64, samples);

        let i2: Vec<f64> = i2.iter().map(|x| x + rng.sample(Uniform::new(0.0, 1.0))).collect();

        for samples in round..round+samples{

            points[samples].0 = i1[samples%3] * f64::sin(i2[samples%3] * 2.5);
            points[samples].1 = i1[samples%3] * f64::cos(i2[samples%3] * 2.5);

        }

        for class_index in round..round+samples{

            respectiv_class[class_index] = class_num;

        }

        round += classes;

    }

    (points, respectiv_class)

}

#[cfg(test)]
mod test{
    use super::{spiral, linspace};


    #[test]
    fn spiralo(){

        let X = spiral(3, 3);

        println!("{:?}\n{:?}", X.0, X.1);


    }

    #[test]
    fn lin(){

        let lin = linspace(0.0, 1.0, 0);
        assert_eq!(lin, Vec::<f64>::new());

        let lin = linspace(5.0, 6.0, 1);
        assert_eq!(lin, vec![5.0]);

        let lin = linspace(5.0, 10.0, 5);
        assert_eq!(lin, vec![5.0, 6.25, 7.5, 8.75, 10.0]);

        let lin = linspace(5.0, 7.5, 5);
        assert_eq!(lin, vec![5.0,5.625,6.25,6.875,7.5]);

    }


}