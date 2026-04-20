#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Pos,
    Not,
}

#[derive(Debug, Clone)]
pub enum Op {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Gt, Le, Ge,
    And, Or, Xor,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    StringLit(String),
    Variable(String),
    ArrayAccess { name: String, indices: Vec<Expr> },
    BinOp  { op: Op,       left: Box<Expr>, right: Box<Expr> },
    UnaryOp { op: UnaryOp, operand: Box<Expr> },
}

#[derive(Debug, Clone)]
pub enum JumpTarget {
    LineNumber(u64),
    Label(String),
}

/// Condition pour DO/LOOP
#[derive(Debug, Clone)]
pub enum DoCondition {
    While(Expr),
    Until(Expr),
}

#[derive(Debug, Clone)]
pub enum Statement {
    // --- Affectation / déclaration ---
    Let   { var: String, value: Expr },
    Dim   { var: String, dims: Vec<usize> },
    /// DIM avec plusieurs tableaux sur une ligne — expansé en Dim individuels dans line()
    DimMulti { items: Vec<(String, Vec<usize>)> },
    ArraySet { name: String, indices: Vec<Expr>, value: Expr },
    // --- Affichage ---
    /// separators[i] = true → ';' entre values[i] et values[i+1] (pas d'espace)
    ///                false → ',' entre values[i] et values[i+1] (espace)
    /// no_newline = true si ';' ou ',' en fin de PRINT (pas de saut de ligne)
    Print { values: Vec<Expr>, separators: Vec<bool>, no_newline: bool },
    // --- Commentaire ---
    Rem,
    // --- Sauts ---
    Label(String),
    Goto(JumpTarget),
    Gosub(JumpTarget),
    Return,
    // --- IF sur une ligne ---
    If { cond: Expr, then_stmt: Box<Statement>, else_stmt: Option<Box<Statement>> },
    // --- IF multiligne ---
    IfThen { cond: Expr },
    ElseIf { cond: Expr },
    Else,
    EndIf,
    // --- FOR/NEXT ---
    For  { var: String, from: Expr, to: Expr, step: Option<Expr> },
    Next { var: Option<String> },
    /// NEXT K, J, I — expansé en Next individuels dans line()
    NextMulti { vars: Vec<String> },
    // --- WHILE/WEND ---
    While { cond: Expr },
    Wend,
    // --- DO/LOOP ---
    DoLoop { pre_cond: Option<DoCondition> },
    Loop   { post_cond: Option<DoCondition> },
    // --- Sous-programmes ---
    SubDef     { name: String, params: Vec<String> },
    EndSub,
    Call       { name: String, args: Vec<Expr> },
    DeclareSub { name: String, params: Vec<String> },
    // --- Divers ---
    Sleep     { duration: Expr },
    Randomize { seed: Expr },
    // --- Console ---
    /// SCREEN mode  — no-op (mode texte uniquement)
    Screen { mode: Expr },
    /// WIDTH cols   — no-op
    Width  { cols: Expr },
    /// COLOR fg [, bg]
    Color  { fg: Expr, bg: Option<Expr> },
    /// LOCATE row, col
    Locate { row: Expr, col: Expr },
    /// CLS
    Cls,
    /// BEEP
    Beep,
    /// KEY ON/OFF/... — no-op
    Key,
    // --- Fin de programme ---
    End,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub number: Option<u64>,
    pub statement: Statement,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub lines: Vec<Line>,
}
