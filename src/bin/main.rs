use rand::{rngs::StdRng, SeedableRng};
use neuralnet::{prelude::*, set_rng};

//rayon anschauen für paralellität
fn main() {

    set_rng(StdRng::seed_from_u64(12));

    println!("Hello, world!");

    //let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    //let input = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    //let input = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let input = vec![-1.0, -1.0, -1.0, -1.0, -1.0, -1.0];

   
    let mut net = Network::new_uniform(
        vec![
            (4,StateType::Scalar,OutputType::ReLU),
            (7,StateType::Scalar,OutputType::ReLU),
            (3,StateType::Scalar,OutputType::ReLU)],
        0.0,
        0.2,
        6,);

    

    println!("y = {:?}", net.call(input));

    net.visualize();

    /*
    n.calc(input);

    println!("z = {} y = {}", n.z, n.y);
    */
}
