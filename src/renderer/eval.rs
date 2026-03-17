use crate::parser::ast::*;
use std::collections::HashMap;

/// Environment for variable storage
pub type Env = HashMap<String, f64>;

/// Evaluate an expression to a f64 value
pub fn eval_expr(expr: &Expr, env: &Env) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::StringLit(_) => 0.0, // strings evaluate to 0 in numeric context
        Expr::Var(name) => *env.get(name).unwrap_or(&0.0),
        Expr::BinOp(left, op, right) => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            match op {
                Op::Add => l + r,
                Op::Sub => l - r,
                Op::Mul => l * r,
                Op::Div => if r != 0.0 { l / r } else { 0.0 },
                Op::Mod => if r != 0.0 { l % r } else { 0.0 },
                Op::Gt => if l > r { 1.0 } else { 0.0 },
                Op::Lt => if l < r { 1.0 } else { 0.0 },
                Op::Eq => if (l - r).abs() < f64::EPSILON { 1.0 } else { 0.0 },
                Op::Gte => if l >= r { 1.0 } else { 0.0 },
                Op::Lte => if l <= r { 1.0 } else { 0.0 },
                Op::NotEq => if (l - r).abs() >= f64::EPSILON { 1.0 } else { 0.0 },
            }
        }
        Expr::UnaryFunc(name, arg) => {
            let val = eval_expr(arg, env);
            match name.as_str() {
                "sin" => val.to_radians().sin(),
                "cos" => val.to_radians().cos(),
                "tan" => val.to_radians().tan(),
                "sqrt" => val.sqrt(),
                "abs" => val.abs(),
                "floor" => val.floor(),
                "ceil" => val.ceil(),
                "round" => val.round(),
                "log" => val.ln(),
                "min" => val, // single-arg min is identity
                "max" => val, // single-arg max is identity
                _ => {
                    eprintln!("Unknown function: {}", name);
                    val
                }
            }
        }
    }
}

/// Evaluate an expression to a string value (for string contexts)
pub fn eval_expr_string(expr: &Expr, env: &Env) -> String {
    match expr {
        Expr::StringLit(s) => s.clone(),
        Expr::Number(n) => format!("{}", n),
        Expr::Var(name) => {
            if let Some(v) = env.get(name) {
                format!("{}", v)
            } else {
                name.clone()
            }
        }
        _ => format!("{}", eval_expr(expr, env)),
    }
}

/// Check if an expression evaluates to truthy (non-zero)
pub fn eval_truthy(expr: &Expr, env: &Env) -> bool {
    eval_expr(expr, env) != 0.0
}
