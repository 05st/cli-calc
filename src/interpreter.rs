use crate::{
    lexer::*,
    parser::*,
};

#[derive(Debug, PartialEq)]
pub enum InterpreterResult {
    Number(f64),
    Bool(bool),
}

fn factorial(num: f64) -> f64 {
    let mut result: f64 = 1f64;
    for n in 2..=(num as i64) {
        result *= n as f64;
    }
    result
}

pub fn evaluate_ast(node: ASTNode) -> InterpreterResult {
    match node {
        ASTNode::NUM(x) => InterpreterResult::Number(x),
        ASTNode::BOOL(x) => InterpreterResult::Bool(x),
        ASTNode::BIN(x, y, z) => match x {
            Operator::EQ => {
                let left_node = match evaluate_ast(*y) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };

                let right_node = match evaluate_ast(*z) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };
                if left_node == right_node {
                    InterpreterResult::Bool(true)
                } else {
                    InterpreterResult::Bool(false)
                }
            }
            Operator::NEQ => {
                let left_node = match evaluate_ast(*y) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };

                let right_node = match evaluate_ast(*z) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };
                if left_node != right_node {
                    InterpreterResult::Bool(true)
                } else {
                    InterpreterResult::Bool(false)
                }
            }
            Operator::AND => {
                let left_node = match evaluate_ast(*y) {
                    InterpreterResult::Number(_) => panic!("expected a boolean, found a number"),
                    InterpreterResult::Bool(x) => {
                        x
                    }
                };

                let right_node = match evaluate_ast(*z) {
                    InterpreterResult::Number(_) => panic!("expected a boolean, found a number"),
                    InterpreterResult::Bool(x) => {
                        x
                    }
                };
                InterpreterResult::Bool(left_node && right_node)
            }
            Operator::ADD => {
                let left_node = match evaluate_ast(*y) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };

                let right_node = match evaluate_ast(*z) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };
                InterpreterResult::Number(left_node + right_node)
            }
            Operator::SUB => {
                let left_node = match evaluate_ast(*y) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };

                let right_node = match evaluate_ast(*z) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };
                InterpreterResult::Number(left_node - right_node)
            }
            Operator::MUL => {
                let left_node = match evaluate_ast(*y) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };

                let right_node = match evaluate_ast(*z) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };
                InterpreterResult::Number(left_node * right_node)
            }
            Operator::DIV => {
                let left_node = match evaluate_ast(*y) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };

                let right_node = match evaluate_ast(*z) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };
                InterpreterResult::Number(left_node / right_node)
            }
            Operator::MOD => {
                let left_node = match evaluate_ast(*y) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };

                let right_node = match evaluate_ast(*z) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };
                InterpreterResult::Number(left_node % right_node)
            }
            Operator::EXP => {
                let left_node = match evaluate_ast(*y) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };

                let right_node = match evaluate_ast(*z) {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                };
                InterpreterResult::Number(left_node.powf(right_node))
            }
        },
        ASTNode::UNA(x, y) => {
            let child: f64 = match evaluate_ast(*y) {
                InterpreterResult::Number(x) => x,
                InterpreterResult::Bool(x) => {
                    if x {
                        1_f64
                    } else {
                        0_f64
                    }
                }
            };
            match x {
                Operator::SUB => InterpreterResult::Number(-child),
                _ => InterpreterResult::Number(child), // ASTNode::UNA can only be Operator::ADD or Operator::SUB
            }
        }
        ASTNode::FUN(x, y) => {
            let args: Vec<f64> = y
                .into_iter()
                .map(evaluate_ast)
                .map(|i| match i {
                    InterpreterResult::Number(x) => x,
                    InterpreterResult::Bool(x) => {
                        if x {
                            1_f64
                        } else {
                            0_f64
                        }
                    }
                })
                .collect();
            match x.as_str() {
                "abs" => InterpreterResult::Number(args[0].abs()),
                "sin" => InterpreterResult::Number(args[0].sin()),
                "cos" => InterpreterResult::Number(args[0].sin()),
                "tan" => InterpreterResult::Number(args[0].tan()),
                "asin" => InterpreterResult::Number(args[0].asin()),
                "atan" => InterpreterResult::Number(args[0].atan()),
                "ln" => InterpreterResult::Number(args[0].ln()),
                "log" => InterpreterResult::Number(args[0].log10()),
                "sqrt" => InterpreterResult::Number(args[0].sqrt()),
                "cbrt" => InterpreterResult::Number(args[0].cbrt()),
                "exp" => InterpreterResult::Number(args[0].exp()),
                "floor" => InterpreterResult::Number(args[0].floor()),
                "ceil" => InterpreterResult::Number(args[0].ceil()),
                "round" => InterpreterResult::Number(args[0].round()),
                "trunc" => InterpreterResult::Number(args[0].trunc()),
                "factorial" => InterpreterResult::Number(factorial(args[0])),
                "fract" => InterpreterResult::Number(args[0].fract()),
                "sign" => InterpreterResult::Number(args[0].signum()),
                "sinh" => InterpreterResult::Number(args[0].sinh()),
                "cosh" => InterpreterResult::Number(args[0].cosh()),
                "tanh" => InterpreterResult::Number(args[0].tanh()),
                "asinh" => InterpreterResult::Number(args[0].asinh()),
                "acosh" => InterpreterResult::Number(args[0].acosh()),
                "atanh" => InterpreterResult::Number(args[0].atanh()),
                "logn" => InterpreterResult::Number(args[1].log(args[0])),
                "hypot" => InterpreterResult::Number(args[0].hypot(args[1])),
                "max" => InterpreterResult::Number(args[0].max(args[1])),
                "min" => InterpreterResult::Number(args[0].min(args[1])),
                "deg" => InterpreterResult::Number(args[0].to_degrees()),
                "rad" => InterpreterResult::Number(args[0].to_radians()),
                "root" => InterpreterResult::Number(args[1].powf(1f64 / args[0])),
                "pow" => InterpreterResult::Number(args[0].powf(args[1])),
                "sum" => InterpreterResult::Number(args.into_iter().sum()),
                _ => InterpreterResult::Number(args[0]),
            }
        }
        ASTNode::VAR(x) => match x.as_str() {
            "pi" => InterpreterResult::Number(std::f64::consts::PI),
            "e" => InterpreterResult::Number(std::f64::consts::E),
            "tau" => InterpreterResult::Number(std::f64::consts::TAU),
            "phi" => InterpreterResult::Number(1.618033988749895_f64),
            _ => InterpreterResult::Number(0f64),
        },
    }
}
