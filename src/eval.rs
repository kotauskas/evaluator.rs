use core::cell::Cell;
use crate::traits::Evaluator;

/// Implements cached (lazy) evaluation **of copyable types** via closures. Use `RcEval` if you need to evaluate clonable non-`Copy` types.
pub struct Eval<Out: Copy, Cl: Fn() -> Out> {
    closure: Cl,
    last: Cell<Option<Out>>
}
impl<Out: Copy, Cl: Fn() -> Out> Eval<Out, Cl> {
    /// Creates the evaluator using the specified closure.
    ///
    /// Aliases the `From` trait implementation.
    #[inline]
    #[must_use]
    pub fn new(closure: Cl) -> Self {
        Self {closure, last: Cell::new(None)}
    }

    /// Invalidates the cached value, if there is one. If the evaluator was never used after creation or was recently flushed, does nothing.
    #[inline]
    pub fn flush(&self) {
        self.last.set(None);
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
impl<Out: Copy, Cl: Fn() -> Out> From<Cl> for Eval<Out, Cl> {
    /// Constructs the evaluator from the specified closure.
    #[inline]
    #[must_use]
    fn from(closure: Cl) -> Self {
        Self {closure, last: Cell::new(None)}
    }
}
impl<Out: Copy, Cl: Fn() -> Out> Evaluator for Eval<Out, Cl> {
    type Output = Out;
    /// Evaluates and returns the result.
    ///
    /// # Panics
    /// Panicking of this function is not defined or restricted and depends on the closure.
    fn eval(&self) -> Self::Output {
        let mut val = self.last.get();
        if val.is_none() {
            val = Some((self.closure)());
            self.last.set(val);
        }
        match val {
            Some(s) => s,
            None => unsafe {core::hint::unreachable_unchecked()} // should've used a mutex bruh
        }
    }
}