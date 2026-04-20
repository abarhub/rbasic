#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg, // - (négation)
    Pos, // + (no-op)
    Not, // NOT (complément bit à bit, convention QBasic : NOT x = -(x+1))
}

#[derive(Debug, Clone)]
pub enum Op {
    // Arithmétique
    Add, Sub, Mul, Div, Mod,
    // Comparaison (résultat : -1 vrai, 0 faux)
    Eq, Ne, Lt, Gt, Le, Ge,
    // Logique / bit à bit
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
    ArraySet { name: String, indices: Vec<Expr>, value: Expr },
    // --- Affichage ---
    Print { values: Vec<Expr> },
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
