use core::cell::RefCell;
use alloc::rc::Rc;
use crate::traits::{Evaluator, RcEvaluator};

/// Implements cached (lazy) evaluation of clonable **and** non-clonable types via closures.
///
/// Because `RcEval` uses a `RefCell` under the hood, it's not `Sync`. Use a mutex if you want to share these between threads.
pub struct RcEval<Out, Cl: Fn() -> Out> {
    closure: Cl,
    last: RefCell<Option<Rc<Out>>>
}
impl<Out, Cl: Fn() -> Out> RcEval<Out, Cl> {
    /// Disowns the cached value, if there is one, destroying it if there are no other owners. If the evaluator was never used after creation or was recently
    /// flushed, does nothing.
    #[inline]
    pub fn flush(&self) {
        *self.last.borrow_mut() = None;
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
impl<Out, Cl: Fn() -> Out> From<Cl> for RcEval<Out, Cl> {
    /// Constructs the evaluator from the specified closure.
    fn from(closure: Cl) -> Self {
        RcEval {closure, last: RefCell::new(None)}
    }
}
impl<Out, Cl: Fn() -> Out> RcEvaluator for RcEval<Out, Cl> {
    type Output = Out;
    /// Evaluates and returns an `Rc` to the result.
    ///
    /// # Panics
    /// Panicking of this function is not defined or restricted and depends on the closure.
    fn rc_eval(&self) -> Rc<Self::Output> {
        let mut val = self.last.borrow_mut();
        if val.is_none() {
            *val = Option::from(Rc::from((self.closure)()));
        }
        match val.clone() {
            Some(s) => s,
            None => unsafe {core::hint::unreachable_unchecked()}
        }
    }
}
// We need the Copy bound here because non-Copy types cannot be moved out of an Rc.
impl<Out: Copy, Cl: Fn() -> Out> Evaluator for RcEval<Out, Cl> {
    type Output = Out;
    /// Evaluates and returns the result.
    ///
    /// # Panics
    /// Panicking of this function is not defined or restricted and depends on the closure.
    fn eval(&self) -> Self::Output {
        let mut val = self.last.borrow_mut();
        if val.is_none() {
            *val = Option::from(Rc::from((self.closure)()));
        }
        match val.clone() {
            Some(s) => *s,
            None => unsafe{core::hint::unreachable_unchecked()}
        }
    }
}