use crate::{lexer::*, parser::*};

#[derive(Debug, PartialEq, PartialOrd)]
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

fn perform_arithmetic_operator(left_result: InterpreterResult, right_result: InterpreterResult, operation: Box<dyn Fn(f64, f64) -> f64>) -> Result<f64, String> {
    if let (InterpreterResult::Number(left_value), InterpreterResult::Number(right_value)) = (left_result, right_result) {
        Ok(operation(left_value, right_value))
    } else {
        Err(String::from("Attempt to perform arithmetic operators on boolean values"))
    }
}

fn perform_logical_operator(left_result: InterpreterResult, right_result: InterpreterResult, operation: Box<dyn Fn(bool, bool) -> bool>) -> Result<bool, String> {
    if let (InterpreterResult::Bool(left_value), InterpreterResult::Bool(right_value)) = (left_result, right_result) {
        Ok(operation(left_value, right_value))
    } else {
        Err(String::from("Attempt to perform logical operators on non-boolean values"))
    }
}

pub fn evaluate_ast(node: ASTNode) -> Result<InterpreterResult, String> {
    match node {
        ASTNode::Number(value) => Ok(InterpreterResult::Number(value)),
        ASTNode::Bool(value) => Ok(InterpreterResult::Bool(value)),
        
        ASTNode::Binary(operator, left_node, right_node) => {
            let (left_result, right_result) = (evaluate_ast(*left_node)?, evaluate_ast(*right_node)?);
            match operator {
                Operator::Equal => Ok(InterpreterResult::Bool(left_result == right_result)),
                Operator::NotEqual => Ok(InterpreterResult::Bool(left_result != right_result)),
                Operator::Greater => Ok(InterpreterResult::Bool(left_result > right_result)),
                Operator::GreaterEqual => Ok(InterpreterResult::Bool(left_result >= right_result)),
                Operator::Lesser => Ok(InterpreterResult::Bool(left_result < right_result)),
                Operator::LesserEqual => Ok(InterpreterResult::Bool(left_result <= right_result)),

                Operator::And => Ok(InterpreterResult::Bool(perform_logical_operator(left_result, right_result, Box::new(|a, b| a && b))?)),
                Operator::Or => Ok(InterpreterResult::Bool(perform_logical_operator(left_result, right_result, Box::new(|a, b| a || b))?)),

                Operator::Add => Ok(InterpreterResult::Number(perform_arithmetic_operator(left_result, right_result, Box::new(|a, b| a + b))?)),
                Operator::Subtract => Ok(InterpreterResult::Number(perform_arithmetic_operator(left_result, right_result, Box::new(|a, b| a - b))?)),
                Operator::Multiply => Ok(InterpreterResult::Number(perform_arithmetic_operator(left_result, right_result, Box::new(|a, b| a * b))?)),
                Operator::Divide => Ok(InterpreterResult::Number(perform_arithmetic_operator(left_result, right_result, Box::new(|a, b| a / b))?)),
                Operator::Modulo => Ok(InterpreterResult::Number(perform_arithmetic_operator(left_result, right_result, Box::new(|a, b| a % b))?)),
                Operator::Exponent => Ok(InterpreterResult::Number(perform_arithmetic_operator(left_result, right_result, Box::new(|a, b| a.powf(b)))?)),

                _ => Err(String::from("Invalid binary operator"))
            }
        }

        ASTNode::Unary(operator, operand_node) => {
            let operand_result = evaluate_ast(*operand_node)?;
            match operator {
                Operator::Subtract => {
                    if let InterpreterResult::Number(value) = operand_result {
                        Ok(InterpreterResult::Number(-value))
                    } else {
                        Err(String::from("Attempt to perform arithmetic negation on boolean"))
                    }
                },
                Operator::Not => {
                    if let InterpreterResult::Bool(value) = operand_result {
                        Ok(InterpreterResult::Bool(!value))
                    } else {
                        Err(String::from("Attempt to perform logical not on number"))
                    }
                }
                _ => Ok(operand_result)
            }
        }

        ASTNode::Function(function_name, arg_nodes) => {
            let mut args: Vec<f64> = Vec::new();

            for arg_node in arg_nodes.iter() {
                let arg_result = evaluate_ast(arg_node.clone())?;
                if let InterpreterResult::Number(value) = arg_result {
                    args.push(value);
                } else {
                    return Err(String::from("Attempt to apply mathematical function on boolean"))
                }
            }

            match function_name.as_str() {
                "abs" => Ok(InterpreterResult::Number(args[0].abs())),
                "sin" => Ok(InterpreterResult::Number(args[0].sin())),
                "cos" => Ok(InterpreterResult::Number(args[0].cos())),
                "tan" => Ok(InterpreterResult::Number(args[0].tan())),
                "asin" => Ok(InterpreterResult::Number(args[0].asin())),
                "atan" => Ok(InterpreterResult::Number(args[0].atan())),
                "atan2" => Ok(InterpreterResult::Number(args[0].atan2(args[1]))),
                "ln" => Ok(InterpreterResult::Number(args[0].ln())),
                "log10" => Ok(InterpreterResult::Number(args[0].log10())),
                "log2" => Ok(InterpreterResult::Number(args[0].log2())),
                "sqrt" => Ok(InterpreterResult::Number(args[0].sqrt())),
                "cbrt" => Ok(InterpreterResult::Number(args[0].cbrt())),
                "exp" => Ok(InterpreterResult::Number(args[0].exp())),
                "floor" => Ok(InterpreterResult::Number(args[0].floor())),
                "ceil" => Ok(InterpreterResult::Number(args[0].ceil())),
                "round" => Ok(InterpreterResult::Number(args[0].round())),
                "trunc" => Ok(InterpreterResult::Number(args[0].trunc())),
                "fact" => Ok(InterpreterResult::Number(factorial(args[0]))),
                "fract" => Ok(InterpreterResult::Number(args[0].fract())),
                "sign" => Ok(InterpreterResult::Number(args[0].signum())),
                "sinh" => Ok(InterpreterResult::Number(args[0].sinh())),
                "cosh" => Ok(InterpreterResult::Number(args[0].cosh())),
                "tanh" => Ok(InterpreterResult::Number(args[0].tanh())),
                "asinh" => Ok(InterpreterResult::Number(args[0].asinh())),
                "acosh" => Ok(InterpreterResult::Number(args[0].acosh())),
                "atanh" => Ok(InterpreterResult::Number(args[0].atanh())),
                "log" => Ok(InterpreterResult::Number(args[1].log(args[0]))),
                "hypot" => Ok(InterpreterResult::Number(args[0].hypot(args[1]))),
                "max" => Ok(InterpreterResult::Number(args[0].max(args[1]))),
                "min" => Ok(InterpreterResult::Number(args[0].min(args[1]))),
                "deg" => Ok(InterpreterResult::Number(args[0].to_degrees())),
                "rad" => Ok(InterpreterResult::Number(args[0].to_radians())),
                "root" => Ok(InterpreterResult::Number(args[1].powf(1f64/args[0]))),
                "pow" => Ok(InterpreterResult::Number(args[0].powf(args[1]))),
                "sum" => Ok(InterpreterResult::Number(args.into_iter().sum())),
                _ => Ok(InterpreterResult::Number(args[0]))
            }
        }

        ASTNode::Variable(variable_name) => match variable_name.as_str() {
            "pi" => Ok(InterpreterResult::Number(std::f64::consts::PI)),
            "e" => Ok(InterpreterResult::Number(std::f64::consts::E)),
            "tau" => Ok(InterpreterResult::Number(std::f64::consts::TAU)),
            "phi" => Ok(InterpreterResult::Number(1.618033988749895_f64)),
            _ => Ok(InterpreterResult::Number(0f64))
        }
    }
}
