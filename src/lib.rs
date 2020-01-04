//! Provides interfaces and implementations for logic constructs that return values without accepting arguments, either via closure captures, global state, calculation or otherwise.

#[cfg(test)]
mod tests;

use std::rc::Rc;

/// An interface for immediate evaluation (without caching) for any types and cached (lazy) evaluation for types that implement `Clone`, including those that implement [`Copy`], integers and floats being the simplest examples.
pub trait Evaluator {
    /// The type that the evaluator evaluates to.
    type Output;
    /// Evaluates and returns the result. Because evaluators may possibly involve caching, a mutable borrow of `self` is required. Thus all evaluators must be mutable.
    /// # Panics
    /// Panicking of this function is not defined or restricted. Refer to the documentation of implementing structs for more information.
    fn eval(&mut self) -> Self::Output;
}

/// An interface for cached (lazy) evaluation for types that do not implement `Clone` (note that `Clone` is a supertrait of `Copy`).
pub trait RcEvaluator {
    /// The type that the evaluator evaluates to.
    type Output;
    /// Evaluates and returns a reference counter to the result. Because evaluators may possibly involve caching, a mutable borrow of `self` is required. Thus all evaluators must be mutable.
    /// # Panics
    /// Panicking of this function is not defined or restricted. Refer to the documentation of implementing structs for more information.
    fn eval(&mut self) -> Rc<Self::Output>;
}

/// Implements immediate evaluation via closures.
pub struct ImmEval<Out, Cl>
where Cl: FnMut() -> Out {
    closure: Cl
}
impl<Out, Cl> From<Cl> for ImmEval<Out, Cl>
where Cl: FnMut() -> Out {
    /// Constructs the evaluator from the specified closure.
    fn from(cl: Cl) -> Self {
        ImmEval {closure: cl}
    }
}
impl<Out, Cl> Evaluator for ImmEval<Out, Cl>
where Cl: FnMut() -> Out {
    type Output = Out;
    /// Evaluates and returns the result. Because evaluators may possibly involve caching, a mutable borrow of `self` is required by the [`Evaluator`](trait.Evaluator.html) trait, though immediate evaluators do not involve any.
    /// # Panics
    /// Panicking of this function is not defined or restricted and depends on the closure.
    fn eval(&mut self) -> Self::Output {
        (self.closure)()
    }
}

/// Implements cached (lazy) evaluation **of clonable types** via closures.
pub struct Eval<Out, Cl>
where Out: Clone, Cl: FnMut() -> Out {
    closure: Cl,
    last: Option<Out>
}
impl<Out, Cl> Eval<Out, Cl>
where Out: Clone, Cl: FnMut() -> Out {
    /// Invalidates the cached value, if there is one. If the evaluator was never used after creation or was recently `flush()`ed, does nothing.
    pub fn flush(&mut self) {
        self.last = None;
    }
}
impl<Out, Cl> From<Cl> for Eval<Out, Cl>
where Out: Clone, Cl: FnMut() -> Out {
    /// Constructs the evaluator from the specified closure.
    fn from(cl: Cl) -> Self {
        Eval {closure: cl, last: None}
    }
}
impl<Out, Cl> Evaluator for Eval<Out, Cl>
where Out: Clone, Cl: FnMut() -> Out {
    type Output = Out;
    /// Evaluates and returns the result. **This includes the overhead of cloning the value, because the internal cache must be preserved. If you need to move the result every time it is evaluated, use [`ImmEval`].**
    ///
    /// Because caching evaluators, as their name suggests, store the cached value inside themselves, a mutable borrow of `self` is required.
    /// # Panics
    /// Panicking of this function is not defined or restricted and depends on the closure.
    fn eval(&mut self) -> Self::Output {
        if self.last.is_none() {
            self.last = Option::from((self.closure)());
        }
        self.last.clone().unwrap()
    }
}

/// Implements cached (lazy) evaluation of clonable **and** non-clonable types via closures.
pub struct RcEval<Out, Cl>
where Cl: FnMut() -> Out {
    closure: Cl,
    last: Option<Rc<Out>>
}
impl<Out, Cl> RcEval<Out, Cl>
where Cl: FnMut() -> Out {
    /// Disowns the cached value (destroying it if there are no other owners), if there is one. If the evaluator was never used after creation or was recently `flush()`ed, does nothing.
    pub fn flush(&mut self) {
        self.last = None;
    }
}
impl<Out, Cl> From<Cl> for RcEval<Out, Cl>
where Cl: FnMut() -> Out {
    /// Constructs the evaluator from the specified closure.
    fn from(cl: Cl) -> Self {
        RcEval {closure: cl, last: None}
    }
}
impl<Out, Cl> RcEvaluator for RcEval<Out, Cl>
where Cl: FnMut() -> Out {
    type Output = Out;
    /// Evaluates and returns an [`std::rc::Rc`] to the result.
    ///
    /// Because caching evaluators, as their name suggests, store the cached value inside themselves, a mutable borrow of `self` is required.
    /// # Panics
    /// Panicking of this function is not defined or restricted and depends on the closure.
    fn eval(&mut self) -> Rc<Self::Output> {
        if self.last.is_none() {
            self.last = Option::from(Rc::from((self.closure)()));
        }
        self.last.clone().unwrap()
    }
}
impl<Out, Cl> Evaluator for RcEval<Out, Cl>
where Out: Clone, Cl: FnMut() -> Out {
    type Output = Out;
    /// Evaluates and returns the result. **This includes the overhead of cloning the value, because the internal cache must be preserved. If you need to move the result every time it is evaluated, use [`ImmEval`].**
    ///
    /// Because caching evaluators, as their name suggests, store the cached value inside themselves, a mutable borrow of `self` is required.
    /// # Panics
    /// Panicking of this function is not defined or restricted and depends on the closure.
    fn eval(&mut self) -> Self::Output {
        if self.last.is_none() {
            self.last = Option::from(Rc::from((self.closure)()));
        }
        Out::clone(&self.last.clone().unwrap())
    }
}

/// Implements dummy evaluation of clonable **and** non-clonable types: every time the `eval()` method is called, a fixed value is returned.
pub struct DummyEval<Out> {
    value: Out
}
impl<Out> From<Out> for DummyEval<Out> {
    /// Constructs the evaluator from the specified dummy value.
    fn from(value: Out) -> Self {
        DummyEval {value}
    }
}
impl<Out> Evaluator for DummyEval<Out>
where Out: Clone {
    type Output = Out;
    /// Evaluates and returns the result. **This includes the overhead of cloning the value, because the fixed value must be preserved.**
    ///
    /// Because caching evaluators, as their name suggests, store the cached value inside themselves, a mutable borrow of `self` is required.
    /// # Panics
    /// May panic only if cloning of the inner value panics.
    fn eval(&mut self) -> Self::Output {
        self.value.clone()
    }
}/*
impl<Out> RcEvaluator for DummyEval<Out> {
    type Output = Out;
    /// Evaluates and returns an [`std::rc::Rc`] to the result. Because evaluators may possibly involve caching, a mutable borrow of `self` is required by the [`Evaluator`](trait.Evaluator.html) trait, though dummy evaluators obviously do not involve any, as no values are produced during evaluation.
    /// # Panics
    /// Guaranteed to never panic.
    fn eval(&mut self) -> Rc<Self::Output> {
        uuh dunno?
    }
}*/