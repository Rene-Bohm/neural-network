use std::vec;

use neuralnet::{prelude::{*, softmax::batch_softmax}, data::spiral, neuron_layer::BasicLayer, output::relu};


fn main(){

    let data_set = spiral(100, 3);

    let data = &data_set.0;
    let classes = &data_set.1;

    let first = Matrix::from(vec![vec![data[2].0,data[2].1]]);

    let mut layer1 = BasicLayer::initialize(
        2,
        64, 

    );

    let mut relu1 = relu::new();

    let mut layer2 = BasicLayer::initialize(
        64,
        3, 

    );

    

    println!("{}", &first);
    

    layer1.call(&first);

    relu1.forward(&layer1.output.clone().unwrap());

    layer2.call(&relu1.output.unwrap());

    let pred = batch_softmax(layer2.output.clone().unwrap());
    
    println!("{:?}\n{:?}",layer2.output.clone().unwrap(), &pred);

    println!("{:?}", batch_loss(&vec![0 as usize], &pred));



    //for i in 0..10000{}
    
    println!("hi")
}
    
    