# Evaluator
![Crates.io](https://img.shields.io/crates/v/evaluator)

Provides interfaces and implementations for logic constructs that return values without accepting arguments, either via closure captures, global state, calculation or otherwise.

## Usage
### Evaluator types
The three main evaluator types are `ImmEval`, `Eval` and `RcEval` (there's also `DummyEval`, which is rarely useful except for dynamically dispatched evaluator calls).

#### ImmEval
Evaluates `Sized` values using a closure without any caching.
#### Eval
Evaluates `Copy`-able values using an internal cache.
#### RcEval
Evaluates all kinds of values by returning a reference counter to the result. The most flexible yet the slowest type.
#### DummyEval
Takes a single `Clone`-able (remember, `Copy` implies `Clone`) object and returns it when evaluated. Useful if you have a `Box<dyn Evaluator>` and want to save a few cycles on the closure indirection if the evaluator which you placed there always returns the same value.

You can implement your own evaluators by implementing the `Evaluator` and `RcEvaluator` traits, which are responsible for the `eval()` methods.

### Creating evaluators
Evaluators are created using the `From` trait, taking a closure as the only argument. They accept closures (or any type that implements `Fn`, really) that returns the kind of value that the evaluator accepts as the output (`Sized` for `ImmEval`, `Copy` for `Eval` and any value for `RcEval`).