use std::collections::HashMap;
use crate::ast::*;

pub fn run(program: &Program) {
    let mut vars: HashMap<String, i64> = HashMap::new();

    let mut lines = program.lines.clone();
    lines.sort_by_key(|l| l.number);

    for line in &lines {
        match &line.statement {
            Statement::Let { var, value } => {
                let v = eval_int(value, &vars);
                vars.insert(var.clone(), v);
            }
            Statement::Print { value } => {
                print_value(value, &vars);
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

fn print_value(expr: &Expr, vars: &HashMap<String, i64>) {
    match expr {
        Expr::Integer(n) => println!("{}", n),
        Expr::StringLit(s) => println!("{}", s),
        Expr::Variable(name) => {
            let v = vars.get(name).unwrap_or(&0);
            println!("{}", v);
        }
    }
}
