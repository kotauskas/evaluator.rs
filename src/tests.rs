use super::*;

#[test]
fn imm_eval() {
    let mut counter = 0;
    {
        let mut eval = ImmEval::from(|| {
            counter += 1;
            counter
        });
        
        eval.eval();
        eval.eval();
    }
    assert_eq!(counter, 2);
}

#[test]
fn eval() {
    let mut counter = 0;
    {
        let mut eval = Eval::from(|| {
            counter += 1;
            counter
        });
        
        eval.eval();
        eval.eval();
        eval.flush();
        eval.eval();
    }
    assert_eq!(counter, 2);
}

#[test]
fn rc_eval() {
    let mut counter = 0;
    let mut first: String;
    let mut second: String;
    let mut third: String;
    let mut fourth: String;
    {
        let mut eval = Eval::from(|| {
            counter += 1;
            match counter {
                1 => String::from("One!"),
                2 => String::from("Two!"),
                _ => String::from("Other!")
            }
        });
        
        first = eval.eval();
        second = eval.eval();
        eval.flush();
        third = eval.eval();
        eval.flush();
        fourth = eval.eval();
    }
    assert_eq!(counter, 3);
    assert_eq!(first.as_str(), "One!");
    assert_eq!(second.as_str(), "One!");
    assert_eq!(third.as_str(), "Two!");
    assert_eq!(fourth.as_str(), "Other!");
}