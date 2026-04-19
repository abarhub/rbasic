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
    StringLit(String),
    Variable(String),
    BinOp  { op: Op,       left: Box<Expr>, right: Box<Expr> },
    UnaryOp { op: UnaryOp, operand: Box<Expr> },
}

#[derive(Debug, Clone)]
pub enum JumpTarget {
    LineNumber(u64),
    Label(String),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let   { var: String, value: Expr },
    Dim   { var: String, size: usize },
    Print { values: Vec<Expr> },
    Rem,
    Label(String),
    Goto(JumpTarget),
    If { cond: Expr, then_stmt: Box<Statement>, else_stmt: Option<Box<Statement>> },
    For  { var: String, from: Expr, to: Expr, step: Option<Expr> },
    Next { var: Option<String> },
    While { cond: Expr },
    Wend,
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
