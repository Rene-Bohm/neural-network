use super::OutputFunction;
use std::f64;


#[derive(Clone)]
pub struct Tangens;

impl OutputFunction for Tangens {
    
    fn call(&self, state: f64) -> f64 {

        let output = (f64::exp(state)-f64::exp(-state))/(f64::exp(state)+f64::exp(-state));   
        output
    }

    fn derivative(&self, z: f64, y:f64) -> f64 {

        1.0 - f64::powi(f64::tanh(z), 2 )
    }
}