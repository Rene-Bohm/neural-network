use super::OutputFunction;
use std::f64;

#[derive(Clone)]
pub struct Fermi {
    c: f64,
}

impl Fermi{
    pub fn new(c: f64) -> Self {
        Self { c }
    }

}

impl OutputFunction for Fermi {
    fn call(&self, state: f64) -> f64 {

        let output = (1.0 + f64::exp((-self.c)*state)).powi(-1); 
        
        output
        
    }
    
    fn derivative(&self, z: f64, y:f64) -> f64 {

        self.c * y * (1.0 - y)

    }
}