use crate::linalg::Matrix;

use super::OutputFunction;
use std::f64;

#[derive(Clone)]
pub struct ZeroReLU;

impl OutputFunction for ZeroReLU {
    fn call(&self, state: f64) -> f64 {
        
        f64::max(state, 0.0)
    }

    fn derivative(&self, z: f64, y:f64) -> f64 {

        if z > 0.0 { 1.0 } else{ 0.0 }

    }
}

#[derive(Clone)]
pub struct ReLU;

impl OutputFunction for ReLU {
    fn call(&self, state: f64) -> f64 {
        f64::max(state, state*0.01)
    }

    fn derivative(&self, z: f64, y:f64) -> f64 {

        if z > 0.0 { 1.0 } else{ 0.01 }

    }
}

pub struct relu{

    pub input: Option<Matrix>,
    pub dinput: Option<Matrix>,
    pub output: Option<Matrix>,

}

impl relu{

    pub fn new() -> Self{

        relu { input: None, dinput: None, output: None }

    }

    pub fn forward(&mut self, input: &Matrix) -> (){

        self.input = Some(input.clone());

        self.output = Some(input.clone().map(|x| f64::max(x, 0.0)));

    }

}