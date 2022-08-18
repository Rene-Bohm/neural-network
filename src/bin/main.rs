use nalgebra as na;
use neuralnet::state::*;

fn main() {
    println!("Hello, world!");

    let input = na::dvector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let weight = na::dvector![6.0, 5.0, 4.0, 3.0, 2.0, 1.0];

    let ex = ScalarProd {};

    println!("{}", ex.calc(weight, input));
}
