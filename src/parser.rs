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

// Hiérarchie de précédence (du plus fort au plus faible) :
//   atom       : littéral, variable, ( expr )
//   unaire     : -atom  +atom  (récursif : --3 = -(-3))
//   mul        : * / %
//   add        : + -
//   cmp        : = <> < > <= >=   (au plus une comparaison)
//   NOT        : NOT cmp
//   AND        : AND (bit à bit)
//   OR         : OR  (bit à bit)
//   XOR        : XOR (bit à bit)
fn expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(|expr_rec| {
        // --- atom ---
        let atom = string_lit().map(Expr::StringLit)
            .or(integer().map(Expr::Integer))
            .or(var_name().map(Expr::Variable))
            .or(just('(')
                .ignore_then(hspace())
                .ignore_then(expr_rec)
                .then_ignore(hspace())
                .then_ignore(just(')')))
            .boxed();

        // --- unaire (récursif : permet --3, -+5, etc.) ---
        let unary = recursive(|unary_rec| {
            just('-').ignore_then(hspace()).ignore_then(unary_rec.clone())
                .map(|e| Expr::UnaryOp { op: UnaryOp::Neg, operand: Box::new(e) })
            .or(just('+').ignore_then(hspace()).ignore_then(unary_rec)
                .map(|e| Expr::UnaryOp { op: UnaryOp::Pos, operand: Box::new(e) }))
            .or(atom)
        }).boxed();

        // --- mul : * / % ---
        let mul_op = just('*').to(Op::Mul)
            .or(just('/').to(Op::Div))
            .or(just('%').to(Op::Mod));

        let mul = unary.clone()
            .then(hspace().ignore_then(mul_op).then_ignore(hspace()).then(unary).repeated())
            .foldl(|l, (op, r)| Expr::BinOp { op, left: Box::new(l), right: Box::new(r) })
            .boxed();

        // --- add : + - ---
        let add_op = just('+').to(Op::Add).or(just('-').to(Op::Sub));

        let add = mul.clone()
            .then(hspace().ignore_then(add_op).then_ignore(hspace()).then(mul).repeated())
            .foldl(|l, (op, r)| Expr::BinOp { op, left: Box::new(l), right: Box::new(r) })
            .boxed();

        // --- cmp : = <> < > <= >= (au plus une comparaison) ---
        let cmp_op = just('<').then(just('>')).to(Op::Ne)
            .or(just('<').then(just('=')).to(Op::Le))
            .or(just('>').then(just('=')).to(Op::Ge))
            .or(just('<').to(Op::Lt))
            .or(just('>').to(Op::Gt))
            .or(just('=').to(Op::Eq));

        let cmp = add.clone()
            .then(hspace().ignore_then(cmp_op).then_ignore(hspace()).then(add).or_not())
            .map(|(l, rest)| match rest {
                Some((op, r)) => Expr::BinOp { op, left: Box::new(l), right: Box::new(r) },
                None => l,
            })
            .boxed();

        // --- NOT (niveau entre cmp et AND) ---
        let not_level = text::keyword("NOT")
            .ignore_then(hspace())
            .ignore_then(cmp.clone())
            .map(|e| Expr::UnaryOp { op: UnaryOp::Not, operand: Box::new(e) })
            .or(cmp)
            .boxed();

        // --- AND ---
        let and_level = not_level.clone()
            .then(
                hspace().ignore_then(text::keyword("AND").to(Op::And))
                    .then_ignore(hspace()).then(not_level).repeated()
            )
            .foldl(|l, (op, r)| Expr::BinOp { op, left: Box::new(l), right: Box::new(r) })
            .boxed();

        // --- OR ---
        let or_level = and_level.clone()
            .then(
                hspace().ignore_then(text::keyword("OR").to(Op::Or))
                    .then_ignore(hspace()).then(and_level).repeated()
            )
            .foldl(|l, (op, r)| Expr::BinOp { op, left: Box::new(l), right: Box::new(r) })
            .boxed();

        // --- XOR (niveau le plus bas) ---
        or_level.clone()
            .then(
                hspace().ignore_then(text::keyword("XOR").to(Op::Xor))
                    .then_ignore(hspace()).then(or_level).repeated()
            )
            .foldl(|l, (op, r)| Expr::BinOp { op, left: Box::new(l), right: Box::new(r) })
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

fn rem_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("REM")
        .ignore_then(filter(|c: &char| *c != '\n' && *c != '\r').repeated())
        .to(Statement::Rem)
}

fn statement() -> impl Parser<char, Statement, Error = Simple<char>> {
    rem_stmt().or(dim_stmt()).or(print_stmt()).or(assign_stmt())
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
