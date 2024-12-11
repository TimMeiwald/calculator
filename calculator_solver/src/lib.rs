mod calculator;
use calculator::{CalculatorContext, CalculatorError};
use calculator_parser::{
    BasicContext, Rules, Source, _var_name, grammar, Context, Key, RULES_SIZE,
};
use std::cell::RefCell;
pub fn calculate(source: &str) -> Result<i64, CalculatorError> {
    let string = source;
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let result: (bool, u32);
    let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
    {
        let executor = _var_name(Rules::Grammar, &context, grammar);
        result = executor(Key(0), &source, position);
    }
    println!("Result: {:?}", result);
    //context.borrow().print_cache();
    //context.borrow().print_publisher();
    let publisher = context.into_inner().get_publisher().clear_false();
    publisher.print(Key(0), Some(true));
    assert_eq!((result.0, result.1), (true, src_len));
    let result = CalculatorContext::new(string, &publisher).calculate();
    return match result {
        Err(err) => {
            println!("Error: {:?} : {}", err, err);
            Err(err)
        }
        Ok(val) => {
            println!("Result: {:?}", val);
            Ok(val)
        }
    };
}

mod tests {
    use calculator_parser::*;
    use core::cell::RefCell;
    use std::env;
    use std::fs::{canonicalize, read_to_string};

    use crate::calculate;
    use crate::CalculatorError;

    #[test]
    fn test() {
        let src = "1+5";
        let result = calculate(src);
        assert_eq!(result.unwrap(), 6);
    }
    #[test]
    fn test2() {
        let src = "10/0";
        let result = calculate(src);
        assert_eq!(result.err().unwrap(), CalculatorError::DivideByZero);
    }
    #[test]
    fn test3() {
        let src = "10/5";
        let result = calculate(src);
        assert_eq!(result.unwrap(), 2);
    }
    #[test]
    fn test7() {
        let src = "10/3";
        let result = calculate(src);
        assert_eq!(result.unwrap(), 3);
    }
    #[test]
    fn test4() {
        let src = "10.0/5";
        let result = calculate(src);
        assert_eq!(
            result,
            Err(CalculatorError::IntegerParseError("10.0".to_string()))
        );
    }

    #[test]
    fn test15() {
        let src = "10/3*9+8-3";
        let result = calculate(src);
        assert_eq!(result.unwrap(), 32);
    }
    #[test]
    fn test16() {
        let src = "10-3*9+8/3";
        let result = calculate(src);
        assert_eq!(result.unwrap(), -15);
    }

    #[test]
    fn test17() {
        let src = "5^2";
        let result = calculate(src);
        assert_eq!(result.unwrap(), 25);
    }

    #[test]
    fn test18() {
        let src = "(10/2+50)-3*9+8/3+(20/5-10)";
        let result = calculate(src);
        assert_eq!(result.unwrap(), 24);
    }
    #[test]
    fn test19() {
        let src = "(20-3)-5";
        let result = calculate(src);
        assert_eq!(result.unwrap(), 12);
    }
    #[test]
    fn test20() {
        let src = "5-(20-3)";
        let result = calculate(src);
        assert_eq!(result.unwrap(), -12);
    }
    #[test]
    fn test21() {
        let src = "5-(20)";
        let result = calculate(src);
        assert_eq!(result.unwrap(), -15);
    }
    #[test]
    fn test22() {
        let src = "(20-3)*5";
        let result = calculate(src);
        assert_eq!(result.unwrap(), 85);
    }
    #[test]
    fn test23() {
        let src = "5*(20-3)";
        let result = calculate(src);
        assert_eq!(result.unwrap(), 85);
    }
    #[test]
    fn test24() {
        let src = "5*(20)";
        let result = calculate(src);
        assert_eq!(result.unwrap(), 100);
    }
    #[test]
    fn test25() {
        let src = "5-((20-3)*5)";
        let result = calculate(src);
        assert_eq!(result.unwrap(), -80);
    }
    #[test]
    fn test26() {
        let src = "(10/2+50)-3*9+8/3^2+(20/5-10)";
        let result = calculate(src);
        assert_eq!(result.unwrap(), 22);
    }
    #[test]
    fn test27() {
        let src = "(10/2+(5*10))-3*9+8/3^2+(20/5-10)";
        let result = calculate(src);
        assert_eq!(result.unwrap(), 22);
    }
}
