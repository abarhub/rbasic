use std::cell::Cell;
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::ast::*;

use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode},
    execute,
    style::{ResetColor, SetBackgroundColor, SetForegroundColor, Color as CtColor},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};

// ---------------------------------------------------------------------------
// Value : valeur numérique unifiée (entier ou flottant)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
}

impl Default for Value {
    fn default() -> Self { Value::Int(0) }
}

impl Value {
    fn to_f64(&self) -> f64 {
        match self {
            Value::Int(n)   => *n as f64,
            Value::Float(f) => *f,
        }
    }
    fn to_i64(&self) -> i64 {
        match self {
            Value::Int(n)   => *n,
            Value::Float(f) => *f as i64,
        }
    }
    fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }
}

fn format_float(f: f64) -> String {
    if f.fract() == 0.0 && f.abs() < 1e15 {
        format!("{}", f as i64)
    } else {
        let s = format!("{:.7}", f);
        let s = s.trim_end_matches('0');
        s.trim_end_matches('.').to_string()
    }
}

// ---------------------------------------------------------------------------
// Tableaux génériques
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// État de l'interpréteur
// ---------------------------------------------------------------------------

struct State {
    num_vars:        HashMap<String, Value>,
    str_vars:        HashMap<String, String>,
    str_dims:        HashMap<String, usize>,
    num_arrays:      HashMap<String, ArrayData<Value>>,
    str_arrays:      HashMap<String, ArrayData<String>>,
    rng_seed:        Cell<u64>,
    /// true quand on tourne dans un vrai terminal (mode console activé)
    console_enabled: bool,
    /// Dernière touche lue par INKEY$ (vide si aucune)
    last_inkey:      String,
}

impl State {
    fn new() -> Self {
        State {
            num_vars:        HashMap::new(),
            str_vars:        HashMap::new(),
            str_dims:        HashMap::new(),
            num_arrays:      HashMap::new(),
            str_arrays:      HashMap::new(),
            rng_seed:        Cell::new(0),
            console_enabled: false,
            last_inkey:      String::new(),
        }
    }

    /// Générateur pseudo-aléatoire (LCG 64 bits) — retourne un flottant dans [0, 1).
    fn next_rnd(&self) -> f64 {
        let mut s = self.rng_seed.get();
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.rng_seed.set(s);
        // Extrait 53 bits pour un f64 dans [0, 1)
        (s >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Retourne le nombre de secondes depuis l'epoch Unix.
    fn timer() -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
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
            let v = self.eval_num(value);
            self.num_vars.insert(var.to_string(), v);
        }
    }

    // -----------------------------------------------------------------------
    // Fonctions built-in numériques → Option<Value>
    // -----------------------------------------------------------------------

    fn num_builtin(&self, name: &str, args: &[Expr]) -> Option<Value> {
        match name {
            "LEN" => {
                assert_eq!(args.len(), 1, "LEN attend 1 argument");
                Some(Value::Int(self.eval_str(&args[0]).chars().count() as i64))
            }
            "ASC" => {
                assert_eq!(args.len(), 1, "ASC attend 1 argument");
                let s = self.eval_str(&args[0]);
                Some(Value::Int(s.chars().next().map_or(0, |c| c as i64)))
            }
            "VAL" => {
                assert_eq!(args.len(), 1, "VAL attend 1 argument");
                let s = self.eval_str(&args[0]);
                let s = s.trim();
                if let Ok(n) = s.parse::<i64>() {
                    Some(Value::Int(n))
                } else if let Ok(f) = s.parse::<f64>() {
                    Some(Value::Float(f))
                } else {
                    Some(Value::Int(0))
                }
            }
            "INSTR" => {
                match args.len() {
                    2 => {
                        let s = self.eval_str(&args[0]);
                        let sub = self.eval_str(&args[1]);
                        Some(Value::Int(match s.find(sub.as_str()) {
                            Some(pos) => s[..pos].chars().count() as i64 + 1,
                            None => 0,
                        }))
                    }
                    3 => {
                        let start = (self.eval_num(&args[0]).to_i64() - 1).max(0) as usize;
                        let s = self.eval_str(&args[1]);
                        let sub = self.eval_str(&args[2]);
                        let chars: Vec<char> = s.chars().collect();
                        let slice: String = chars[start.min(chars.len())..].iter().collect();
                        Some(Value::Int(match slice.find(sub.as_str()) {
                            Some(pos) => start as i64 + slice[..pos].chars().count() as i64 + 1,
                            None => 0,
                        }))
                    }
                    _ => panic!("INSTR attend 2 ou 3 arguments"),
                }
            }
            "ABS" => {
                assert_eq!(args.len(), 1, "ABS attend 1 argument");
                Some(match self.eval_num(&args[0]) {
                    Value::Int(n)   => Value::Int(n.abs()),
                    Value::Float(f) => Value::Float(f.abs()),
                })
            }
            "SGN" => {
                assert_eq!(args.len(), 1, "SGN attend 1 argument");
                let n = self.eval_num(&args[0]).to_f64();
                Some(Value::Int(if n > 0.0 { 1 } else if n < 0.0 { -1 } else { 0 }))
            }
            "SQR" => {
                assert_eq!(args.len(), 1, "SQR attend 1 argument");
                let n = self.eval_num(&args[0]).to_f64();
                Some(Value::Float(n.sqrt()))
            }
            "INT" => {
                assert_eq!(args.len(), 1, "INT attend 1 argument");
                let f = self.eval_num(&args[0]).to_f64();
                Some(Value::Int(f.floor() as i64))
            }
            "FIX" => {
                assert_eq!(args.len(), 1, "FIX attend 1 argument");
                let f = self.eval_num(&args[0]).to_f64();
                Some(Value::Int(f.trunc() as i64))
            }
            "CINT" => {
                assert_eq!(args.len(), 1, "CINT attend 1 argument");
                let f = self.eval_num(&args[0]).to_f64();
                Some(Value::Int(f.round() as i64))
            }
            "CSNG" | "CDBL" => {
                assert_eq!(args.len(), 1, "{} attend 1 argument", name);
                let f = self.eval_num(&args[0]).to_f64();
                Some(Value::Float(f))
            }
            "RND" => {
                // RND() ou RND(n) : génère le prochain nombre aléatoire dans [0, 1)
                Some(Value::Float(self.next_rnd()))
            }
            // POS(n) : colonne courante du curseur (1-based, QBasic ignore n)
            "POS" => {
                Some(if self.console_enabled {
                    crossterm::cursor::position()
                        .map(|(col, _)| Value::Int(col as i64 + 1))
                        .unwrap_or(Value::Int(1))
                } else {
                    Value::Int(1)
                })
            }
            _ => None,
        }
    }

    // -----------------------------------------------------------------------
    // Fonctions built-in chaînes → Option<String>
    // -----------------------------------------------------------------------

    fn str_builtin(&self, name: &str, args: &[Expr]) -> Option<String> {
        match name {
            "STR$" => {
                assert_eq!(args.len(), 1, "STR$ attend 1 argument");
                Some(match self.eval_num(&args[0]) {
                    Value::Int(n)   => n.to_string(),
                    Value::Float(f) => format_float(f),
                })
            }
            "CHR$" => {
                assert_eq!(args.len(), 1, "CHR$ attend 1 argument");
                let n = self.eval_num(&args[0]).to_i64() as u32;
                Some(char::from_u32(n).map_or(String::new(), |c| c.to_string()))
            }
            "SPACE$" => {
                assert_eq!(args.len(), 1, "SPACE$ attend 1 argument");
                let n = self.eval_num(&args[0]).to_i64().max(0) as usize;
                Some(" ".repeat(n))
            }
            "LEFT$" => {
                assert_eq!(args.len(), 2, "LEFT$ attend 2 arguments");
                let s = self.eval_str(&args[0]);
                let n = self.eval_num(&args[1]).to_i64().max(0) as usize;
                Some(s.chars().take(n).collect())
            }
            "RIGHT$" => {
                assert_eq!(args.len(), 2, "RIGHT$ attend 2 arguments");
                let s = self.eval_str(&args[0]);
                let n = self.eval_num(&args[1]).to_i64().max(0) as usize;
                let chars: Vec<char> = s.chars().collect();
                let start = chars.len().saturating_sub(n);
                Some(chars[start..].iter().collect())
            }
            "MID$" => {
                assert!(args.len() == 2 || args.len() == 3, "MID$ attend 2 ou 3 arguments");
                let s = self.eval_str(&args[0]);
                let start = (self.eval_num(&args[1]).to_i64() - 1).max(0) as usize;
                let chars: Vec<char> = s.chars().collect();
                let from = start.min(chars.len());
                Some(if args.len() == 2 {
                    chars[from..].iter().collect()
                } else {
                    let len = self.eval_num(&args[2]).to_i64().max(0) as usize;
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
            "STRING$" => {
                assert_eq!(args.len(), 2, "STRING$ attend 2 arguments");
                let n = self.eval_num(&args[0]).to_i64().max(0) as usize;
                // 2e arg : chaîne (on prend le 1er caractère) ou entier (code ASCII)
                let ch = if Self::is_string_expr(&args[1]) {
                    self.eval_str(&args[1]).chars().next().unwrap_or('\0')
                } else {
                    let code = self.eval_num(&args[1]).to_i64() as u32;
                    char::from_u32(code).unwrap_or('\0')
                };
                Some(ch.to_string().repeat(n))
            }
            _ => None,
        }
    }

    // -----------------------------------------------------------------------
    // eval_num : évalue une expression numérique → Value
    // -----------------------------------------------------------------------

    fn eval_num(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Integer(n) => Value::Int(*n),
            Expr::Float(f)   => Value::Float(*f),
            Expr::Variable(name) if !name.ends_with('$') => {
                match name.as_str() {
                    "RND"    => Value::Float(self.next_rnd()),
                    "TIMER"  => Value::Float(Self::timer()),
                    // CSRLIN : ligne courante du curseur (1-based)
                    "CSRLIN" => {
                        if self.console_enabled {
                            crossterm::cursor::position()
                                .map(|(_, row)| Value::Int(row as i64 + 1))
                                .unwrap_or(Value::Int(1))
                        } else {
                            Value::Int(1)
                        }
                    }
                    _ => self.num_vars.get(name).cloned().unwrap_or(Value::Int(0)),
                }
            }
            Expr::ArrayAccess { name, indices } if !name.ends_with('$') => {
                if let Some(result) = self.num_builtin(name, indices) {
                    return result;
                }
                let idx: Vec<i64> = indices.iter().map(|e| self.eval_num(e).to_i64()).collect();
                self.num_arrays.get(name)
                    .unwrap_or_else(|| panic!("Tableau numérique ou fonction {} non déclaré(e)", name))
                    .get(&idx)
                    .clone()
            }
            Expr::UnaryOp { op, operand } => match op {
                UnaryOp::Neg => match self.eval_num(operand) {
                    Value::Int(n)   => Value::Int(-n),
                    Value::Float(f) => Value::Float(-f),
                },
                UnaryOp::Pos => self.eval_num(operand),
                UnaryOp::Not => Value::Int(!self.eval_num(operand).to_i64()),
            },
            Expr::BinOp { op, left, right } => {
                match op {
                    // Arithmétique : promotion flottante si l'un des opérandes est flottant.
                    // Exception : int / int = division entière (rétrocompatibilité).
                    Op::Add | Op::Sub | Op::Mul | Op::Mod => {
                        let l = self.eval_num(left);
                        let r = self.eval_num(right);
                        if l.is_float() || r.is_float() {
                            let lf = l.to_f64();
                            let rf = r.to_f64();
                            Value::Float(match op {
                                Op::Add => lf + rf,
                                Op::Sub => lf - rf,
                                Op::Mul => lf * rf,
                                Op::Mod => lf % rf,
                                _ => unreachable!(),
                            })
                        } else {
                            let li = l.to_i64();
                            let ri = r.to_i64();
                            Value::Int(match op {
                                Op::Add => li + ri,
                                Op::Sub => li - ri,
                                Op::Mul => li * ri,
                                Op::Mod => li % ri,
                                _ => unreachable!(),
                            })
                        }
                    }
                    Op::Div => {
                        let l = self.eval_num(left);
                        let r = self.eval_num(right);
                        if l.is_float() || r.is_float() {
                            Value::Float(l.to_f64() / r.to_f64())
                        } else {
                            // Division entière : rétrocompatibilité (PRINT 10/3 → 3)
                            Value::Int(l.to_i64() / r.to_i64())
                        }
                    }
                    // Comparaisons : résultat entier -1 (vrai) ou 0 (faux)
                    Op::Eq | Op::Ne | Op::Lt | Op::Gt | Op::Le | Op::Ge => {
                        let l = self.eval_num(left);
                        let r = self.eval_num(right);
                        let result = if l.is_float() || r.is_float() {
                            let lf = l.to_f64(); let rf = r.to_f64();
                            match op {
                                Op::Eq => lf == rf, Op::Ne => lf != rf,
                                Op::Lt => lf <  rf, Op::Gt => lf >  rf,
                                Op::Le => lf <= rf, Op::Ge => lf >= rf,
                                _ => unreachable!(),
                            }
                        } else {
                            let li = l.to_i64(); let ri = r.to_i64();
                            match op {
                                Op::Eq => li == ri, Op::Ne => li != ri,
                                Op::Lt => li <  ri, Op::Gt => li >  ri,
                                Op::Le => li <= ri, Op::Ge => li >= ri,
                                _ => unreachable!(),
                            }
                        };
                        Value::Int(if result { -1 } else { 0 })
                    }
                    // Opérateurs bit à bit : toujours entiers
                    Op::And => Value::Int(self.eval_num(left).to_i64() & self.eval_num(right).to_i64()),
                    Op::Or  => Value::Int(self.eval_num(left).to_i64() | self.eval_num(right).to_i64()),
                    Op::Xor => Value::Int(self.eval_num(left).to_i64() ^ self.eval_num(right).to_i64()),
                }
            }
            _ => panic!("Erreur de type : valeur numérique attendue"),
        }
    }

    // -----------------------------------------------------------------------
    // eval_str
    // -----------------------------------------------------------------------

    fn eval_str(&self, expr: &Expr) -> String {
        match expr {
            Expr::StringLit(s) => s.clone(),
            Expr::Variable(name) if name.ends_with('$') => {
                // INKEY$ : retourne la dernière touche lue (non-bloquant)
                if name == "INKEY$" {
                    return self.last_inkey.clone();
                }
                self.str_vars.get(name).cloned().unwrap_or_default()
            }
            Expr::ArrayAccess { name, indices } if name.ends_with('$') => {
                if let Some(result) = self.str_builtin(name, indices) {
                    return result;
                }
                let idx: Vec<i64> = indices.iter().map(|e| self.eval_num(e).to_i64()).collect();
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
            match self.eval_num(expr) {
                Value::Int(n)   => n.to_string(),
                Value::Float(f) => format_float(f),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Frames
// ---------------------------------------------------------------------------

struct ProcFrame {
    return_pc: usize,
    saved_num_vars: HashMap<String, Value>,
    saved_str_vars: HashMap<String, String>,
}

struct ForFrame {
    var: String,
    to: f64,
    step: f64,
    is_float: bool,
    body_start: usize,
}

// ---------------------------------------------------------------------------
// Fonctions utilitaires de navigation
// ---------------------------------------------------------------------------

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
                if depth == 0 { return i; }
                depth -= 1;
            }
            _ => {}
        }
    }
    panic!("WEND sans WHILE correspondant");
}

/// Cherche le prochain ELSEIF / ELSE / END IF au même niveau d'imbrication.
fn find_next_branch(lines: &[Line], from_pc: usize) -> usize {
    let mut depth = 0usize;
    for i in (from_pc + 1)..lines.len() {
        match &lines[i].statement {
            Statement::IfThen { .. } => depth += 1,
            Statement::ElseIf { .. } if depth == 0 => return i,
            Statement::Else          if depth == 0 => return i,
            Statement::EndIf         if depth == 0 => return i,
            Statement::EndIf => { if depth > 0 { depth -= 1; } }
            _ => {}
        }
    }
    panic!("END IF manquant (find_next_branch)");
}

/// Cherche le END IF correspondant à un IF THEN multiligne.
fn find_end_if(lines: &[Line], from_pc: usize) -> usize {
    let mut depth = 0usize;
    for i in (from_pc + 1)..lines.len() {
        match &lines[i].statement {
            Statement::IfThen { .. } => depth += 1,
            Statement::EndIf if depth == 0 => return i,
            Statement::EndIf => { if depth > 0 { depth -= 1; } }
            _ => {}
        }
    }
    panic!("END IF manquant (find_end_if)");
}

/// Cherche le LOOP correspondant à un DO.
fn find_matching_loop(lines: &[Line], do_pc: usize) -> usize {
    let mut depth = 0usize;
    for i in (do_pc + 1)..lines.len() {
        match &lines[i].statement {
            Statement::DoLoop { .. } => depth += 1,
            Statement::Loop { .. } if depth == 0 => return i,
            Statement::Loop { .. } => { if depth > 0 { depth -= 1; } }
            _ => {}
        }
    }
    panic!("LOOP sans DO correspondant");
}

// ---------------------------------------------------------------------------
// Console helpers
// ---------------------------------------------------------------------------

/// Convertit un code couleur QBasic (0-15) en couleur crossterm.
fn qbasic_color(n: u8) -> CtColor {
    match n & 0x0F {
        0  => CtColor::Black,
        1  => CtColor::DarkBlue,
        2  => CtColor::DarkGreen,
        3  => CtColor::DarkCyan,
        4  => CtColor::DarkRed,
        5  => CtColor::DarkMagenta,
        6  => CtColor::DarkYellow,   // Brown dans QBasic
        7  => CtColor::Grey,
        8  => CtColor::DarkGrey,
        9  => CtColor::Blue,
        10 => CtColor::Green,
        11 => CtColor::Cyan,
        12 => CtColor::Red,
        13 => CtColor::Magenta,
        14 => CtColor::Yellow,
        15 => CtColor::White,
        _  => CtColor::White,
    }
}

/// Lit une touche sans bloquer (nécessite le mode raw activé).
/// Retourne "" si aucune touche n'est disponible.
/// Les touches spéciales retournent chr(0) + code BIOS (convention QBasic).
fn poll_inkey() -> String {
    if poll(Duration::ZERO).unwrap_or(false) {
        match read() {
            Ok(Event::Key(key_event)) => match key_event.code {
                KeyCode::Char(c)  => c.to_string(),
                KeyCode::Enter    => "\r".to_string(),
                KeyCode::Esc      => "\x1b".to_string(),
                KeyCode::Backspace => "\x08".to_string(),
                KeyCode::Tab      => "\t".to_string(),
                // Touches de déplacement — codes BIOS PC
                KeyCode::Up       => "\x00H".to_string(),
                KeyCode::Down     => "\x00P".to_string(),
                KeyCode::Left     => "\x00K".to_string(),
                KeyCode::Right    => "\x00M".to_string(),
                KeyCode::Home     => "\x00G".to_string(),
                KeyCode::End      => "\x00O".to_string(),
                KeyCode::PageUp   => "\x00I".to_string(),
                KeyCode::PageDown => "\x00Q".to_string(),
                KeyCode::Insert   => "\x00R".to_string(),
                KeyCode::Delete   => "\x00S".to_string(),
                // Touches de fonction F1-F10
                KeyCode::F(n) if n >= 1 && n <= 10 => {
                    format!("\x00{}", char::from(58 + n))
                }
                _ => String::new(),
            },
            _ => String::new(),
        }
    } else {
        String::new()
    }
}

/// Adaptateur Write qui remplace les \n isolés par \r\n (mode raw terminal).
struct RawOutput<W: Write>(W);

impl<W: Write> Write for RawOutput<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut pos = 0;
        while pos < buf.len() {
            match buf[pos..].iter().position(|&b| b == b'\n') {
                None => {
                    self.0.write_all(&buf[pos..])?;
                    break;
                }
                Some(rel) => {
                    let abs = pos + rel;
                    // Écrire le contenu avant \n
                    if abs > pos {
                        self.0.write_all(&buf[pos..abs])?;
                    }
                    // \r\n sauf si déjà précédé d'un \r
                    if abs == 0 || buf[abs - 1] != b'\r' {
                        self.0.write_all(b"\r\n")?;
                    } else {
                        self.0.write_all(b"\n")?;
                    }
                    pos = abs + 1;
                }
            }
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> { self.0.flush() }
}

// ---------------------------------------------------------------------------
// exec_stmt
// ---------------------------------------------------------------------------

fn exec_stmt(
    stmt: &Statement,
    pc: usize,
    lines: &[Line],
    state: &mut State,
    for_stack:   &mut Vec<ForFrame>,
    while_stack: &mut Vec<usize>,
    call_stack:  &mut Vec<usize>,
    proc_stack:  &mut Vec<ProcFrame>,
    if_stack:    &mut Vec<bool>,   // true = une branche du IF courant a été exécutée
    do_stack:    &mut Vec<usize>,  // PC du DO correspondant
    sub_table: &HashMap<String, (usize, Vec<String>)>,
    output: &mut dyn Write,
) -> usize {
    match stmt {
        Statement::Rem => pc + 1,
        Statement::Label(_) => pc + 1,

        Statement::Dim { var, dims } => {
            if var.ends_with('$') {
                state.str_arrays.insert(var.clone(), ArrayData::new(dims));
                if dims.len() == 1 {
                    state.str_dims.insert(var.clone(), dims[0]);
                    state.str_vars.entry(var.clone()).or_insert_with(String::new);
                }
            } else {
                state.num_arrays.insert(var.clone(), ArrayData::new(dims));
            }
            pc + 1
        }

        Statement::ArraySet { name, indices, value } => {
            let idx: Vec<i64> = indices.iter().map(|e| state.eval_num(e).to_i64()).collect();
            if name.ends_with('$') {
                let s = state.eval_str(value);
                state.str_arrays.get_mut(name)
                    .unwrap_or_else(|| panic!("Tableau chaîne {} non déclaré", name))
                    .set(&idx, s);
            } else {
                let v = state.eval_num(value);
                state.num_arrays.get_mut(name)
                    .unwrap_or_else(|| panic!("Tableau numérique {} non déclaré", name))
                    .set(&idx, v);
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
            if state.eval_num(cond).to_i64() != 0 {
                exec_stmt(then_stmt, pc, lines, state, for_stack, while_stack, call_stack, proc_stack, if_stack, do_stack, sub_table, output)
            } else if let Some(e) = else_stmt {
                exec_stmt(e, pc, lines, state, for_stack, while_stack, call_stack, proc_stack, if_stack, do_stack, sub_table, output)
            } else {
                pc + 1
            }
        }

        Statement::For { var, from, to, step } => {
            let from_val = state.eval_num(from);
            let to_val   = state.eval_num(to);
            let step_val = step.as_ref().map_or(Value::Int(1), |s| state.eval_num(s));

            let is_float = from_val.is_float() || to_val.is_float() || step_val.is_float();
            let from_f = from_val.to_f64();
            let to_f   = to_val.to_f64();
            let step_f = step_val.to_f64();

            // Initialiser la variable de boucle
            let init_val = if is_float { Value::Float(from_f) } else { Value::Int(from_f as i64) };
            state.num_vars.insert(var.clone(), init_val);

            let next_pc = find_matching_next(lines, pc);
            let done = if step_f >= 0.0 { from_f > to_f } else { from_f < to_f };
            if done {
                return next_pc + 1;
            }

            for_stack.push(ForFrame {
                var: var.clone(),
                to: to_f,
                step: step_f,
                is_float,
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
            let cur = state.num_vars.get(&frame.var).cloned().unwrap_or(Value::Int(0));
            let new_f = cur.to_f64() + frame.step;
            let done = if frame.step >= 0.0 { new_f > frame.to } else { new_f < frame.to };
            if done {
                for_stack.pop();
                pc + 1
            } else {
                let new_val = if frame.is_float {
                    Value::Float(new_f)
                } else {
                    Value::Int(new_f as i64)
                };
                state.num_vars.insert(frame.var.clone(), new_val);
                frame.body_start
            }
        }

        Statement::While { cond } => {
            if state.eval_num(cond).to_i64() != 0 {
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
                    if state.eval_num(cond).to_i64() != 0 {
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
            find_end_sub(lines, pc) + 1
        }

        Statement::EndSub => {
            let frame = proc_stack.pop()
                .unwrap_or_else(|| panic!("END SUB sans CALL correspondant"));
            state.num_vars = frame.saved_num_vars;
            state.str_vars = frame.saved_str_vars;
            frame.return_pc
        }

        Statement::Call { name, args } => {
            let (body_start, params) = sub_table.get(name)
                .unwrap_or_else(|| panic!("Sous-programme '{}' non défini", name));
            let body_start = *body_start;
            let params = params.clone();

            let mut new_num_vars: HashMap<String, Value> = HashMap::new();
            let mut new_str_vars: HashMap<String, String> = HashMap::new();
            for (param, arg) in params.iter().zip(args.iter()) {
                if param.ends_with('$') {
                    new_str_vars.insert(param.clone(), state.eval_str(arg));
                } else {
                    new_num_vars.insert(param.clone(), state.eval_num(arg));
                }
            }

            proc_stack.push(ProcFrame {
                return_pc: pc + 1,
                saved_num_vars: std::mem::replace(&mut state.num_vars, new_num_vars),
                saved_str_vars: std::mem::replace(&mut state.str_vars, new_str_vars),
            });
            body_start
        }

        Statement::Sleep { duration } => {
            let secs = state.eval_num(duration).to_f64().max(0.0);
            let millis = (secs * 1000.0) as u64;
            std::thread::sleep(std::time::Duration::from_millis(millis));
            pc + 1
        }

        Statement::Randomize { seed } => {
            let v = state.eval_num(seed).to_f64();
            state.rng_seed.set(v.to_bits());
            pc + 1
        }

        // -----------------------------------------------------------------------
        // IF multiligne
        // -----------------------------------------------------------------------

        Statement::IfThen { cond } => {
            if state.eval_num(cond).to_i64() != 0 {
                if_stack.push(true);   // une branche a été prise
                pc + 1
            } else {
                if_stack.push(false);  // aucune branche prise pour l'instant
                find_next_branch(lines, pc)
            }
        }

        Statement::ElseIf { cond } => {
            let taken = *if_stack.last()
                .unwrap_or_else(|| panic!("ELSEIF sans IF correspondant"));
            if taken {
                // Une branche précédente a déjà été exécutée → sauter jusqu'à END IF
                find_end_if(lines, pc)
            } else if state.eval_num(cond).to_i64() != 0 {
                *if_stack.last_mut().unwrap() = true;
                pc + 1
            } else {
                find_next_branch(lines, pc)
            }
        }

        Statement::Else => {
            let taken = *if_stack.last()
                .unwrap_or_else(|| panic!("ELSE sans IF correspondant"));
            if taken {
                find_end_if(lines, pc)
            } else {
                *if_stack.last_mut().unwrap() = true;
                pc + 1
            }
        }

        Statement::EndIf => {
            if_stack.pop();
            pc + 1
        }

        // -----------------------------------------------------------------------
        // DO / LOOP
        // -----------------------------------------------------------------------

        Statement::DoLoop { pre_cond } => {
            let enter = match pre_cond {
                None => true,
                Some(DoCondition::While(cond)) => state.eval_num(cond).to_i64() != 0,
                Some(DoCondition::Until(cond)) => state.eval_num(cond).to_i64() == 0,
            };
            if enter {
                do_stack.push(pc);
                pc + 1
            } else {
                find_matching_loop(lines, pc) + 1
            }
        }

        Statement::Loop { post_cond } => {
            let do_pc = do_stack.pop()
                .unwrap_or_else(|| panic!("LOOP sans DO correspondant"));
            let loop_again = match post_cond {
                // LOOP seul : re-évaluer la condition du DO
                None => match &lines[do_pc].statement {
                    Statement::DoLoop { pre_cond: None } => true, // DO sans cond = boucle infinie
                    Statement::DoLoop { pre_cond: Some(DoCondition::While(cond)) } => {
                        state.eval_num(cond).to_i64() != 0
                    }
                    Statement::DoLoop { pre_cond: Some(DoCondition::Until(cond)) } => {
                        state.eval_num(cond).to_i64() == 0
                    }
                    _ => panic!("LOOP : le PC do_stack ne pointe pas sur DO"),
                },
                // LOOP WHILE cond
                Some(DoCondition::While(cond)) => state.eval_num(cond).to_i64() != 0,
                // LOOP UNTIL cond
                Some(DoCondition::Until(cond)) => state.eval_num(cond).to_i64() == 0,
            };
            if loop_again {
                do_stack.push(do_pc);
                do_pc + 1  // re-entre dans le corps (saute le DO)
            } else {
                pc + 1
            }
        }

        // -----------------------------------------------------------------------
        // DECLARE SUB — déclaration anticipée, no-op à l'exécution
        // -----------------------------------------------------------------------

        Statement::DeclareSub { .. } => pc + 1,

        // -----------------------------------------------------------------------
        // Console
        // -----------------------------------------------------------------------

        // SCREEN mode — no-op (mode texte uniquement supporté)
        Statement::Screen { .. } => pc + 1,

        // WIDTH cols — no-op
        Statement::Width { .. } => pc + 1,

        // CLS — efface l'écran et ramène le curseur en haut à gauche
        Statement::Cls => {
            if state.console_enabled {
                let _ = execute!(
                    io::stdout(),
                    Clear(ClearType::All),
                    MoveTo(0, 0),
                );
            }
            pc + 1
        }

        // BEEP — émet le caractère BEL (fonctionne même hors console)
        Statement::Beep => {
            write!(output, "\x07").unwrap();
            output.flush().unwrap();
            pc + 1
        }

        // COLOR fg [, bg] — définit les couleurs du texte suivant
        Statement::Color { fg, bg } => {
            if state.console_enabled {
                let fg_n = state.eval_num(fg).to_i64().clamp(0, 15) as u8;
                let _ = execute!(io::stdout(), SetForegroundColor(qbasic_color(fg_n)));
                if let Some(bg_expr) = bg {
                    let bg_n = state.eval_num(bg_expr).to_i64().clamp(0, 7) as u8;
                    let _ = execute!(io::stdout(), SetBackgroundColor(qbasic_color(bg_n)));
                }
            }
            pc + 1
        }

        // LOCATE row, col — positionne le curseur (QBasic : 1-based)
        Statement::Locate { row, col } => {
            if state.console_enabled {
                let r = (state.eval_num(row).to_i64() as u16).saturating_sub(1);
                let c = (state.eval_num(col).to_i64() as u16).saturating_sub(1);
                let _ = execute!(io::stdout(), MoveTo(c, r));
            }
            pc + 1
        }
    }
}

// ---------------------------------------------------------------------------
// Point d'entrée public
// ---------------------------------------------------------------------------

/// Logique d'exécution commune à run() et run_with_output().
fn run_internal(program: &Program, output: &mut dyn Write, state: &mut State) {
    let mut for_stack:   Vec<ForFrame>  = Vec::new();
    let mut while_stack: Vec<usize>     = Vec::new();
    let mut call_stack:  Vec<usize>     = Vec::new();
    let mut proc_stack:  Vec<ProcFrame> = Vec::new();
    let mut if_stack:    Vec<bool>      = Vec::new();
    let mut do_stack:    Vec<usize>     = Vec::new();
    let lines = &program.lines;

    // Pré-scan des SUB pour la table de sous-programmes
    let mut sub_table: HashMap<String, (usize, Vec<String>)> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        if let Statement::SubDef { name, params } = &line.statement {
            sub_table.insert(name.clone(), (i + 1, params.clone()));
        }
    }

    let mut pc = 0usize;
    while pc < lines.len() {
        // Mise à jour INKEY$ avant chaque instruction
        if state.console_enabled {
            state.last_inkey = poll_inkey();
        }
        pc = exec_stmt(
            &lines[pc].statement,
            pc,
            lines,
            state,
            &mut for_stack,
            &mut while_stack,
            &mut call_stack,
            &mut proc_stack,
            &mut if_stack,
            &mut do_stack,
            &sub_table,
            output,
        );
    }
}

/// Exécute le programme sur le terminal réel (mode raw, couleurs, curseur).
pub fn run(program: &Program) {
    let mut state = State::new();
    state.console_enabled = true;

    // Active le mode raw pour INKEY$ et les séquences de contrôle
    let raw_ok = enable_raw_mode().is_ok();
    {
        let mut out = RawOutput(io::stdout());
        run_internal(program, &mut out, &mut state);
        let _ = out.flush();
    }
    if raw_ok {
        let _ = disable_raw_mode();
        // Réinitialise les couleurs après l'exécution
        let _ = execute!(io::stdout(), ResetColor);
    }
}

/// Exécute le programme en redirigeant la sortie vers `output` (tests, pipes).
/// Les commandes console (COLOR, LOCATE, CLS …) sont des no-ops.
pub fn run_with_output(program: &Program, output: &mut dyn Write) {
    let mut state = State::new();
    // console_enabled reste false : toutes les opérations terminal sont ignorées
    run_internal(program, output, &mut state);
}
