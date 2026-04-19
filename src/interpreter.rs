use std::collections::HashMap;
use std::io::{self, Write};
use crate::ast::*;

struct ArrayData<T> {
    dims: Vec<usize>, // taille par dimension (max_index + 1)
    data: Vec<T>,
}

impl<T: Default + Clone> ArrayData<T> {
    fn new(max_indices: &[usize]) -> Self {
        let dims: Vec<usize> = max_indices.iter().map(|&n| n + 1).collect();
        let total: usize = dims.iter().product();
        ArrayData { dims, data: vec![T::default(); total] }
    }

    fn flat_index(&self, indices: &[i64]) -> usize {
        assert_eq!(indices.len(), self.dims.len(), "Nombre de dimensions incorrect");
        let mut idx = 0usize;
        for (&i, &size) in indices.iter().zip(self.dims.iter()) {
            assert!(i >= 0 && (i as usize) < size, "Indice hors limites : {}", i);
            idx = idx * size + i as usize;
        }
        idx
    }

    fn get(&self, indices: &[i64]) -> &T { &self.data[self.flat_index(indices)] }
    fn set(&mut self, indices: &[i64], val: T) { let i = self.flat_index(indices); self.data[i] = val; }
}

struct State {
    int_vars: HashMap<String, i64>,
    str_vars: HashMap<String, String>,
    str_dims: HashMap<String, usize>,
    int_arrays: HashMap<String, ArrayData<i64>>,
    str_arrays: HashMap<String, ArrayData<String>>,
}

impl State {
    fn new() -> Self {
        State {
            int_vars: HashMap::new(),
            str_vars: HashMap::new(),
            str_dims: HashMap::new(),
            int_arrays: HashMap::new(),
            str_arrays: HashMap::new(),
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

    fn int_builtin(&self, name: &str, args: &[Expr]) -> Option<i64> {
        match name {
            "LEN" => {
                assert_eq!(args.len(), 1, "LEN attend 1 argument");
                Some(self.eval_str(&args[0]).chars().count() as i64)
            }
            "ASC" => {
                assert_eq!(args.len(), 1, "ASC attend 1 argument");
                let s = self.eval_str(&args[0]);
                Some(s.chars().next().map_or(0, |c| c as i64))
            }
            "VAL" => {
                assert_eq!(args.len(), 1, "VAL attend 1 argument");
                let s = self.eval_str(&args[0]);
                Some(s.trim().parse::<i64>().unwrap_or(0))
            }
            "INSTR" => {
                match args.len() {
                    2 => {
                        let s = self.eval_str(&args[0]);
                        let sub = self.eval_str(&args[1]);
                        Some(match s.find(sub.as_str()) {
                            Some(pos) => s[..pos].chars().count() as i64 + 1,
                            None => 0,
                        })
                    }
                    3 => {
                        let start = (self.eval_int(&args[0]) - 1).max(0) as usize;
                        let s = self.eval_str(&args[1]);
                        let sub = self.eval_str(&args[2]);
                        let chars: Vec<char> = s.chars().collect();
                        let slice: String = chars[start.min(chars.len())..].iter().collect();
                        Some(match slice.find(sub.as_str()) {
                            Some(pos) => start as i64 + slice[..pos].chars().count() as i64 + 1,
                            None => 0,
                        })
                    }
                    _ => panic!("INSTR attend 2 ou 3 arguments"),
                }
            }
            "ABS" => {
                assert_eq!(args.len(), 1, "ABS attend 1 argument");
                Some(self.eval_int(&args[0]).abs())
            }
            "SGN" => {
                assert_eq!(args.len(), 1, "SGN attend 1 argument");
                let n = self.eval_int(&args[0]);
                Some(if n > 0 { 1 } else if n < 0 { -1 } else { 0 })
            }
            "SQR" => {
                assert_eq!(args.len(), 1, "SQR attend 1 argument");
                let n = self.eval_int(&args[0]);
                Some((n as f64).sqrt() as i64)
            }
            _ => None,
        }
    }

    fn str_builtin(&self, name: &str, args: &[Expr]) -> Option<String> {
        match name {
            "STR$" => {
                assert_eq!(args.len(), 1, "STR$ attend 1 argument");
                Some(self.eval_int(&args[0]).to_string())
            }
            "CHR$" => {
                assert_eq!(args.len(), 1, "CHR$ attend 1 argument");
                let n = self.eval_int(&args[0]) as u32;
                Some(char::from_u32(n).map_or(String::new(), |c| c.to_string()))
            }
            "SPACE$" => {
                assert_eq!(args.len(), 1, "SPACE$ attend 1 argument");
                let n = self.eval_int(&args[0]).max(0) as usize;
                Some(" ".repeat(n))
            }
            "LEFT$" => {
                assert_eq!(args.len(), 2, "LEFT$ attend 2 arguments");
                let s = self.eval_str(&args[0]);
                let n = self.eval_int(&args[1]).max(0) as usize;
                Some(s.chars().take(n).collect())
            }
            "RIGHT$" => {
                assert_eq!(args.len(), 2, "RIGHT$ attend 2 arguments");
                let s = self.eval_str(&args[0]);
                let n = self.eval_int(&args[1]).max(0) as usize;
                let chars: Vec<char> = s.chars().collect();
                let start = chars.len().saturating_sub(n);
                Some(chars[start..].iter().collect())
            }
            "MID$" => {
                assert!(args.len() == 2 || args.len() == 3, "MID$ attend 2 ou 3 arguments");
                let s = self.eval_str(&args[0]);
                let start = (self.eval_int(&args[1]) - 1).max(0) as usize;
                let chars: Vec<char> = s.chars().collect();
                let from = start.min(chars.len());
                Some(if args.len() == 2 {
                    chars[from..].iter().collect()
                } else {
                    let len = self.eval_int(&args[2]).max(0) as usize;
                    chars[from..].iter().take(len).collect()
                })
            }
            "UCASE$" => {
                assert_eq!(args.len(), 1, "UCASE$ attend 1 argument");
                Some(self.eval_str(&args[0]).to_uppercase())
            }
            "LCASE$" => {
                assert_eq!(args.len(), 1, "LCASE$ attend 1 argument");
                Some(self.eval_str(&args[0]).to_lowercase())
            }
            "LTRIM$" => {
                assert_eq!(args.len(), 1, "LTRIM$ attend 1 argument");
                Some(self.eval_str(&args[0]).trim_start().to_string())
            }
            "RTRIM$" => {
                assert_eq!(args.len(), 1, "RTRIM$ attend 1 argument");
                Some(self.eval_str(&args[0]).trim_end().to_string())
            }
            _ => None,
        }
    }

    fn eval_int(&self, expr: &Expr) -> i64 {
        match expr {
            Expr::Integer(n) => *n,
            Expr::Variable(name) if !name.ends_with('$') => {
                *self.int_vars.get(name).unwrap_or(&0)
            }
            Expr::ArrayAccess { name, indices } if !name.ends_with('$') => {
                if let Some(result) = self.int_builtin(name, indices) {
                    return result;
                }
                let idx: Vec<i64> = indices.iter().map(|e| self.eval_int(e)).collect();
                *self.int_arrays.get(name)
                    .unwrap_or_else(|| panic!("Tableau entier ou fonction {} non déclaré(e)", name))
                    .get(&idx)
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
            Expr::ArrayAccess { name, indices } if name.ends_with('$') => {
                if let Some(result) = self.str_builtin(name, indices) {
                    return result;
                }
                let idx: Vec<i64> = indices.iter().map(|e| self.eval_int(e)).collect();
                self.str_arrays.get(name)
                    .unwrap_or_else(|| panic!("Tableau chaîne ou fonction {} non déclaré(e)", name))
                    .get(&idx)
                    .clone()
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
            Expr::ArrayAccess { name, .. } => name.ends_with('$'),
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

struct ProcFrame {
    return_pc: usize,
    saved_int_vars: HashMap<String, i64>,
    saved_str_vars: HashMap<String, String>,
}

struct ForFrame {
    var: String,
    to: i64,
    step: i64,
    body_start: usize, // PC of the line after FOR
}

fn find_end_sub(lines: &[Line], sub_pc: usize) -> usize {
    let mut depth = 0usize;
    for i in (sub_pc + 1)..lines.len() {
        match &lines[i].statement {
            Statement::SubDef { .. } => depth += 1,
            Statement::EndSub => {
                if depth == 0 { return i; }
                depth -= 1;
            }
            _ => {}
        }
    }
    panic!("END SUB manquant");
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
    proc_stack: &mut Vec<ProcFrame>,
    sub_table: &HashMap<String, (usize, Vec<String>)>,
    output: &mut dyn Write,
) -> usize {
    match stmt {
        Statement::Rem => pc + 1,
        Statement::Label(_) => pc + 1,

        Statement::Dim { var, dims } => {
            if var.ends_with('$') {
                // Crée le tableau chaîne (usage avec indices)
                state.str_arrays.insert(var.clone(), ArrayData::new(dims));
                // Rétrocompatibilité 1D : str_dims pour la troncature scalaire
                if dims.len() == 1 {
                    state.str_dims.insert(var.clone(), dims[0]);
                    state.str_vars.entry(var.clone()).or_insert_with(String::new);
                }
            } else {
                state.int_arrays.insert(var.clone(), ArrayData::new(dims));
            }
            pc + 1
        }

        Statement::ArraySet { name, indices, value } => {
            let idx: Vec<i64> = indices.iter().map(|e| state.eval_int(e)).collect();
            if name.ends_with('$') {
                let s = state.eval_str(value);
                state.str_arrays.get_mut(name)
                    .unwrap_or_else(|| panic!("Tableau chaîne {} non déclaré", name))
                    .set(&idx, s);
            } else {
                let n = state.eval_int(value);
                state.int_arrays.get_mut(name)
                    .unwrap_or_else(|| panic!("Tableau entier {} non déclaré", name))
                    .set(&idx, n);
            }
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
                exec_stmt(then_stmt, pc, lines, state, for_stack, while_stack, call_stack, proc_stack, sub_table, output)
            } else if let Some(e) = else_stmt {
                exec_stmt(e, pc, lines, state, for_stack, while_stack, call_stack, proc_stack, sub_table, output)
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

        Statement::SubDef { .. } => {
            // Saute le corps du sous-programme (exécuté uniquement via CALL)
            find_end_sub(lines, pc) + 1
        }

        Statement::EndSub => {
            let frame = proc_stack.pop()
                .unwrap_or_else(|| panic!("END SUB sans CALL correspondant"));
            state.int_vars = frame.saved_int_vars;
            state.str_vars = frame.saved_str_vars;
            frame.return_pc
        }

        Statement::Call { name, args } => {
            let (body_start, params) = sub_table.get(name)
                .unwrap_or_else(|| panic!("Sous-programme '{}' non défini", name));
            let body_start = *body_start;
            let params = params.clone();

            // Évaluer les arguments dans la portée appelante
            let mut new_int_vars: HashMap<String, i64> = HashMap::new();
            let mut new_str_vars: HashMap<String, String> = HashMap::new();
            for (param, arg) in params.iter().zip(args.iter()) {
                if param.ends_with('$') {
                    new_str_vars.insert(param.clone(), state.eval_str(arg));
                } else {
                    new_int_vars.insert(param.clone(), state.eval_int(arg));
                }
            }

            // Sauvegarder la portée courante et entrer dans la nouvelle
            proc_stack.push(ProcFrame {
                return_pc: pc + 1,
                saved_int_vars: std::mem::replace(&mut state.int_vars, new_int_vars),
                saved_str_vars: std::mem::replace(&mut state.str_vars, new_str_vars),
            });
            body_start
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
    let mut proc_stack: Vec<ProcFrame> = Vec::new();
    let lines = &program.lines;

    // Construction de la table des sous-programmes
    let mut sub_table: HashMap<String, (usize, Vec<String>)> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        if let Statement::SubDef { name, params } = &line.statement {
            sub_table.insert(name.clone(), (i + 1, params.clone()));
        }
    }

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
            &mut proc_stack,
            &sub_table,
            output,
        );
    }
}
