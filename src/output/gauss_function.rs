use super::OutputFunction;
use std::f64;


#[derive(Clone)]
pub struct Gauss {
    pub delta: f64,
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
    
    fn derivative(&self, z: f64, y:f64) -> f64 {
        
        (-1.0 * z * y)/(f64::powi(self.delta, 2))

    }
}
