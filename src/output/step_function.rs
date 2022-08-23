use super::OutputFunction;
use std::f64;

#[derive(Clone)]
pub struct Step {
    t: f64,
}

impl Step {
    pub fn new(t: f64) -> Self {
        Self { t }
    }
}

impl OutputFunction for Step {
    fn call(&self, state: f64) -> f64 {
        if state > self.t {
            1.0
        } else {
            0.0
        }
    }
}
