pub mod Id;

pub mod Step;

pub mod OutputTypFun;


pub struct IdFunction{}


pub struct StepFunction{

    t: f64,

}

pub struct FermiFunction{

    c: f64,

}

pub struct TangensFunction{}

pub struct ReLUFunction{

    typ: Variant,

}

pub struct GaussFunction{

    variance: f64

}

pub enum Variant{

    Leaky,
    Zero,

}

pub enum OutputType{

    Id(IdFunction),
    Step(StepFunction),
    Fermi(FermiFunction),
    Tangens(TangensFunction),
    ReLU(ReLUFunction),
    Gauss(GaussFunction),

}

pub type OutputFunction = dyn Fn(f64) -> f64;