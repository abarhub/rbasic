use chumsky::prelude::*;
use crate::ast::*;

// Espaces horizontaux seulement (pas les sauts de ligne)
fn hspace() -> impl Parser<char, (), Error = Simple<char>> {
    filter(|c: &char| *c == ' ' || *c == '\t').repeated().ignored()
}

fn integer() -> impl Parser<char, i64, Error = Simple<char>> {
    text::int(10).map(|s: String| s.parse::<i64>().unwrap())
}

fn string_lit() -> impl Parser<char, String, Error = Simple<char>> {
    just('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .map(|chars| chars.into_iter().collect())
}

fn expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    string_lit().map(Expr::StringLit)
        .or(integer().map(Expr::Integer))
        .or(text::ident().map(Expr::Variable))
}

fn assign_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    let with_let = text::keyword("LET")
        .ignore_then(hspace())
        .ignore_then(text::ident())
        .then_ignore(hspace())
        .then_ignore(just('='))
        .then_ignore(hspace())
        .then(integer().map(Expr::Integer))
        .map(|(var, value)| Statement::Let { var, value });

    let without_let = text::ident()
        .then_ignore(hspace())
        .then_ignore(just('='))
        .then_ignore(hspace())
        .then(integer().map(Expr::Integer))
        .map(|(var, value)| Statement::Let { var, value });

    with_let.or(without_let)
}

fn print_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    let values = expr()
        .then_ignore(hspace())
        .separated_by(just(',').then_ignore(hspace()));

    text::keyword("PRINT")
        .ignore_then(hspace())
        .ignore_then(values)
        .map(|values| Statement::Print { values })
}

fn statement() -> impl Parser<char, Statement, Error = Simple<char>> {
    print_stmt().or(assign_stmt())
}

fn line() -> impl Parser<char, Line, Error = Simple<char>> {
    let line_number = text::int(10)
        .map(|s: String| s.parse::<u64>().unwrap())
        .then_ignore(hspace())
        .or_not();

    line_number
        .then(statement())
        .map(|(number, statement)| Line { number, statement })
}

pub fn parse(source: &str) -> Result<Program, Vec<Simple<char>>> {
    line()
        .separated_by(
            filter(|c: &char| *c == '\r' || *c == '\n').repeated().at_least(1)
        )
        .allow_leading()
        .allow_trailing()
        .map(|lines| Program { lines })
        .parse(source)
}
