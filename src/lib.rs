#![allow(unused)]
// #![warn(missing_docs, missing_debug_implementations, unreachable_pub)]

pub mod data;
pub mod enums;
pub mod help_func;
pub mod linalg;
pub mod loss_calc;
pub mod network;
pub mod neuron_layer;
pub mod neurons;
pub mod optimizer;
pub mod output;
pub mod state;

pub mod prelude;

mod rng;
pub use rng::*;
