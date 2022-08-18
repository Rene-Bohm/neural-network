pub mod neurons{
    use crate::{output::OutputFunction, state::StateFunction};


    pub struct FireNeuron{
    }

    pub struct LeakyNeuron{

        pub z: f64,
        pub y: f64,
        output_function: Box<dyn OutputFunction>,
        state_function: Box<dyn StateFunction>,

    }

    pub struct StaticNeuron{

        pub z: f64,
        pub y: f64,
        output_function: Box<dyn OutputFunction>,
        state_function: Box<dyn StateFunction>,

    }

}

pub mod output{
pub trait OutputFunction{

    fn calc(&self);

}

pub struct Id{}

pub struct Step{
    pub t: f64,
}

pub struct Fermi{
    pub c: f64,
}

pub struct Tangen{}

pub struct RelU{}

pub struct Gauss{}

pub struct RBF{}

pub struct EBF{}

}

pub mod state{
    
    pub trait StateFunction{

    fn calc(&self);

    }

pub struct ScalarProd{}

pub struct EuklidDist{}

pub struct MaxDist{}

pub struct MinDist{}

pub struct ManhattanDist{}

pub struct MahalanobisDist{}

}