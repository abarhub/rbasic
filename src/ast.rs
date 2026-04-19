#[derive(Debug, Clone)]
pub enum Op {
    // Arithmétique
    Add, Sub, Mul, Div, Mod,
    // Comparaison (résultat : -1 si vrai, 0 si faux)
    Eq, Ne, Lt, Gt, Le, Ge,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    StringLit(String),
    Variable(String),
    BinOp { op: Op, left: Box<Expr>, right: Box<Expr> },
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let { var: String, value: Expr },
    Dim { var: String, size: usize },
    Print { values: Vec<Expr> },
}

#[derive(Debug, Clone)]
pub struct Line {
    #[allow(dead_code)]
    pub number: Option<u64>,
    pub statement: Statement,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub lines: Vec<Line>,
}
