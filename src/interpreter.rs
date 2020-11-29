use crate::lexer::*;
use crate::parser::*;

fn factorial(num: f64) -> f64 {
    let mut result: f64 = 1f64;
    for n in 2..=(num as i64) {
        result *= n as f64;
    }
    result
}

pub fn evaluate_ast(node: ASTNode) -> f64 {
    match node {
        ASTNode::NUM(x) => x,
        ASTNode::BIN(x, y, z) => {
            let left_node: f64 = evaluate_ast(*y);
            let right_node: f64 = evaluate_ast(*z);
            match x {
                Operator::ADD => left_node + right_node,
                Operator::SUB => left_node - right_node,
                Operator::MUL => left_node * right_node,
                Operator::DIV => left_node / right_node,
                Operator::MOD => left_node % right_node,
                Operator::EXP => left_node.powf(right_node),
            }
        }
        ASTNode::UNA(x, y) => {
            let child: f64 = evaluate_ast(*y);
            match x {
                Operator::SUB => -child,
                _ => child, // ASTNode::UNA can only be Operator::ADD or Operator::SUB
            }
        }
        ASTNode::FUN(x, y) => {
            let args: Vec<f64> = y.into_iter().map(|a| evaluate_ast(*a)).collect();
            match x.as_str() {
                "abs" => args[0].abs(),
                "sin" => args[0].sin(),
                "cos" => args[0].sin(),
                "tan" => args[0].tan(),
                "asin" => args[0].asin(),
                "atan" => args[0].atan(),
                "ln" => args[0].ln(),
                "log" => args[0].log10(),
                "sqrt" => args[0].sqrt(),
                "cbrt" => args[0].cbrt(),
                "exp" => args[0].exp(),
                "floor" => args[0].floor(),
                "ceil" => args[0].ceil(),
                "round" => args[0].round(),
                "trunc" => args[0].trunc(),
                "factorial" => factorial(args[0]),
                "fract" => args[0].fract(),
                "sign" => args[0].signum(),
                "sinh" => args[0].sinh(),
                "cosh" => args[0].cosh(),
                "tanh" => args[0].tanh(),
                "asinh" => args[0].asinh(),
                "acosh" => args[0].acosh(),
                "atanh" => args[0].atanh(),
                "logn" => args[1].log(args[0]),
                "hypot" => args[0].hypot(args[1]),
                "max" => args[0].max(args[1]),
                "min" => args[0].min(args[1]),
                "deg" => args[0].to_degrees(),
                "rad" => args[0].to_radians(),
                "root" => args[1].powf(1f64 / args[0]),
                "pow" => args[0].powf(args[1]),
                "sum" => args.into_iter().sum(),
                _ => args[0],
            }
        }
        ASTNode::VAR(x) => match x.as_str() {
            "pi" => std::f64::consts::PI,
            "e" => std::f64::consts::E,
            "tau" => std::f64::consts::TAU,
            "phi" => 1.6180339887498948482045868343656381f64,
            _ => 0f64,
        },
        // _ => 0f64
    }
}
