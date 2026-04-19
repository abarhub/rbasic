use chumsky::prelude::*;
use crate::ast::*;

fn integer() -> impl Parser<char, i64, Error = Simple<char>> {
    text::int(10)
        .map(|s: String| s.parse::<i64>().unwrap())
}

fn string_lit() -> impl Parser<char, String, Error = Simple<char>> {
    just('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .map(|chars| chars.into_iter().collect())
}

fn variable_name() -> impl Parser<char, String, Error = Simple<char>> {
    text::ident()
}

fn expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    let int = integer().map(Expr::Integer);
    let s = string_lit().map(Expr::StringLit);
    let var = variable_name().map(Expr::Variable);
    s.or(int).or(var)
}

fn let_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("LET")
        .padded()
        .ignore_then(variable_name())
        .then_ignore(just('=').padded())
        .then(integer().map(Expr::Integer))
        .map(|(var, value)| Statement::Let { var, value })
}

fn print_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("PRINT")
        .padded()
        .ignore_then(expr())
        .map(|value| Statement::Print { value })
}

fn statement() -> impl Parser<char, Statement, Error = Simple<char>> {
    let_stmt().or(print_stmt())
}

fn line() -> impl Parser<char, Line, Error = Simple<char>> {
    text::int(10)
        .map(|s: String| s.parse::<u64>().unwrap())
        .then_ignore(text::whitespace())
        .then(statement())
        .map(|(number, statement)| Line { number, statement })
}

fn program() -> impl Parser<char, Program, Error = Simple<char>> {
    line()
        .separated_by(text::newline())
        .allow_trailing()
        .map(|lines| Program { lines })
}

pub fn parse(source: &str) -> Result<Program, Vec<Simple<char>>> {
    program().parse(source)
}
