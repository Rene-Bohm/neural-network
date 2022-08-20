use super::OutputFunction;
use std::f64;

pub struct Gauss {
    delta: f64,
}

impl Gauss {
    pub fn new(delta: f64) -> Self {
        Self { delta }
    }
}

impl OutputFunction for Gauss {
    fn call(&self, state: f64) -> f64 {
        
        let output = f64::exp((-1.0)*(state*state)/(2.0*self.delta*self.delta));

        output

    }
}
