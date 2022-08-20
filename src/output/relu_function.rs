use super::OutputFunction;
use std::f64;

pub struct ZeroReLU;

impl OutputFunction for ZeroReLU {
    fn call(&self, state: f64) -> f64 {
        
        f64::max(state, 0.0)
    }
}

pub struct ReLU;

impl OutputFunction for ReLU {
    fn call(&self, state: f64) -> f64 {
        f64::max(state, state*0.01)
    }
}

