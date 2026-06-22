// Library: Calculator struct with add, sub, mul, divide (Result), memory store/recall
//
// Full test suite in tests/ directory.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum CalcError {
    DivideByZero,
    Overflow,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::DivideByZero => write!(f, "division by zero"),
            CalcError::Overflow => write!(f, "numeric overflow"),
        }
    }
}

impl std::error::Error for CalcError {}

#[derive(Debug, Clone)]
pub struct Calculator {
    memory: f64,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { memory: 0.0 }
    }

    pub fn add(&self, a: f64, b: f64) -> f64 {
        a + b
    }

    pub fn sub(&self, a: f64, b: f64) -> f64 {
        a - b
    }

    pub fn mul(&self, a: f64, b: f64) -> f64 {
        a * b
    }

    pub fn divide(&self, a: f64, b: f64) -> Result<f64, CalcError> {
        if b == 0.0 {
            Err(CalcError::DivideByZero)
        } else {
            Ok(a / b)
        }
    }

    // Memory operations
    pub fn store(&mut self, value: f64) {
        self.memory = value;
    }

    pub fn recall(&self) -> f64 {
        self.memory
    }

    pub fn clear_memory(&mut self) {
        self.memory = 0.0;
    }

    pub fn add_to_memory(&mut self, value: f64) {
        self.memory += value;
    }

    // Chain operations
    pub fn eval_expression(&self, expr: &str) -> Result<f64, String> {
        let parts: Vec<&str> = expr.trim().split_whitespace().collect();
        if parts.len() != 3 {
            return Err("format: <a> <op> <b> (e.g. 10 + 3)".to_string());
        }

        let a: f64 = parts[0]
            .parse()
            .map_err(|_| format!("invalid number: '{}'", parts[0]))?;
        let b: f64 = parts[2]
            .parse()
            .map_err(|_| format!("invalid number: '{}'", parts[2]))?;

        match parts[1] {
            "+" => Ok(self.add(a, b)),
            "-" => Ok(self.sub(a, b)),
            "*" | "x" => Ok(self.mul(a, b)),
            "/" => self.divide(a, b).map_err(|e| e.to_string()),
            op => Err(format!("unknown operator: '{}'", op)),
        }
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_new_calculator() {
        let calc = Calculator::new();
        assert_eq!(calc.recall(), 0.0);
    }

    #[test]
    fn test_add() {
        let calc = Calculator::new();
        assert_eq!(calc.add(2.0, 3.0), 5.0);
        assert_eq!(calc.add(-1.0, 1.0), 0.0);
        assert_eq!(calc.add(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_sub() {
        let calc = Calculator::new();
        assert_eq!(calc.sub(10.0, 3.0), 7.0);
        assert_eq!(calc.sub(0.0, 5.0), -5.0);
    }

    #[test]
    fn test_mul() {
        let calc = Calculator::new();
        assert_eq!(calc.mul(4.0, 3.0), 12.0);
        assert_eq!(calc.mul(0.0, 5.0), 0.0);
        assert_eq!(calc.mul(-2.0, 3.0), -6.0);
    }

    #[test]
    fn test_divide() {
        let calc = Calculator::new();
        assert_eq!(calc.divide(10.0, 2.0).unwrap(), 5.0);
        assert_eq!(calc.divide(7.0, 2.0).unwrap(), 3.5);
    }

    #[test]
    fn test_divide_by_zero() {
        let calc = Calculator::new();
        let result = calc.divide(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CalcError::DivideByZero);
    }

    #[test]
    fn test_memory_store_recall() {
        let mut calc = Calculator::new();
        calc.store(42.0);
        assert_eq!(calc.recall(), 42.0);
    }

    #[test]
    fn test_memory_overwrite() {
        let mut calc = Calculator::new();
        calc.store(10.0);
        calc.store(20.0);
        assert_eq!(calc.recall(), 20.0);
    }

    #[test]
    fn test_clear_memory() {
        let mut calc = Calculator::new();
        calc.store(100.0);
        calc.clear_memory();
        assert_eq!(calc.recall(), 0.0);
    }

    #[test]
    fn test_add_to_memory() {
        let mut calc = Calculator::new();
        calc.store(5.0);
        calc.add_to_memory(3.0);
        assert_eq!(calc.recall(), 8.0);
        calc.add_to_memory(-2.0);
        assert_eq!(calc.recall(), 6.0);
    }

    #[test]
    fn test_eval_expression() {
        let calc = Calculator::new();
        assert_eq!(calc.eval_expression("3 + 4").unwrap(), 7.0);
        assert_eq!(calc.eval_expression("10 - 3").unwrap(), 7.0);
        assert_eq!(calc.eval_expression("4 * 5").unwrap(), 20.0);
        assert_eq!(calc.eval_expression("4 x 5").unwrap(), 20.0);
        assert_eq!(calc.eval_expression("10 / 2").unwrap(), 5.0);
    }

    #[test]
    fn test_eval_division_by_zero() {
        let calc = Calculator::new();
        let result = calc.eval_expression("10 / 0");
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_invalid_format() {
        let calc = Calculator::new();
        assert!(calc.eval_expression("3 +").is_err());
        assert!(calc.eval_expression("hello").is_err());
    }

    #[test]
    fn test_eval_unknown_operator() {
        let calc = Calculator::new();
        assert!(calc.eval_expression("3 ^ 4").is_err());
    }
}
