use nalgebra as na;
use neuralnet::{neurons::LeakyNeuron, output::Id, state::Euklid_Distance::Euklid_Distance};

//rayon anschauen für paralellität
fn main() {
    println!("Hello, world!");

    let input = na::dvector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let weight = na::dvector![6.0, 5.0, 4.0, 3.0, 2.0, 1.0];

    let mut n = LeakyNeuron {
        z: 0.0,
        y: 0.0,
        state_function: Box::new(Euklid_Distance),
        output_function: Box::new(Id {}),
    };

    n.calc(weight, input);

    println!("z = {} y = {}", n.z, n.y);
}
