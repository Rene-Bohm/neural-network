use super::OutputFunction;
use std::f64;

pub struct Id;

impl OutputFunction for Id {
    fn call(&self, state: f64) -> f64 {
        state
    }
}
