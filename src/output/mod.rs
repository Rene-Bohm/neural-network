mod id_function;
pub use id_function::Id;

mod step_function;
pub use step_function::Step;

mod fermi_function;
pub use fermi_function::Fermi;

mod tangens_function;
pub use tangens_function::Tangens;

mod relu_function;
pub use relu_function::ReLU;
pub use relu_function::ZeroReLU;

mod gauss_function;
pub use gauss_function::Gauss;

pub trait OutputFunction {
    fn call(&self, state: f64) -> f64;
}


pub enum OutputType {
    Id(Id),
    Step(Step),
    Fermi(Fermi),
    Tangens(Tangens),
    ReLU(ReLU),
    Gauss(Gauss),
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

