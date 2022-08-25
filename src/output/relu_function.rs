use super::OutputFunction;
use std::f64;

#[derive(Clone)]
pub struct ZeroReLU;

impl OutputFunction for ZeroReLU {
    fn call(&self, state: f64) -> f64 {
        
        f64::max(state, 0.0)
    }

    fn derivative(&self, z: f64, y:f64) -> f64 {

        if z > 0.0 { 1.0 } else{ 0.0 }

    }
}

#[derive(Clone)]
pub struct ReLU;

impl OutputFunction for ReLU {
    fn call(&self, state: f64) -> f64 {
        f64::max(state, state*0.01)
    }

    fn derivative(&self, z: f64, y:f64) -> f64 {

        if z > 0.0 { 1.0 } else{ 0.01 }

    }
}

