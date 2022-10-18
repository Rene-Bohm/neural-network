use std::vec;

use neuralnet::{
    data::spiral,
    neuron_layer::BasicLayer,
    output::relu,
    prelude::{softmax::batch_softmax, *},
};

fn main() {
    //----------------------------------------------------------------------------------------
    //Setup

    let data_set = spiral(2, 3);

    let data = &data_set.0;
    let classes = &data_set.1;

    let first = Matrix::from(vec![
        vec![data[0].0, data[0].1],
        vec![data[1].0, data[1].1],
        vec![data[2].0, data[2].1],
        vec![data[3].0, data[3].1],
        vec![data[4].0, data[4].1],
        vec![data[5].0, data[5].1],
    ]);

    let mut layer1 = BasicLayer::initialize(2, 3);

    let mut relu1 = relu::new();

    let mut layer2 = BasicLayer::initialize(3, 3);

    let mut optimizer = Stochastic::new(1.0, 1e-4, 0.9);

    println!("----------------------Setup--------------------------\n");

    println!("{}\n", &first);

    println!("{}\n", &layer1.neurons);

    println!("{}\n", &layer2.neurons);

    //----------------------------------------------------------------------------------------
    //Forward

    println!("----------------------forward------------------------\n");

    layer1.call(&first);

    println!("Layer1 output: {}\n", &layer1.output.clone().unwrap());

    relu1.forward(&layer1.output.clone().unwrap());

    println!("relu1 output: {}\n", &relu1.output.clone().unwrap());

    layer2.call(relu1.output.as_ref().unwrap());

    println!("Layer2 output: {}\n", &layer2.output.clone().unwrap());

    let mut pred = batch_softmax(layer2.output.clone().unwrap());

    let loss = batch_loss(classes, &pred);

    let acc = batch_accuracy(classes, &pred);

    println!("\n{:?}\n", &pred);

    println!("loss: {:?}| Acc: {:?}", loss, acc);

    //----------------------------------------------------------------------------------------
    //Backward
    println!("----------------------backward-----------------------\n");

    let softmax_deriv = batch_derivative(&mut pred, classes);

    println!("Softmax derivative: {}", softmax_deriv.clone());

    layer2.backwards(&softmax_deriv);

    relu1.backwards(&layer2.dinput.clone().unwrap());
    println!("\n{:?}\n", &relu1.dinput.clone().unwrap());
    println!("\n{:?}\n", &layer1.neurons.clone().transpose());
    layer1.backwards(&relu1.dinput.unwrap());
    println!("----");

    optimizer.update_learning_param();
    optimizer.update_weights(&mut layer1);
    optimizer.update_weights(&mut layer2);
    optimizer.increment_iteration();

    println!("{}\n", &layer1.neurons);

    println!("{}\n", &layer2.neurons);
}
