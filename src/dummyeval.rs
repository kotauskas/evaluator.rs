use crate::traits::Evaluator;

/// Implements dummy evaluation of clonable types: every time the `eval()` method is called, a fixed value is returned.
pub struct DummyEval<Out> {
    value: Out
}
impl<Out> DummyEval<Out> {
    /// Consumes the evaluator and returns the underlying value.
    pub fn into_inner(self) -> Out {
        self.value
    }
}
impl<Out> From<Out> for DummyEval<Out> {
    /// Constructs the evaluator from the specified dummy value.
    fn from(value: Out) -> Self {
        Self {value}
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
    fn eval(&self) -> Self::Output {
        self.value.clone()
    }
}/*
impl<Out> RcEvaluator for DummyEval<Out> {
    type Output = Out;
    /// Evaluates and returns an `Rc` to the result. Because evaluators may possibly involve caching, a mutable borrow of `self` is required by the [`Evaluator`](trait.Evaluator.html) trait, though dummy evaluators obviously do not involve any, as no values are produced during evaluation.
    /// # Panics
    /// Guaranteed to never panic.
    fn eval(&mut self) -> Rc<Self::Output> {
        uuh dunno?
    }
}*/