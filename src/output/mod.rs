mod id;
pub use id::IdFunction;

mod step;
pub use step::StepFunction;

pub mod OutputTypFun;

pub struct FermiFunction {
    c: f64,
}

pub struct TangensFunction {}

pub struct ReLUFunction {
    typ: Variant,
}

pub struct GaussFunction {
    variance: f64,
}

pub enum Variant {
    Leaky,
    Zero,
}

pub enum OutputType {
    Id(IdFunction),
    Step(StepFunction),
    Fermi(FermiFunction),
    Tangens(TangensFunction),
    ReLU(ReLUFunction),
    Gauss(GaussFunction),
}

impl OutputType {
    pub fn call(&self, state: f64) -> f64 {
        match self {
            Self::Id(id_struct) => id_struct.call(state),
            // Self::Step(step_struct) => step_struct.call(state),
            _ => panic!("BOOM"),
        }
    }
}

pub trait OutputFunction {
    fn call(&self, state: f64) -> f64;
}
