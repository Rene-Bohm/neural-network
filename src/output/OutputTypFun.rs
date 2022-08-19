use super::Id;
use super::OutputType;
use std::f64;

impl OutputType{

    fn calc(&self)-> f64{

        match self {

            OutputType::Id(Id) => Id.calc(self),
            
        }



    }


}