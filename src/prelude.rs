//!
//! re-export of common members.
//!

//
// # State functions
//

pub use crate::state::scalar_product::Scalar;

pub use crate::state::euklid_distance::Euklid;

pub use crate::state::maximum_distance::Maximum;

pub use crate::state::minimum_distance::Minimum;

pub use crate::state::manhattan_distance::Manhattan;

pub use crate::state::mahalanobis_distance;//todo

//
// # Output functions
//

pub use crate::output::id_function::Id;

pub use crate::output::step_function::Step;

pub use crate::output::fermi_function::Fermi;

pub use crate::output::tangens_function::Tangens;

pub use crate::output::relu_function::ReLU;

pub use crate::output::relu_function::ZeroReLU;

pub use crate::output::gauss_function::Gauss;

//
// # Neurons
//

pub use crate::neurons::leaky_neuron::LeakyNeuron;

pub use crate::neurons::static_neuron::StaticNeuron;