use std::num::{self, ParseIntError};

use calculator_parser::{parse, BasicPublisher, Key, Node, Rules};
use thiserror::Error;
#[derive(Error, Debug, PartialEq)]
pub enum CalculatorError {
    #[error("Dividing by Zero is undefined!")]
    DivideByZero,
    #[error("Could not parse `{0}` as an integer.")]
    IntegerParseError(String),
}

pub struct CalculatorContext<'a> {
    source: &'a str,
    publisher: &'a BasicPublisher,
}

impl<'a> CalculatorContext<'a> {
    pub fn new(source: &'a str, publisher: &'a BasicPublisher) -> Self {
        CalculatorContext { source, publisher }
    }

    pub fn calculate(&self) -> Result<i64, CalculatorError> {
        let root_node = self.publisher.get_node(Key(0));
        debug_assert_eq!(root_node.rule, Rules::Grammar);
        let root_node_children = root_node.get_children();
        debug_assert_eq!(root_node_children.len(), 1);
        let grammar_node_key = root_node_children[0];
        let grammar_node = self.publisher.get_node(grammar_node_key);
        println!("\nCalculating; {:?}", self.source);
        for child in grammar_node.get_children() {
            let c = self.publisher.get_node(*child);
            match c.rule {
                Rules::expr_addsub => {
                    return self.expr_addsub(&c);
                }
                Rules::expr_divmul => {
                    return self.expr_divmul(&c);
                }
                e => {
                    panic!("Rule: {:?} should not be the roots child.", e)
                }
            }
        }

        panic!("Should not get here");
    }

    fn expr_addsub(&self, node: &Node) -> Result<i64, CalculatorError> {
        // println!("Expr Addsub");
        let current_node = node;
        debug_assert_eq!(current_node.rule, Rules::expr_addsub);
        let children = current_node.get_children();
        debug_assert_eq!(children.len(), 1);
        let lhs = children[0];
        let child = self.publisher.get_node(lhs);
        // println!("Child: Rule: {:?}", child.rule);
        return match child.rule {
            Rules::add_expr => self.expr_add(child),
            Rules::sub_expr => self.expr_sub(child),
            Rules::expr_divmul => self.expr_divmul(child),
            _ => {
                panic!("Should not happen")
            }
        };
    }
    fn expr_divmul(&self, node: &Node) -> Result<i64, CalculatorError> {
        // println!("Expr DivMul");
        let current_node = node;
        debug_assert_eq!(current_node.rule, Rules::expr_divmul);
        let children = current_node.get_children();
        debug_assert_eq!(children.len(), 1);
        let lhs = children[0];
        let child = self.publisher.get_node(lhs);
        return match child.rule {
            Rules::mult_expr => self.expr_mul(child),
            Rules::div_expr => self.expr_div(child),
            Rules::term => self.term(child),
            Rules::expr_exponentiation => self.expr_exponentiation(child),

            e => {
                panic!("Unexpected rule: {:?}", e);
            }
        };
    }

    fn expr_exponentiation(&self, node: &Node) -> Result<i64, CalculatorError>{
        let current_node = node;
        debug_assert_eq!(current_node.rule, Rules::expr_exponentiation);
        let children = current_node.get_children();
        debug_assert_eq!(children.len(), 1);
        let child = self.publisher.get_node(children[0]);
        return match child.rule {
            Rules::exponent_expr => {self.expr_exponent(child)}
            Rules::expr_parentheses => {self.expr_parenthesis_not_executor(child)}
            e => {panic!("Unexpected Rule: {:?}", e)}
        }
    }
    fn expr_parenthesis_not_executor(&self, node: &Node) -> Result<i64, CalculatorError>{
        let current_node = node;
        debug_assert_eq!(current_node.rule, Rules::expr_parentheses);
        let children = current_node.get_children();
        debug_assert_eq!(children.len(), 1);
        let child = self.publisher.get_node(children[0]);
        return match child.rule {
            Rules::term => {self.term(child)}
            Rules::parentheses_expr => {self.expr_parentheses(child)}
            e => {panic!("Unexpected Rule: {:?}", e)}
        }
    }
    fn expr_parentheses(&self, node: &Node) -> Result<i64, CalculatorError>{
        let current_node = node;
        debug_assert_eq!(current_node.rule, Rules::parentheses_expr);
        let children = current_node.get_children();
        debug_assert_eq!(children.len(), 1);
        let child = self.publisher.get_node(children[0]);
        return match child.rule {
            Rules::term => {self.term(child)}
            Rules::expr_addsub => {self.expr_addsub(child)}
            e => {panic!("Unexpected Rule: {:?}", e)}
        }
        

    }

    fn expr_exponent(&self, node: &Node) -> Result<i64, CalculatorError>{
        let current_node = node;
        debug_assert_eq!(current_node.rule, Rules::exponent_expr);
        let children = current_node.get_children();
        debug_assert_eq!(children.len(), 2);

        let lhs = self.publisher.get_node(children[0]);
        let lhs_val: Result<i64, CalculatorError> = match lhs.rule {
            e => {
                panic!("Unexpected Rule: {:?}", e)
            }
        };
        let rhs = self.publisher.get_node(children[1]);
        let rhs_val: Result<i64, CalculatorError> = match rhs.rule {
            e => {
                panic!("Unexpected Rule: {:?}", e)
            }
        };
        return match lhs_val {
            Ok(lhs_val) => match rhs_val {
                Ok(rhs_val) => {
                    let result = lhs_val.pow(rhs_val.try_into().expect("For now no float support"));
                    println!("Exponenting {:?} ^ {:?} = {:?}", lhs_val, rhs_val, result);
                    Ok(result)
                }
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        };

    }

    fn expr_add(&self, node: &Node) -> Result<i64, CalculatorError> {
        // println!("Expr Add");
        let current_node = node;
        debug_assert_eq!(current_node.rule, Rules::add_expr);
        let children = current_node.get_children();
        debug_assert_eq!(children.len(), 2);
        let lhs_val: Result<i64, CalculatorError>;
        let rhs_val: Result<i64, CalculatorError>;
        let lhs = self.publisher.get_node(children[0]);
        let lhs_val = match lhs.rule {
            Rules::expr_addsub => self.expr_addsub(lhs),
            _ => {
                panic!("Should not happen")
            }
        };
        let rhs = self.publisher.get_node(children[1]);
        let rhs_val = match rhs.rule {
            Rules::expr_divmul => self.expr_divmul(rhs),
            _ => {
                panic!("Should not happen")
            }
        };
        return match lhs_val {
            Ok(lhs_val) => match rhs_val {
                Ok(rhs_val) => {
                    let result = lhs_val + rhs_val;
                    println!("Adding {:?} + {:?} = {:?}", lhs_val, rhs_val, result);
                    Ok(result)
                }
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        };
    }
    fn expr_sub(&self, node: &Node) -> Result<i64, CalculatorError> {
        // println!("Expr Sub");

        let current_node = node;
        debug_assert_eq!(current_node.rule, Rules::sub_expr);
        let children = current_node.get_children();
        debug_assert_eq!(children.len(), 2);
        let lhs_val: Result<i64, CalculatorError>;
        let rhs_val: Result<i64, CalculatorError>;
        let lhs = self.publisher.get_node(children[0]);
        let lhs_val = match lhs.rule {
            Rules::expr_addsub => self.expr_addsub(lhs),
            _ => {
                panic!("Should not happen")
            }
        };
        let rhs = self.publisher.get_node(children[1]);
        let rhs_val = match rhs.rule {
            Rules::expr_divmul => self.expr_divmul(rhs),
            _ => {
                panic!("Should not happen")
            }
        };
        return match lhs_val {
            Ok(lhs_val) => match rhs_val {
                Ok(rhs_val) => {
                    let result = lhs_val - rhs_val;
                    println!("Subtracting {:?} - {:?} = {:?}", lhs_val, rhs_val, result);
                    Ok(result)
                }
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        };
    }
    fn expr_div(&self, node: &Node) -> Result<i64, CalculatorError> {
        // println!("Expr Div");

        let current_node = node;
        debug_assert_eq!(current_node.rule, Rules::div_expr);
        let children = current_node.get_children();
        debug_assert_eq!(children.len(), 2);
        let lhs_val: Result<i64, CalculatorError>;
        let rhs_val: Result<i64, CalculatorError>;
        let lhs = self.publisher.get_node(children[0]);
        let lhs_val = match lhs.rule {
            Rules::expr_divmul => self.expr_divmul(lhs),
            _ => {
                panic!("Should not happen")
            }
        };
        let rhs = self.publisher.get_node(children[1]);
        let rhs_val = match rhs.rule {
            Rules::term => self.term(rhs),
            _ => {
                panic!("Should not happen")
            }
        };
        return match rhs_val {
            Ok(rhs_val) => {
                if rhs_val == 0 {
                    Err(CalculatorError::DivideByZero)
                } else {
                    return match lhs_val {
                        Ok(lhs_val) => {
                            let result = lhs_val / rhs_val;
                            println!("Dividing: {:?} / {:?} = {:?}", lhs_val, rhs_val, result);
                            Ok(result)
                        }
                        Err(err) => Err(err),
                    };
                }
            }
            Err(err) => Err(err),
        };
    }
    fn expr_mul(&self, node: &Node) -> Result<i64, CalculatorError> {
        // println!("Expr Mul");

        let current_node = node;
        debug_assert_eq!(current_node.rule, Rules::mult_expr);
        let children = current_node.get_children();
        debug_assert_eq!(children.len(), 2);
        let lhs_val: Result<i64, CalculatorError>;
        let rhs_val: Result<i64, CalculatorError>;
        let lhs = self.publisher.get_node(children[0]);
        let lhs_val = match lhs.rule {
            Rules::expr_divmul => self.expr_divmul(lhs),
            _ => {
                panic!("Should not happen")
            }
        };
        let rhs = self.publisher.get_node(children[1]);
        let rhs_val = match rhs.rule {
            Rules::term => self.term(rhs),
            _ => {
                panic!("Should not happen")
            }
        };
        return match lhs_val {
            Ok(lhs_val) => match rhs_val {
                Ok(rhs_val) => {
                    let result = lhs_val * rhs_val;
                    println!("Multiplying: {:?} * {:?} = {:?}", lhs_val, rhs_val, result);
                    Ok(result)
                }
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        };
    }

    fn term(&self, node: &Node) -> Result<i64, CalculatorError> {
        // println!("Term");

        let terminal_string = node.get_string(self.source);
        //println!("Terminal: {:?}", terminal_string);
        let val: Result<i64, ParseIntError> = terminal_string.parse();
        return match val {
            Ok(val) => Ok(val),
            Err(err) => Err(CalculatorError::IntegerParseError(
                terminal_string.to_string(),
            )),
        };
    }
}
