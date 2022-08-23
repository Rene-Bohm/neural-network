use super::OutputFunction;
use std::f64;

#[derive(Clone)]
pub struct ZeroReLU;

impl OutputFunction for ZeroReLU {
    fn call(&self, state: f64) -> f64 {
        
        f64::max(state, 0.0)
    }
}

#[derive(Clone)]
pub struct ReLU;

impl OutputFunction for ReLU {
    fn call(&self, state: f64) -> f64 {
        f64::max(state, state*0.01)
    }
}

