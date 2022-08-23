pub struct State {
    state_function: Box<fn(Vec<f64>, Vec<f64>) -> f64>,
}

impl State {
    pub fn new(fun: Box<fn(Vec<f64>, Vec<f64>) -> f64>) -> Self {
        State {
            state_function: fun,
        }
    }
}
