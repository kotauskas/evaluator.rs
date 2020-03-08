use crate::*;
use core::cell::Cell;

#[test]
fn imm_eval() {
    let counter = Cell::new(0);
    {
        let eval = ImmEval::from(|| {
            counter.set(counter.get() + 1);
            counter.get()
        });

        eval.eval();
        eval.eval();
    }
    assert_eq!(counter.get(), 2);
}

#[test]
fn eval() {
    let counter = Cell::new(0);
    {
        let eval = Eval::from(|| {
            counter.set(counter.get() + 1);
            counter.get()
        });

        eval.eval();
        eval.eval();
        eval.flush();
        eval.eval();
    }
    assert_eq!(counter.get(), 2);
}

#[test]
fn rc_eval() {
    let counter = Cell::new(0);
    let first: &str;
    let second: &str;
    let third: &str;
    let fourth: &str;
    {
        let eval = RcEval::from(|| {
            counter.set(counter.get() + 1);
            match counter.get() {
                1 => "One!",
                2 => "Two!",
                _ => "Other!"
            }
        });

        first = *eval.rc_eval();
        second = *eval.rc_eval();
        eval.flush();
        third = *eval.rc_eval();
        eval.flush();
        fourth = *eval.rc_eval();
    }
    assert_eq!(counter.get(), 3);
    assert_eq!(first, "One!");
    assert_eq!(second, "One!");
    assert_eq!(third, "Two!");
    assert_eq!(fourth, "Other!");
}