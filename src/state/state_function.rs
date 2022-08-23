
pub struct State {

    state_function: Box<dyn Fn(Vec<f64>, Vec<f64>) -> f64>,

}

impl State {

    pub fn new(fun: Box<dyn Fn(Vec<f64>, Vec<f64>) -> f64>) -> Self{

        State {state_function: fun}

    }

}

impl StateFunction for State {
    fn call(&self, input: Vec<f64>) -> f64 {
        if state > self.t {
            1.0
        } else {
            0.0
        }
    }
}
