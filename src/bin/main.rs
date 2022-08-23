use neuralnet::{network::*, neurons::*, output::*, set_rng, state::*};
use rand::{rngs::StdRng, SeedableRng};

//rayon anschauen für paralellität
fn main() {
    set_rng(StdRng::seed_from_u64(12));

    println!("Hello, world!");

    //let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    //let input = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    //let input = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let input = vec![-1.0, -1.0, -1.0, -1.0, -1.0, -1.0];
    let weight = vec![6.0, 5.0, 4.0, 3.0, 2.0, 1.0];

    let n = StaticNeuron {
        //56, 21, 0
        n: 0.2,
        z: 0.0,
        y: 0.0,
        weights: weight,
        state_function: Box::new(Scalar),
        output_function: Box::new(ReLU),
    };

    let n1 = StaticNeuron {
        n: 0.2,
        z: 0.0,
        y: 0.0,
        weights: vec![1.0, 10.0, 14.0, -3.0, 2.0, 1.0], //67, 25, 0
        state_function: Box::new(Scalar),
        output_function: Box::new(ReLU),
    };

    let n2 = StaticNeuron {
        n: 0.2,
        z: 0.0,
        y: 0.0,
        weights: vec![5.0, 42.0],
        state_function: Box::new(Scalar),
        output_function: Box::new(ReLU),
    };

    let layer1 = vec![n, n1];
    let layer2 = vec![n2];
    let net = vec![layer1, layer2];

    let mut neuron = Network { layer: net };

    println!("y = {:?}", neuron.call(input)); //3094, 1155, 0

    neuron.visualize();

    /*
    n.calc(input);

    println!("z = {} y = {}", n.z, n.y);
    */
}
