// #![warn(missing_docs, missing_debug_implementations, unreachable_pub)]

pub mod neurons; 
/*{
    use crate::{output::OutputFunction, state::StateFunction};
    use nalgebra as alg;

    pub struct FireNeuron {}

    pub struct LeakyNeuron {
        pub z: f64,
        pub y: f64,
        pub output_function: Box<OutputFunction>,
        pub state_function: Box<StateFunction>,
    }

    impl LeakyNeuron {
        pub fn new() {}

        pub fn calc(&mut self, weights: alg::DVector<f64>, inputs: alg::DVector<f64>) {
            let f = &self.state_function;
            self.z = f(weights, inputs);

            self.y = self.output_function.calc(self.z)
        }
    }

    pub struct StaticNeuron {
        pub z: f64,
        pub y: f64,
        pub output_function: Box<OutputFunction>,
        pub state_function: Box<StateFunction>,
    }
}*/

pub mod output;
/* 
{
    pub trait OutputFunction {
        fn calc(&self, state: f64) -> f64;
    }

    pub struct Id {}

    impl OutputFunction for Id {
        fn calc(&self, z: f64) -> f64 {
            z
        }
    }

}
*/

pub mod state;
