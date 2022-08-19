use super::OutputFunction;
use std::f64;

pub struct IdFunction;

impl OutputFunction for IdFunction {
    fn call(&self, state: f64) -> f64 {
        state
    }
}
