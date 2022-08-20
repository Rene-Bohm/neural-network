use neuralnet::{
    neurons::*, output::*, state::*,
};
use nalgebra as na;

struct network{

    layer: na::dvector<na::dvector<StaticNeuron>>,
    output: na::dvector,

}

impl network{

    pub fn new(input_layer: na::dvector<na::dvector<StaticNeuron>>) -> self{

        layer = input_layer;
        output = na::dvector<f64>![];

    }

    pub fn call(input: na::dvector<f64>) -> na::dvector<f64>{

        layer_dimension = self.layer.shape();
        input_dimension = self.layer_dimension();

        for i in 0..layer_dimension {

            for j in 0..input_dimension{


            }

        } 

    }


}