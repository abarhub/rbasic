use chumsky::prelude::*;
use crate::ast::*;

fn hspace() -> impl Parser<char, (), Error = Simple<char>> {
    filter(|c: &char| *c == ' ' || *c == '\t').repeated().ignored()
}

fn integer() -> impl Parser<char, i64, Error = Simple<char>> {
    text::int(10).map(|s: String| s.parse::<i64>().unwrap())
}

// Parses an integer or a float literal.
// "3.14" → Expr::Float(3.14), "42" → Expr::Integer(42)
fn number() -> impl Parser<char, Expr, Error = Simple<char>> {
    text::int(10)
        .then(
            just('.')
                .ignore_then(
                    filter(|c: &char| c.is_ascii_digit())
                        .repeated()
                        .at_least(1)
                        .collect::<String>()
                )
                .or_not()
        )
        .map(|(int_part, frac_opt): (String, Option<String>)| {
            if let Some(frac) = frac_opt {
                Expr::Float(format!("{}.{}", int_part, frac).parse::<f64>().unwrap())
            } else {
                Expr::Integer(int_part.parse::<i64>().unwrap())
            }
        })
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
//   mul        : * / MOD
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
            .or(number())
            .or(var_name()
                .then(
                    hspace()
                        .ignore_then(just('('))
                        .ignore_then(hspace())
                        .ignore_then(
                            expr_rec.clone()
                                .then_ignore(hspace())
                                .separated_by(just(',').then_ignore(hspace()))
                                .at_least(1)
                        )
                        .then_ignore(hspace())
                        .then_ignore(just(')'))
                        .or_not()
                )
                .map(|(name, opt_indices)| match opt_indices {
                    Some(indices) => Expr::ArrayAccess { name, indices },
                    None => Expr::Variable(name),
                })
            )
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

        // --- mul : * / MOD ---
        let mul_op = just('*').to(Op::Mul)
            .or(just('/').to(Op::Div))
            .or(text::keyword("MOD").to(Op::Mod));

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
        .then(
            integer()
                .map(|n| n as usize)
                .then_ignore(hspace())
                .separated_by(just(',').then_ignore(hspace()))
                .at_least(1)
        )
        .then_ignore(hspace())
        .then_ignore(just(')'))
        .map(|(var, dims)| Statement::Dim { var, dims })
}

fn array_set_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    var_name()
        .then_ignore(hspace())
        .then_ignore(just('('))
        .then_ignore(hspace())
        .then(
            expr()
                .then_ignore(hspace())
                .separated_by(just(',').then_ignore(hspace()))
                .at_least(1)
        )
        .then_ignore(hspace())
        .then_ignore(just(')'))
        .then_ignore(hspace())
        .then_ignore(just('='))
        .then_ignore(hspace())
        .then(expr())
        .map(|((name, indices), value)| Statement::ArraySet { name, indices, value })
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

fn jump_target() -> impl Parser<char, JumpTarget, Error = Simple<char>> {
    text::int(10)
        .map(|s: String| JumpTarget::LineNumber(s.parse::<u64>().unwrap()))
        .or(text::ident().map(JumpTarget::Label))
}

fn goto_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("GOTO")
        .ignore_then(hspace())
        .ignore_then(jump_target())
        .map(Statement::Goto)
}

fn gosub_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("GOSUB")
        .ignore_then(hspace())
        .ignore_then(jump_target())
        .map(Statement::Gosub)
}

fn return_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("RETURN").to(Statement::Return)
}

// Paramètres partagés entre SUB, DECLARE SUB
fn param_list() -> impl Parser<char, Vec<String>, Error = Simple<char>> {
    hspace()
        .ignore_then(just('('))
        .ignore_then(hspace())
        .ignore_then(
            var_name()
                .then_ignore(hspace())
                .separated_by(just(',').then_ignore(hspace()))
        )
        .then_ignore(hspace())
        .then_ignore(just(')'))
        .or_not()
        .map(|opt| opt.unwrap_or_default())
}

fn sub_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("SUB")
        .ignore_then(hspace())
        .ignore_then(text::ident())
        .then(param_list())
        .map(|(name, params)| Statement::SubDef { name, params })
}

fn end_sub_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("END")
        .ignore_then(hspace())
        .ignore_then(text::keyword("SUB"))
        .to(Statement::EndSub)
}

fn declare_sub_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("DECLARE")
        .ignore_then(hspace())
        .ignore_then(text::keyword("SUB"))
        .ignore_then(hspace())
        .ignore_then(text::ident())
        .then(param_list())
        .map(|(name, params)| Statement::DeclareSub { name, params })
}

fn call_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("CALL")
        .ignore_then(hspace())
        .ignore_then(text::ident())
        .then(
            hspace()
                .ignore_then(just('('))
                .ignore_then(hspace())
                .ignore_then(
                    expr()
                        .then_ignore(hspace())
                        .separated_by(just(',').then_ignore(hspace()))
                )
                .then_ignore(hspace())
                .then_ignore(just(')'))
                .or_not()
                .map(|opt| opt.unwrap_or_default())
        )
        .map(|(name, args)| Statement::Call { name, args })
}

fn label_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::ident()
        .then_ignore(hspace())
        .then_ignore(just(':'))
        .map(Statement::Label)
}

fn sleep_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("SLEEP")
        .ignore_then(hspace())
        .ignore_then(expr())
        .map(|duration| Statement::Sleep { duration })
}

fn randomize_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("RANDOMIZE")
        .ignore_then(hspace())
        .ignore_then(expr())
        .map(|seed| Statement::Randomize { seed })
}

fn for_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    let step = hspace()
        .ignore_then(text::keyword("STEP"))
        .ignore_then(hspace())
        .ignore_then(expr());

    text::keyword("FOR")
        .ignore_then(hspace())
        .ignore_then(var_name())
        .then_ignore(hspace())
        .then_ignore(just('='))
        .then_ignore(hspace())
        .then(expr())
        .then_ignore(hspace())
        .then_ignore(text::keyword("TO"))
        .then_ignore(hspace())
        .then(expr())
        .then(step.or_not())
        .map(|(((var, from), to), step)| Statement::For { var, from, to, step })
}

fn next_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("NEXT")
        .ignore_then(
            hspace().ignore_then(var_name()).or_not()
        )
        .map(|var| Statement::Next { var })
}

fn while_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("WHILE")
        .ignore_then(hspace())
        .ignore_then(expr())
        .map(|cond| Statement::While { cond })
}

fn wend_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("WEND").to(Statement::Wend)
}

// ---------------------------------------------------------------------------
// DO / LOOP
// ---------------------------------------------------------------------------

fn do_condition() -> impl Parser<char, DoCondition, Error = Simple<char>> {
    text::keyword("WHILE")
        .ignore_then(hspace())
        .ignore_then(expr())
        .map(DoCondition::While)
    .or(
        text::keyword("UNTIL")
            .ignore_then(hspace())
            .ignore_then(expr())
            .map(DoCondition::Until)
    )
}

fn do_loop_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("DO")
        .ignore_then(
            hspace()
                .ignore_then(do_condition())
                .or_not()
        )
        .map(|pre_cond| Statement::DoLoop { pre_cond })
}

fn loop_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("LOOP")
        .ignore_then(
            hspace()
                .ignore_then(do_condition())
                .or_not()
        )
        .map(|post_cond| Statement::Loop { post_cond })
}

// ---------------------------------------------------------------------------
// Console : SCREEN, WIDTH, COLOR, LOCATE, CLS, BEEP
// ---------------------------------------------------------------------------

fn screen_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("SCREEN")
        .ignore_then(hspace())
        .ignore_then(expr())
        .map(|mode| Statement::Screen { mode })
}

fn width_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("WIDTH")
        .ignore_then(hspace())
        .ignore_then(expr())
        .map(|cols| Statement::Width { cols })
}

fn color_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("COLOR")
        .ignore_then(hspace())
        .ignore_then(expr())
        .then(
            hspace()
                .ignore_then(just(','))
                .ignore_then(hspace())
                .ignore_then(expr())
                .or_not()
        )
        .map(|(fg, bg)| Statement::Color { fg, bg })
}

fn locate_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("LOCATE")
        .ignore_then(hspace())
        .ignore_then(expr())
        .then_ignore(hspace())
        .then_ignore(just(','))
        .then_ignore(hspace())
        .then(expr())
        .map(|(row, col)| Statement::Locate { row, col })
}

fn cls_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("CLS").to(Statement::Cls)
}

fn beep_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("BEEP").to(Statement::Beep)
}

// ---------------------------------------------------------------------------
// IF multiligne
// ---------------------------------------------------------------------------

fn elseif_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("ELSEIF")
        .ignore_then(hspace())
        .ignore_then(expr())
        .then_ignore(hspace())
        .then_ignore(text::keyword("THEN"))
        .map(|cond| Statement::ElseIf { cond })
}

fn else_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("ELSE").to(Statement::Else)
}

fn end_if_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("END")
        .ignore_then(hspace())
        .ignore_then(text::keyword("IF"))
        .to(Statement::EndIf)
}

// ---------------------------------------------------------------------------
// statement()
// ---------------------------------------------------------------------------

fn statement() -> impl Parser<char, Statement, Error = Simple<char>> {
    recursive(|stmt_rec| {
        // --- IF sur une seule ligne ---
        let if_singleline = text::keyword("IF")
            .ignore_then(hspace())
            .ignore_then(expr())
            .then_ignore(hspace())
            .then_ignore(text::keyword("THEN"))
            .then_ignore(hspace())
            .then(stmt_rec.clone())
            .then(
                hspace()
                    .ignore_then(text::keyword("ELSE"))
                    .ignore_then(hspace())
                    .ignore_then(stmt_rec)
                    .or_not()
            )
            .map(|((cond, then_stmt), else_stmt)| Statement::If {
                cond,
                then_stmt: Box::new(then_stmt),
                else_stmt: else_stmt.map(Box::new),
            });

        // --- IF multiligne : IF cond THEN  (rien après THEN sur la ligne) ---
        let if_multiline = text::keyword("IF")
            .ignore_then(hspace())
            .ignore_then(expr())
            .then_ignore(hspace())
            .then_ignore(text::keyword("THEN"))
            .map(|cond| Statement::IfThen { cond });

        rem_stmt()
            // END … doit être tenté avant les mots-clés solo (END IF avant END SUB)
            .or(end_if_stmt())
            .or(end_sub_stmt())
            // DECLARE avant SUB (contient SUB comme second mot-clé)
            .or(declare_sub_stmt())
            .or(sub_stmt())
            .or(call_stmt())
            .or(dim_stmt())
            .or(print_stmt())
            .or(for_stmt())
            .or(next_stmt())
            .or(while_stmt())
            .or(wend_stmt())
            // DO/LOOP
            .or(do_loop_stmt())
            .or(loop_stmt())
            .or(gosub_stmt())
            .or(return_stmt())
            .or(goto_stmt())
            .or(sleep_stmt())
            .or(randomize_stmt())
            // Console
            .or(screen_stmt())
            .or(width_stmt())
            .or(color_stmt())
            .or(locate_stmt())
            .or(cls_stmt())
            .or(beep_stmt())
            // ELSEIF avant ELSE (ELSEIF contient ELSE comme préfixe)
            .or(elseif_stmt())
            .or(else_stmt())
            // IF sur une ligne en premier, multiligne en fallback
            .or(if_singleline)
            .or(if_multiline)
            .or(array_set_stmt())
            .or(label_stmt())
            .or(assign_stmt())
    })
}

fn line() -> impl Parser<char, Line, Error = Simple<char>> {
    let line_number = text::int(10)
        .map(|s: String| s.parse::<u64>().unwrap())
        .then_ignore(hspace())
        .or_not();

    hspace()
        .ignore_then(line_number)
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
