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

mod softmax;
pub use softmax::softmax;

//-----------------------------------------------

pub trait OutputFunction: OutputFunctionClone {
    fn call(&self, state: f64) -> f64;
    fn derivative(&self, z: f64, y: f64) -> f64;
}

pub trait OutputFunctionClone {
    fn clone_box(&self) -> Box<dyn OutputFunction>;
}

impl<T> OutputFunctionClone for T
where
    T: 'static + OutputFunction + Clone,
{
    fn clone_box(&self) -> Box<dyn OutputFunction> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn OutputFunction> {
    fn clone(&self) -> Box<dyn OutputFunction> {
        self.clone_box()
    }
}
