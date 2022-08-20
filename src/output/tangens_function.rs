use super::OutputFunction;
use std::f64;

pub struct Tangens;

impl OutputFunction for Tangens {
    fn call(&self, state: f64) -> f64 {

        let output = (f64::exp(state)-f64::exp(-state))/(f64::exp(state)+f64::exp(-state));   
        output
    }
}