// Integration tests for the calc library.
// These test the public API only — just like an external consumer would.

use calc::{CalcError, Calculator};

#[test]
fn integration_add_sub() {
    let calc = Calculator::new();
    assert_eq!(calc.add(10.0, 5.0), 15.0);
    assert_eq!(calc.sub(10.0, 5.0), 5.0);
}

#[test]
fn integration_mul_div() {
    let calc = Calculator::new();
    assert_eq!(calc.mul(5.0, 4.0), 20.0);
    assert_eq!(calc.divide(20.0, 4.0).unwrap(), 5.0);
}

#[test]
fn integration_divide_by_zero() {
    let calc = Calculator::new();
    match calc.divide(1.0, 0.0) {
        Err(CalcError::DivideByZero) => {} // expected
        other => panic!("Expected DivideByZero, got {:?}", other),
    }
}

#[test]
fn integration_eval() {
    let calc = Calculator::new();
    assert_eq!(calc.eval_expression("2 + 3").unwrap(), 5.0);
    assert_eq!(calc.eval_expression("10 - 3").unwrap(), 7.0);
    assert_eq!(calc.eval_expression("6 * 7").unwrap(), 42.0);
    assert_eq!(calc.eval_expression("15 / 3").unwrap(), 5.0);
}

#[test]
fn integration_memory_workflow() {
    let mut calc = Calculator::new();

    // Realistic workflow
    calc.store(100.0);
    assert_eq!(calc.recall(), 100.0);

    let result = calc.add(calc.recall(), 50.0);
    calc.store(result);
    assert_eq!(calc.recall(), 150.0);

    calc.add_to_memory(25.0);
    assert_eq!(calc.recall(), 175.0);

    calc.clear_memory();
    assert_eq!(calc.recall(), 0.0);
}

#[test]
fn integration_operator_x_syntax() {
    let calc = Calculator::new();
    // 'x' should work as multiply too
    assert_eq!(calc.eval_expression("3 x 4").unwrap(), 12.0);
}
