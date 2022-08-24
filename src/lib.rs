// #![warn(missing_docs, missing_debug_implementations, unreachable_pub)]

pub mod network;
pub mod neuron_layer;
pub mod neurons;
pub mod output;
pub mod state;
pub mod enums;

pub mod prelude;

mod rng;
pub use rng::*;
