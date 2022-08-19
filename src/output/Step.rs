use super::OutputFunction;
use std::f64;

pub struct StepFunction {
    t: f64,
}

impl StepFunction {
    pub fn new(t: f64) -> Self {
        Self { t }
    }
}

impl OutputFunction for StepFunction {
    fn call(&self, state: f64) -> f64 {
        if state > self.t {
            1.0
        } else {
            0.0
        }
    }
}
