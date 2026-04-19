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
            Expr::UnaryOp { op, operand } => match op {
                UnaryOp::Neg => -self.eval_int(operand),
                UnaryOp::Pos =>  self.eval_int(operand),
                UnaryOp::Not => !self.eval_int(operand),
            },
            Expr::BinOp { op, left, right } => match op {
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
                Op::And => self.eval_int(left) & self.eval_int(right),
                Op::Or  => self.eval_int(left) | self.eval_int(right),
                Op::Xor => self.eval_int(left) ^ self.eval_int(right),
            },
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

struct ForFrame {
    var: String,
    to: i64,
    step: i64,
    body_start: usize, // PC of the line after FOR
}

fn find_target(lines: &[Line], target: &JumpTarget) -> usize {
    match target {
        JumpTarget::LineNumber(n) => {
            lines.iter().position(|l| l.number == Some(*n))
                .unwrap_or_else(|| panic!("Numéro de ligne introuvable : {}", n))
        }
        JumpTarget::Label(name) => {
            lines.iter().position(|l| matches!(&l.statement, Statement::Label(s) if s == name))
                .unwrap_or_else(|| panic!("Label introuvable : {}", name))
        }
    }
}

fn find_matching_next(lines: &[Line], for_pc: usize) -> usize {
    let var = match &lines[for_pc].statement {
        Statement::For { var, .. } => var.clone(),
        _ => panic!("find_matching_next appelé sur autre chose qu'un FOR"),
    };
    let mut depth = 0usize;
    for i in (for_pc + 1)..lines.len() {
        match &lines[i].statement {
            Statement::For { .. } => depth += 1,
            Statement::Next { var: v } => {
                if depth == 0 {
                    if v.as_deref().map_or(true, |n| n == var) {
                        return i;
                    }
                } else {
                    depth -= 1;
                }
            }
            _ => {}
        }
    }
    panic!("NEXT sans FOR correspondant pour {}", var);
}

fn find_matching_wend(lines: &[Line], while_pc: usize) -> usize {
    let mut depth = 0usize;
    for i in (while_pc + 1)..lines.len() {
        match &lines[i].statement {
            Statement::While { .. } => depth += 1,
            Statement::Wend => {
                if depth == 0 {
                    return i;
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    panic!("WEND sans WHILE correspondant");
}

fn exec_stmt(
    stmt: &Statement,
    pc: usize,
    lines: &[Line],
    state: &mut State,
    for_stack: &mut Vec<ForFrame>,
    while_stack: &mut Vec<usize>,
    call_stack: &mut Vec<usize>,
    output: &mut dyn Write,
) -> usize {
    match stmt {
        Statement::Rem => pc + 1,
        Statement::Label(_) => pc + 1,

        Statement::Dim { var, size } => {
            state.str_dims.insert(var.clone(), *size);
            state.str_vars.entry(var.clone()).or_insert_with(String::new);
            pc + 1
        }

        Statement::Let { var, value } => {
            state.assign(var, value);
            pc + 1
        }

        Statement::Print { values } => {
            let parts: Vec<String> = values.iter()
                .map(|e| state.format_value(e))
                .collect();
            writeln!(output, "{}", parts.join(" ")).unwrap();
            pc + 1
        }

        Statement::Goto(target) => find_target(lines, target),

        Statement::If { cond, then_stmt, else_stmt } => {
            if state.eval_int(cond) != 0 {
                exec_stmt(then_stmt, pc, lines, state, for_stack, while_stack, call_stack, output)
            } else if let Some(e) = else_stmt {
                exec_stmt(e, pc, lines, state, for_stack, while_stack, call_stack, output)
            } else {
                pc + 1
            }
        }

        Statement::For { var, from, to, step } => {
            let from_val = state.eval_int(from);
            let to_val = state.eval_int(to);
            let step_val = step.as_ref().map_or(1, |s| state.eval_int(s));
            state.int_vars.insert(var.clone(), from_val);

            let next_pc = find_matching_next(lines, pc);
            // Check loop condition before entering
            let done = if step_val >= 0 { from_val > to_val } else { from_val < to_val };
            if done {
                return next_pc + 1;
            }

            for_stack.push(ForFrame {
                var: var.clone(),
                to: to_val,
                step: step_val,
                body_start: pc + 1,
            });
            pc + 1
        }

        Statement::Next { var } => {
            let frame = for_stack.last_mut()
                .unwrap_or_else(|| panic!("NEXT sans FOR"));
            if let Some(v) = var {
                if *v != frame.var {
                    panic!("NEXT {} ne correspond pas au FOR {}", v, frame.var);
                }
            }
            let new_val = *state.int_vars.get(&frame.var).unwrap_or(&0) + frame.step;
            let done = if frame.step >= 0 {
                new_val > frame.to
            } else {
                new_val < frame.to
            };
            if done {
                for_stack.pop();
                pc + 1
            } else {
                state.int_vars.insert(frame.var.clone(), new_val);
                frame.body_start
            }
        }

        Statement::While { cond } => {
            if state.eval_int(cond) != 0 {
                while_stack.push(pc);
                pc + 1
            } else {
                find_matching_wend(lines, pc) + 1
            }
        }

        Statement::Wend => {
            let while_pc = while_stack.pop()
                .unwrap_or_else(|| panic!("WEND sans WHILE"));
            match &lines[while_pc].statement {
                Statement::While { cond } => {
                    if state.eval_int(cond) != 0 {
                        while_stack.push(while_pc);
                        while_pc + 1
                    } else {
                        pc + 1
                    }
                }
                _ => panic!("WEND : PC while ne pointe pas sur WHILE"),
            }
        }

        Statement::Gosub(target) => {
            call_stack.push(pc + 1);
            find_target(lines, target)
        }

        Statement::Return => {
            call_stack.pop()
                .unwrap_or_else(|| panic!("RETURN sans GOSUB"))
        }
    }
}

pub fn run(program: &Program) {
    run_with_output(program, &mut io::stdout());
}

pub fn run_with_output(program: &Program, output: &mut dyn Write) {
    let mut state = State::new();
    let mut for_stack: Vec<ForFrame> = Vec::new();
    let mut while_stack: Vec<usize> = Vec::new();
    let mut call_stack: Vec<usize> = Vec::new();
    let lines = &program.lines;

    let mut pc = 0usize;
    while pc < lines.len() {
        pc = exec_stmt(
            &lines[pc].statement,
            pc,
            lines,
            &mut state,
            &mut for_stack,
            &mut while_stack,
            &mut call_stack,
            output,
        );
    }
}
