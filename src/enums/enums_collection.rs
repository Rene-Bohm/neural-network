#[derive(Clone,Copy)]
pub enum OutputType {
    Id,
    Step,
    Fermi,
    Tangens,
    ZeroReLU,
    ReLU,
    Gauss,
}

#[derive(Clone,Copy)]
pub enum StateType {
    Euklid,
    Manhatten,
    Min,
    Max,
    Scalar,
}
