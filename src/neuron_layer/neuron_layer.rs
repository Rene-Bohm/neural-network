use crate::{neurons::*, state::*, output::*};

extern crate rand;
use rand::Rng;

    
pub enum OutputType {

    Id,
    Step,
    Fermi,
    Tangens,
    ZeroReLU,
    ReLU,
    Gauss,

}

pub enum StateType {

    Euklid,
    Manhatten,
    Min,
    Max,
    Scalar,

}

pub struct Layer {

    bias: (f64,f64),
    neurons: Vec<StaticNeuron>,

}

impl Layer{

    pub fn new_vec(_layer_input: Vec<StaticNeuron>, NBias:(f64,f64)) -> Self{

        Layer {neurons: _layer_input, bias: NBias}


    }

    pub fn new_uniform(num_neurons: i32, num_of_weights: i32, state_function:StateType, output_function: OutputType, OutPutParam: f64, LearnFactor:f64) -> Self{

        let mut rng = rand::thread_rng();

        let mut neuron_layer:Vec<StaticNeuron> = Vec::new();

        //-----------------------------------------------------

        let State: Box<dyn Fn(Vec<f64>, Vec<f64>) -> f64> = match state_function{

            StateType::Euklid => Box::new(Euklid),
            StateType::Scalar => Box::new(Scalar),
            StateType::Min => Box::new(Minimum),
            StateType::Max => Box::new(Maximum),
            StateType::Manhatten => Box::new(Manhattan),

        };

        //------------------------------------------------------

        let Output: Box<dyn OutputFunction> = match output_function{

            OutputType::Id => Box::new(Id),
            OutputType::Step => Box::new(Step{t: OutPutParam}),
            OutputType::Fermi => Box::new(Fermi{c: OutPutParam}),
            OutputType::Tangens => Box::new(Tangens),
            OutputType::ReLU => Box::new(ReLU),
            OutputType::ZeroReLU => Box::new(ZeroReLU),
            OutputType::Gauss => Box::new(Gauss{delta: OutPutParam}),

        };

        //------------------------------------------------------

        let NBias:(f64,f64) = (rng.gen_range(0.0,1.0),rng.gen_range(0.0,1.0));

        //------------------------------------------------------

        for i in 0..num_neurons{

            let mut weights: Vec<f64> = Vec::new();

            for j in 0..num_of_weights{

                weights.push(rng.gen_range(0.0,0.1));

            }

            neuron_layer.push(StaticNeuron {

                n: LearnFactor,
                z: 0.0,
                y: 0.0,
                weights: weights,
                output_function: Output.clone(),
                state_function: State,

            });

        }

        //------------------------------------------------------

        Layer {neurons: neuron_layer, bias: NBias}


    }

}