use super::StepFunction;
use std::f64;

impl StepFunction{

    fn calc(&self, input: f64) -> f64{

        if(input>self.t){

            (1.0)

        }else{

            (0.0)

        }
    }
}