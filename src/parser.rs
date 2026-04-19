use chumsky::prelude::*;
use crate::ast::*;

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

fn var_name() -> impl Parser<char, String, Error = Simple<char>> {
    text::ident()
        .then(just('$').or_not())
        .map(|(name, dollar): (String, Option<char>)| {
            if dollar.is_some() { format!("{}$", name) } else { name }
        })
}

// Grammaire des expressions avec précédence :
//   comparaison = addition (op_cmp addition)?
//   addition    = multiplication ((+ | -) multiplication)*
//   multiplié   = atom ((* | / | %) atom)*
//   atom        = littéral | variable | '(' comparaison ')'
fn expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(|expr_rec| {
        let atom = string_lit().map(Expr::StringLit)
            .or(integer().map(Expr::Integer))
            .or(var_name().map(Expr::Variable))
            .or(just('(')
                .ignore_then(hspace())
                .ignore_then(expr_rec)
                .then_ignore(hspace())
                .then_ignore(just(')')))
            .boxed();

        let mul_op = just('*').to(Op::Mul)
            .or(just('/').to(Op::Div))
            .or(just('%').to(Op::Mod));

        let mul = atom.clone()
            .then(
                hspace()
                    .ignore_then(mul_op)
                    .then_ignore(hspace())
                    .then(atom)
                    .repeated()
            )
            .foldl(|l, (op, r)| Expr::BinOp { op, left: Box::new(l), right: Box::new(r) })
            .boxed();

        let add_op = just('+').to(Op::Add).or(just('-').to(Op::Sub));

        let add = mul.clone()
            .then(
                hspace()
                    .ignore_then(add_op)
                    .then_ignore(hspace())
                    .then(mul)
                    .repeated()
            )
            .foldl(|l, (op, r)| Expr::BinOp { op, left: Box::new(l), right: Box::new(r) })
            .boxed();

        // Les opérateurs multi-chars sont essayés avant les opérateurs simples
        let cmp_op = just('<').then(just('>')).to(Op::Ne)
            .or(just('<').then(just('=')).to(Op::Le))
            .or(just('>').then(just('=')).to(Op::Ge))
            .or(just('<').to(Op::Lt))
            .or(just('>').to(Op::Gt))
            .or(just('=').to(Op::Eq));

        add.clone()
            .then(
                hspace()
                    .ignore_then(cmp_op)
                    .then_ignore(hspace())
                    .then(add)
                    .or_not()
            )
            .map(|(l, rest)| match rest {
                Some((op, r)) => Expr::BinOp { op, left: Box::new(l), right: Box::new(r) },
                None => l,
            })
    })
}

fn assign_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    let with_let = text::keyword("LET")
        .ignore_then(hspace())
        .ignore_then(var_name())
        .then_ignore(hspace())
        .then_ignore(just('='))
        .then_ignore(hspace())
        .then(expr())
        .map(|(var, value)| Statement::Let { var, value });

    let without_let = var_name()
        .then_ignore(hspace())
        .then_ignore(just('='))
        .then_ignore(hspace())
        .then(expr())
        .map(|(var, value)| Statement::Let { var, value });

    with_let.or(without_let)
}

fn dim_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("DIM")
        .ignore_then(hspace())
        .ignore_then(var_name())
        .then_ignore(hspace())
        .then_ignore(just('('))
        .then_ignore(hspace())
        .then(integer().map(|n| n as usize))
        .then_ignore(hspace())
        .then_ignore(just(')'))
        .map(|(var, size)| Statement::Dim { var, size })
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
    dim_stmt().or(print_stmt()).or(assign_stmt())
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
