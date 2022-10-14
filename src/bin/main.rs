use neuralnet::{prelude::*, set_rng};
use rand::{rngs::StdRng, SeedableRng};

//rayon anschauen für paralellität
fn main() {
    set_rng(StdRng::seed_from_u64(12));

    println!("Hello, world!");

    let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    //let input = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    //let input = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    //let input = vec![-1.0, -1.0, -1.0, -1.0, -1.0, -1.0];

    let mut net = ReluNetwork::new(vec![1, 3], 0.0, 6);

    println!("y = {:?}", net.call(input));

    net.visualize();


    let mut vec:Vec<i32> = Vec::new();
    for i in 0..10{

        vec.push(i);

    }

    println!("{:?}", vec);

    /*
    n.calc(input);

    println!("z = {} y = {}", n.z, n.y);
    */
}
