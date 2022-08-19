use crate::{output::OutputFunction, state::StateFunction};
pub struct StaticNeuron {
    pub z: f64,
    pub y: f64,
    pub output_function: Box<dyn OutputFunction>,
    pub state_function: Box<StateFunction>,
}
