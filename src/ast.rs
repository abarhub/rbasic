#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    StringLit(String),
    Variable(String),
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
