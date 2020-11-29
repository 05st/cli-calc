use crate::lexer::*;
use crate::parser::*;

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
                Operator::EXP => left_node.powf(right_node)
            }
        },
        ASTNode::UNA(x, y) => {
            let child: f64 = evaluate_ast(*y);
            match x {
                Operator::SUB => -child,
                _ => child // ASTNode::UNA can only be Operator::ADD or Operator::SUB
            }
        },
        ASTNode::FUN(x, y) => {
            let child: f64 = evaluate_ast(*y);
            match x.as_str() {
                "abs" => child.abs(),
                "sin" => child.sin(),
                "cos" => child.sin(),
                "tan" => child.tan(),
                "asin" => child.asin(),
                "acos" => child.acos(),
                "atan" => child.atan(),
                "ln" => child.ln(),
                "sqrt" => child.sqrt(),
                "cbrt" => child.cbrt(),
                "exp" => child.exp(),
                "floor" => child.floor(),
                "ceil" => child.ceil(),
                "round" => child.round(),
                "trunc" => child.trunc(),
                "fract" => child.fract(),
                "signum" => child.signum(),
                "sinh" => child.sinh(),
                "cosh" => child.cosh(),
                "tanh" => child.tanh(),
                "asinh" => child.asinh(),
                "acosh" => child.acosh(),
                "atanh" => child.atanh(),
                _ => child
            }
        },
        ASTNode::VAR(x) => {
            match x.as_str() {
                "pi" => std::f64::consts::PI,
                "e" => std::f64::consts::E,
                "tau" => std::f64::consts::TAU,
                "phi" => 1.6180339887498948482045868343656381f64,
                _ => 0f64
            }
        },
        // _ => 0f64
    }
}