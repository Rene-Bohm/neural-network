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
    //println!("----------------------Setup--------------------------\n");
    let data_set = spiral(100, 3);

    let data = &data_set.0;
    let classes = &data_set.1;

    let first = Matrix::from(data.clone()).transpose();

    let mut layer1 = BasicLayer::initialize(2, 64);

    let mut relu1 = relu::new();

    let mut layer2 = BasicLayer::initialize(64, 3);

    let mut optimizer = Stochastic::new(1.0, 1e-4, 0.9);

    optimizer.setup_momentum(&mut layer1);
    optimizer.setup_momentum(&mut layer2);

    let x2 = layer1.neurons.clone();
    //println!("{}\n", &layer1.neurons);

    let x3 = layer2.neurons.clone();
    //println!("{}\n", &layer2.neurons);

    //----------------------------------------------------------------------------------------
    //Forward
    //println!("----------------------forward------------------------\n");

    for i in 1..1000 {
        layer1.call(&first);

        //println!("Layer1 output: {}\n", &layer1.output.clone().unwrap());

        relu1.forward(&layer1.output.clone().unwrap());

        //println!("relu1 output: {}\n", &relu1.output.clone().unwrap());

        layer2.call(relu1.output.as_ref().unwrap());

        //println!("Layer2 output: {}\n", &layer2.output.clone().unwrap());

        let mut pred = batch_softmax(layer2.output.clone().unwrap());

        let loss = batch_loss(classes, &pred);

        let acc = batch_accuracy(classes, &pred);

        //----------------------------------------------------------------------------------------
        //Backward
        //i % 1000 == 0
        if i % 100 == 0 {
            println!(
                "
                    Epoch : {:?} \n
                    Acc   : {:?} \n
                    loss  : {:?} \n
                    lr    : {:?} \n",
                i,
                acc,
                loss,
                optimizer.current_learning_rate.clone()
            );
        }
        //println!("----------------------backward-----------------------\n");

        let softmax_deriv = batch_derivative(&mut pred, classes);

        layer2.backwards(&softmax_deriv);

        relu1.backwards(&layer2.dinput.clone().unwrap());

        layer1.backwards(&relu1.dinput.as_ref().unwrap());

        optimizer.update_learning_param();
        optimizer.update_weights(&mut layer1);
        optimizer.update_weights(&mut layer2);
        optimizer.increment_iteration();
    }

    println!("{}\n{}", x2, layer1.neurons);
    println!("\n");
    println!("{}\n{}", x3, layer2.neurons);
}
