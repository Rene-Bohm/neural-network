//!
//! re-export of common members.
//!

//
// # enums
//

pub use crate::enums::*;


//
// # State functions
//

pub use crate::state::Scalar;

pub use crate::state::Euklid;

pub use crate::state::Maximum;

pub use crate::state::Minimum;

pub use crate::state::Manhattan;

//
// # Output functions
//

pub use crate::output::Id;

pub use crate::output::Step;

pub use crate::output::Fermi;

pub use crate::output::Tangens;

pub use crate::output::ReLU;

pub use crate::output::ZeroReLU;

pub use crate::output::Gauss;

//
// # Neurons
//

pub use crate::neurons::LeakyNeuron;

pub use crate::neurons::StaticNeuron;

//
// # Layer
//

pub use crate::neuron_layer::Layer;

//
// # Network
//

pub use crate::network::Network;