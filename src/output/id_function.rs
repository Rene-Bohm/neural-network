use super::OutputFunction;
use std::f64;

#[derive(Clone)]
pub struct Id;

impl OutputFunction for Id {
    fn call(&self, state: f64) -> f64 {
        state
    }
}
