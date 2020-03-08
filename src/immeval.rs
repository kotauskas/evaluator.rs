use crate::traits::Evaluator;

/// Implements immediate evaluation via closures.
///
/// This type of evaluator performs no caching whatsoever.
pub struct ImmEval<Out, Cl: Fn() -> Out> {
    closure: Cl,
    _phantom: core::marker::PhantomData<Out>
}
impl<Out, Cl: Fn() -> Out> ImmEval<Out, Cl> {
    /// Creates the evaluator using the specified closure.
    ///
    /// Aliases the `From` trait implementation.
    #[inline]
    #[must_use]
    pub fn new(closure: Cl) -> Self {
        Self {closure, _phantom: core::marker::PhantomData}
    }
    /// Returns a reference to the underlying closure.
    ///
    /// You can subsequently clone the closure or even copy it using this reference.
    #[inline]
    #[must_use]
    pub fn closure(&self) -> &Cl {
        &self.closure
    }
}
impl<Out, Cl: Fn() -> Out> Evaluator for ImmEval<Out, Cl> {
    type Output = Out;
    /// Evaluates and returns the result.
    /// # Panics
    /// Panicking of this function is not defined or restricted and depends on the closure.
    fn eval(&self) -> Self::Output {
        (self.closure)()
    }
}
impl<Out, Cl: Fn() -> Out> From<Cl> for ImmEval<Out, Cl> {
    #[inline]
    #[must_use]
    fn from(closure: Cl) -> Self {
        Self {closure, _phantom: core::marker::PhantomData}
    }
}