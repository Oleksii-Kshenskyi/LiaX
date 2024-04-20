mod common;

extern crate liblia;
use common::assert_eval_error;
use liblia::errors::s;
use liblia::eval::evaluate_sexpr;

#[test]
fn arithmetic_ops_work() {
    assert_eq!(Ok(s("8")), evaluate_sexpr(s("(+ 1 2 5)")));

    assert_eq!(Ok(s("5")), evaluate_sexpr(s("(- 10 5)")));
    assert_eq!(Ok(s("-5")), evaluate_sexpr(s("(- 5)")));
    assert_eq!(Ok(s("7")), evaluate_sexpr(s("(- 10 1 1 1)")));
    assert_eq!(Ok(s("0")), evaluate_sexpr(s("(-)")));

    assert_eq!(Ok(s("0")), evaluate_sexpr(s("(* 10 2 3 0)")));
    assert_eq!(Ok(s("20")), evaluate_sexpr(s("(* 10 2)")));
    assert_eq!(Ok(s("10")), evaluate_sexpr(s("(* 10)")));
    assert_eq!(Ok(s("1")), evaluate_sexpr(s("(*)")));
    assert_eq!(Ok(s("1")), evaluate_sexpr(s("(* 1 1 1)")));
    assert_eq!(Ok(s("60")), evaluate_sexpr(s("(* 1 1 (* 3 2) (* 10))")));

    assert_eq!(Ok(s("5")), evaluate_sexpr(s("(/ 10 2)")));
    assert_eq!(Ok(s("1")), evaluate_sexpr(s("(/ 100 2 2 5 5)")));
    assert_eq!(Ok(s("1")), evaluate_sexpr(s("(/)")));
    assert_eq!(Ok(s("50")), evaluate_sexpr(s("(/ 50)")));
    assert_eq!(Ok(s("0")), evaluate_sexpr(s("(/ 2 50)")));
    assert_eq!(Ok(s("1")), evaluate_sexpr(s("(/ 50 (/ 50 2) (/ 2))")));
    assert!(assert_eval_error(evaluate_sexpr(s("(/ 2 0)"))));
}

// TODO: Write tests for (map), (list) and any other non-arithmetic builtins to be added.
