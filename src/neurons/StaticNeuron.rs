use crate::{output::OutputFunction, state::StateFunction};
use nalgebra as alg;

pub struct StaticNeuron {
    pub z: f64,
    pub y: f64,
    pub output_function: Box<OutputFunction>,
    pub state_function: Box<StateFunction>,
}