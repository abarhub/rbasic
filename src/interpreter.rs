use std::collections::HashMap;
use crate::ast::*;

pub fn run(program: &Program) {
    let mut vars: HashMap<String, i64> = HashMap::new();

    for line in &program.lines {
        match &line.statement {
            Statement::Let { var, value } => {
                let v = eval_int(value, &vars);
                vars.insert(var.clone(), v);
            }
            Statement::Print { values } => {
                let parts: Vec<String> = values.iter()
                    .map(|e| format_value(e, &vars))
                    .collect();
                println!("{}", parts.join(" "));
            }
        }
    }
}

fn eval_int(expr: &Expr, vars: &HashMap<String, i64>) -> i64 {
    match expr {
        Expr::Integer(n) => *n,
        Expr::Variable(name) => *vars.get(name).unwrap_or(&0),
        Expr::StringLit(_) => panic!("Expected integer, got string"),
    }
}

fn format_value(expr: &Expr, vars: &HashMap<String, i64>) -> String {
    match expr {
        Expr::Integer(n) => n.to_string(),
        Expr::StringLit(s) => s.clone(),
        Expr::Variable(name) => vars.get(name).unwrap_or(&0).to_string(),
    }
}
