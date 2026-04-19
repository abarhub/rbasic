#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    StringLit(String),
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let { var: String, value: Expr },
    Print { value: Expr },
}

#[derive(Debug, Clone)]
pub struct Line {
    pub number: u64,
    pub statement: Statement,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub lines: Vec<Line>,
}
