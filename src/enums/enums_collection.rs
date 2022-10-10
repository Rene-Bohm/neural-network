#[derive(Clone, Copy)]
pub enum OutputType {
    Id,
    Step,
    Fermi,
    Tangens,
    ZeroReLU,
    ReLU,
    Gauss,
}

#[derive(Clone, Copy)]
pub enum StateType {
    Euklid,
    Manhatten,
    Min,
    Max,
    Scalar,
}

/*

impl OutputType {
    pub fn call(&self, state: f64) -> f64 {
        match self {
            Self::Id(id_struct) => id_struct.call(state),
            // Self::Step(step_struct) => step_struct.call(state),
            _ => panic!("BOOM"),
        }
    }
}
*/
