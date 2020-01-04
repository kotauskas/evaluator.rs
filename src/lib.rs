#[cfg(test)]
mod tests;

use std::rc::Rc;

/// Trait for describing objects that can be used to evaluate something.
pub trait Evaluator {
    /// 
    type Output;
    fn eval(&mut self) -> Self::Output;
}

pub trait RcEvaluator {
    type Output;
    fn eval(&mut self) -> Rc<Self::Output>;
}

pub struct ImmEval<Out, Cl>
where Cl: FnMut() -> Out {
    closure: Cl
}
impl<Out, Cl> From<Cl> for ImmEval<Out, Cl>
where Cl: FnMut() -> Out {
    fn from(cl: Cl) -> Self {
        ImmEval {closure: cl}
    }
}
impl<Out, Cl> Evaluator for ImmEval<Out, Cl>
where Cl: FnMut() -> Out {
    type Output = Out;
    
    fn eval(&mut self) -> Self::Output {
        (self.closure)()
    }
}

pub struct Eval<Out, Cl>
where Out: Clone, Cl: FnMut() -> Out {
    closure: Cl,
    last: Option<Out>
}
impl<Out, Cl> Eval<Out, Cl>
where Out: Clone, Cl: FnMut() -> Out {
    pub fn flush(&mut self) {
        self.last = None;
    }
}
impl<Out, Cl> From<Cl> for Eval<Out, Cl>
where Out: Clone, Cl: FnMut() -> Out {
    fn from(cl: Cl) -> Self {
        Eval {closure: cl, last: None}
    }
}
impl<Out, Cl> Evaluator for Eval<Out, Cl>
where Out: Clone, Cl: FnMut() -> Out {
    type Output = Out;
    
    fn eval(&mut self) -> Self::Output {
        if self.last.is_none() {
            self.last = Option::from((self.closure)());
        }
        self.last.clone().unwrap()
    }
}

pub struct RcEval<Out, Cl>
where Cl: FnMut() -> Out {
    closure: Cl,
    last: Option<Rc<Out>>
}
impl<Out, Cl> RcEval<Out, Cl>
where Cl: FnMut() -> Out {
    pub fn flush(&mut self) {
        self.last = None;
    }
}
impl<Out, Cl> From<Cl> for RcEval<Out, Cl>
where Cl: FnMut() -> Out {
    fn from(cl: Cl) -> Self {
        RcEval {closure: cl, last: None}
    }
}
impl<Out, Cl> RcEvaluator for RcEval<Out, Cl>
where Cl: FnMut() -> Out {
    type Output = Out;
    
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
    
    fn eval(&mut self) -> Self::Output {
        if self.last.is_none() {
            self.last = Option::from(Rc::from((self.closure)()));
        }
        Out::clone(&self.last.clone().unwrap())
    }
}