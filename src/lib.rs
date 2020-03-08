//! Provides interfaces and implementations for logic constructs that return values without accepting arguments, either via closure captures, global state, calculation or otherwise.

extern crate alloc;

pub mod traits;
pub use traits::*;
pub mod immeval;
pub use immeval::ImmEval;
pub mod eval;
pub use eval::Eval;
pub mod rceval;
pub use rceval::RcEval;
pub mod dummyeval;
pub use dummyeval::DummyEval;

#[cfg(test)]
mod tests;