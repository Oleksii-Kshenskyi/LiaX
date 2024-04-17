mod common;

extern crate liblia;
use common::*;
use liblia::errors::s;
use liblia::eval::evaluate_sexpr;

#[test]
fn basic_sanity_checks() {
    assert_eq!(Ok(s("")), evaluate_sexpr(s("")));
    assert_eq!(Ok(s("()")), evaluate_sexpr(s("()")));
    assert_eq!(Ok(s("6")), evaluate_sexpr(s("(+ 3 3)")));
    assert_eq!(Ok(s("0")), evaluate_sexpr(s("(+ 0 0)")));
    assert_eq!(
        Ok(s("544")),
        evaluate_sexpr(s("(+ 1 2 3 4 5 6 7 8 9 10 69 420)"))
    );
}

#[test]
fn eval_understands_weird_spacing() {
    assert_eq!(Ok(s("")), evaluate_sexpr(s("      ")));
    assert_eq!(
        Ok(s("3000")),
        evaluate_sexpr(s("(      +       1000      2000    )       "))
    );
    assert_eq!(Ok(s("3000")), evaluate_sexpr(s("       (+ 1000 2000)")));
    assert_eq!(Ok(s("3000")), evaluate_sexpr(s("(      + 1000 2000)")));
    assert_eq!(Ok(s("3000")), evaluate_sexpr(s("( +     1000    2000)")));
    assert_eq!(Ok(s("3000")), evaluate_sexpr(s("( + 1000 2000     )  ")));
}

#[test]
fn only_s_exprs_and_atoms_are_valid() {
    assert_eq!(Ok(s("3")), evaluate_sexpr(s("3")));
    assert_eq!(Ok(s("69420")), evaluate_sexpr(s("69420")));
    assert!(assert_eval_error(evaluate_sexpr(s("var"))));
    assert!(assert_eval_error(evaluate_sexpr(s("var69"))));
    assert!(assert_eval_error(evaluate_sexpr(s("var_69"))));

    assert!(assert_lexing_error(evaluate_sexpr(s("."))));

    assert!(assert_parsing_error(evaluate_sexpr(s("("))));
    assert!(assert_parsing_error(evaluate_sexpr(s(")"))));
    assert!(assert_parsing_error(evaluate_sexpr(s("(+ 3 3) 3"))));
    assert!(assert_parsing_error(evaluate_sexpr(s("3 (+ 3 3)"))));
    assert!(assert_parsing_error(evaluate_sexpr(s("(+ 3 3"))));
}

#[test]
fn test_weird_arithmetic_op_corner_cases() {
    assert_eq!(Ok(s("0")), evaluate_sexpr(s("(+)")));
    assert_eq!(Ok(s("69")), evaluate_sexpr(s("(+ 69)")));
}

#[test]
#[should_panic = "attempt to add with overflow"]
fn panics_due_to_int_overflow() {
    evaluate_sexpr(format!("(+ {} {})", i64::MAX, i64::MAX)).unwrap();
}
