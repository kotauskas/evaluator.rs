/// An interface for immediate evaluation (without caching) for any types and cached (lazy) evaluation for types that implement [`Copy`], integers and floats being the simplest examples.
pub trait Evaluator {
    /// The type that the evaluator evaluates to.
    type Output;
    /// Evaluates and returns the result.
    /// # Panics
    /// Panicking of this function is not defined or restricted. Refer to the documentation of implementing structs for more information.
    fn eval(&self) -> Self::Output;
}

/// An interface for cached (lazy) evaluation for types that do not implement `Copy`.
pub trait RcEvaluator {
    /// The type that the evaluator evaluates to.
    type Output;
    /// Evaluates and returns a reference counter to the result.
    /// # Panics
    /// Panicking of this function is not defined or restricted. Refer to the documentation of implementing structs for more information.
    fn rc_eval(&self) -> alloc::rc::Rc<Self::Output>;
}