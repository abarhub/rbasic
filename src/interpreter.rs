use std::collections::HashMap;
use std::io::{self, Write};
use crate::ast::*;

struct State {
    int_vars: HashMap<String, i64>,
    str_vars: HashMap<String, String>,
    str_dims: HashMap<String, usize>,
}

impl State {
    fn new() -> Self {
        State {
            int_vars: HashMap::new(),
            str_vars: HashMap::new(),
            str_dims: HashMap::new(),
        }
    }

    fn assign(&mut self, var: &str, value: &Expr) {
        if var.ends_with('$') {
            let s = self.eval_str(value);
            let s = if let Some(&max) = self.str_dims.get(var) {
                s.chars().take(max).collect()
            } else {
                s
            };
            self.str_vars.insert(var.to_string(), s);
        } else {
            let n = self.eval_int(value);
            self.int_vars.insert(var.to_string(), n);
        }
    }

    fn eval_int(&self, expr: &Expr) -> i64 {
        match expr {
            Expr::Integer(n) => *n,
            Expr::Variable(name) if !name.ends_with('$') => {
                *self.int_vars.get(name).unwrap_or(&0)
            }
            Expr::BinOp { op, left, right } => {
                match op {
                    Op::Add => self.eval_int(left) + self.eval_int(right),
                    Op::Sub => self.eval_int(left) - self.eval_int(right),
                    Op::Mul => self.eval_int(left) * self.eval_int(right),
                    Op::Div => self.eval_int(left) / self.eval_int(right),
                    Op::Mod => self.eval_int(left) % self.eval_int(right),
                    Op::Eq  => if self.eval_int(left) == self.eval_int(right) { -1 } else { 0 },
                    Op::Ne  => if self.eval_int(left) != self.eval_int(right) { -1 } else { 0 },
                    Op::Lt  => if self.eval_int(left) <  self.eval_int(right) { -1 } else { 0 },
                    Op::Gt  => if self.eval_int(left) >  self.eval_int(right) { -1 } else { 0 },
                    Op::Le  => if self.eval_int(left) <= self.eval_int(right) { -1 } else { 0 },
                    Op::Ge  => if self.eval_int(left) >= self.eval_int(right) { -1 } else { 0 },
                }
            }
            _ => panic!("Erreur de type : entier attendu"),
        }
    }

    fn eval_str(&self, expr: &Expr) -> String {
        match expr {
            Expr::StringLit(s) => s.clone(),
            Expr::Variable(name) if name.ends_with('$') => {
                self.str_vars.get(name).cloned().unwrap_or_default()
            }
            Expr::BinOp { op: Op::Add, left, right } => {
                self.eval_str(left) + &self.eval_str(right)
            }
            _ => panic!("Erreur de type : chaîne attendue"),
        }
    }

    // Détermine si une expression produit une chaîne (basé sur le membre gauche)
    fn is_string_expr(expr: &Expr) -> bool {
        match expr {
            Expr::StringLit(_) => true,
            Expr::Variable(name) => name.ends_with('$'),
            Expr::BinOp { op: Op::Add, left, .. } => Self::is_string_expr(left),
            _ => false,
        }
    }

    fn format_value(&self, expr: &Expr) -> String {
        if Self::is_string_expr(expr) {
            self.eval_str(expr)
        } else {
            self.eval_int(expr).to_string()
        }
    }
}

pub fn run(program: &Program) {
    run_with_output(program, &mut io::stdout());
}

pub fn run_with_output(program: &Program, output: &mut dyn Write) {
    let mut state = State::new();

    for line in &program.lines {
        match &line.statement {
            Statement::Dim { var, size } => {
                state.str_dims.insert(var.clone(), *size);
                state.str_vars.entry(var.clone()).or_insert_with(String::new);
            }
            Statement::Let { var, value } => {
                state.assign(var, value);
            }
            Statement::Print { values } => {
                let parts: Vec<String> = values.iter()
                    .map(|e| state.format_value(e))
                    .collect();
                writeln!(output, "{}", parts.join(" ")).unwrap();
            }
        }
    }
}
