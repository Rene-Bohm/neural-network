use nalgebra as na;
use neuralnet::{
    neurons::*, output::*, state::*,
};

//rayon anschauen für paralellität
fn main() {
    println!("Hello, world!");

    let input = na::dvector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let weight = na::dvector![6.0, 5.0, 4.0, 3.0, 2.0, 1.0];

    let mut n = StaticNeuron {
        z: 0.0,
        y: 0.0,
        weights: weight,
        state_function: Box::new(Minimum),
        output_function: Box::new(Tangens),
    };

    n.calc(input);

    println!("z = {} y = {}", n.z, n.y);
}
