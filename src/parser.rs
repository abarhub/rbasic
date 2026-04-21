use chumsky::prelude::*;
use crate::ast::*;

// ---------------------------------------------------------------------------
// Prétraitement : normalisation de la casse et suppression des directives $
// ---------------------------------------------------------------------------

/// Met en majuscules tout le texte source sauf le contenu des littéraux chaîne.
fn normalize_case(src: &str) -> String {
    let mut result = String::with_capacity(src.len());
    let mut in_string = false;
    for c in src.chars() {
        if in_string {
            result.push(c);
            if c == '"' {
                in_string = false;
            }
        } else if c == '"' {
            in_string = true;
            result.push(c);
        } else {
            result.push(c.to_ascii_uppercase());
        }
    }
    result
}

/// Supprime la portion commentaire d'une ligne (après `'` non inclus dans un littéral string).
fn strip_line_comment(line: &str) -> &str {
    let mut in_string = false;
    for (i, c) in line.char_indices() {
        match c {
            '"' => in_string = !in_string,
            '\'' if !in_string => return line[..i].trim_end(),
            _ => {}
        }
    }
    line
}

/// Supprime les lignes commençant par '$' (directives de préprocesseur QBasic)
/// et les commentaires `'` (apostrophe) en fin de ligne ou sur toute une ligne.
fn preprocess(src: &str) -> String {
    src.lines()
        .filter(|line| !line.trim_start().starts_with('$'))
        .map(|line| strip_line_comment(line))
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

// ---------------------------------------------------------------------------
// Parseurs de base
// ---------------------------------------------------------------------------

fn hspace() -> impl Parser<char, (), Error = Simple<char>> {
    filter(|c: &char| *c == ' ' || *c == '\t').repeated().ignored()
}

fn integer() -> impl Parser<char, i64, Error = Simple<char>> {
    text::int(10).map(|s: String| s.parse::<i64>().unwrap())
}

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

/// Nom de variable : identifiant optionnellement suivi de '$'.
/// Les suffixes de type QBasic '!', '#', '%' sont acceptés et supprimés
/// (ils indiquent le type mais sont ignorés par l'interpréteur).
fn var_name() -> impl Parser<char, String, Error = Simple<char>> {
    text::ident()
        .then(
            just('$').to(true)
                .or(just('!').to(false))
                .or(just('#').to(false))
                .or(just('%').to(false))
                .or_not()
        )
        .map(|(name, opt): (String, Option<bool>)| match opt {
            Some(true)  => format!("{}$", name),
            Some(false) => name,   // suffixe de type supprimé
            None        => name,
        })
}

// ---------------------------------------------------------------------------
// Expressions
// ---------------------------------------------------------------------------

fn expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(|expr_rec| {
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

        let unary = recursive(|unary_rec| {
            just('-').ignore_then(hspace()).ignore_then(unary_rec.clone())
                .map(|e| Expr::UnaryOp { op: UnaryOp::Neg, operand: Box::new(e) })
            .or(just('+').ignore_then(hspace()).ignore_then(unary_rec)
                .map(|e| Expr::UnaryOp { op: UnaryOp::Pos, operand: Box::new(e) }))
            .or(atom)
        }).boxed();

        let mul_op = just('*').to(Op::Mul)
            .or(just('/').to(Op::Div))
            .or(text::keyword("MOD").to(Op::Mod));

        let mul = unary.clone()
            .then(hspace().ignore_then(mul_op).then_ignore(hspace()).then(unary).repeated())
            .foldl(|l, (op, r)| Expr::BinOp { op, left: Box::new(l), right: Box::new(r) })
            .boxed();

        let add_op = just('+').to(Op::Add).or(just('-').to(Op::Sub));

        let add = mul.clone()
            .then(hspace().ignore_then(add_op).then_ignore(hspace()).then(mul).repeated())
            .foldl(|l, (op, r)| Expr::BinOp { op, left: Box::new(l), right: Box::new(r) })
            .boxed();

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

        let not_level = text::keyword("NOT")
            .ignore_then(hspace())
            .ignore_then(cmp.clone())
            .map(|e| Expr::UnaryOp { op: UnaryOp::Not, operand: Box::new(e) })
            .or(cmp)
            .boxed();

        let and_level = not_level.clone()
            .then(
                hspace().ignore_then(text::keyword("AND").to(Op::And))
                    .then_ignore(hspace()).then(not_level).repeated()
            )
            .foldl(|l, (op, r)| Expr::BinOp { op, left: Box::new(l), right: Box::new(r) })
            .boxed();

        let or_level = and_level.clone()
            .then(
                hspace().ignore_then(text::keyword("OR").to(Op::Or))
                    .then_ignore(hspace()).then(and_level).repeated()
            )
            .foldl(|l, (op, r)| Expr::BinOp { op, left: Box::new(l), right: Box::new(r) })
            .boxed();

        or_level.clone()
            .then(
                hspace().ignore_then(text::keyword("XOR").to(Op::Xor))
                    .then_ignore(hspace()).then(or_level).repeated()
            )
            .foldl(|l, (op, r)| Expr::BinOp { op, left: Box::new(l), right: Box::new(r) })
    })
}

// ---------------------------------------------------------------------------
// Instructions
// ---------------------------------------------------------------------------

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

/// Parseur d'une seule déclaration de tableau : NOM$(dim1[, dim2, ...])
fn dim_one() -> impl Parser<char, (String, Vec<usize>), Error = Simple<char>> {
    var_name()
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
}

/// DIM supporte plusieurs tableaux sur une ligne : DIM A$(3), B(10, 5)
fn dim_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("DIM")
        .ignore_then(hspace())
        .ignore_then(
            dim_one()
                .then_ignore(hspace())
                .separated_by(just(',').then_ignore(hspace()))
                .at_least(1)
        )
        .map(|items| {
            if items.len() == 1 {
                let (var, dims) = items.into_iter().next().unwrap();
                Statement::Dim { var, dims }
            } else {
                Statement::DimMulti { items }
            }
        })
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

/// PRINT supporte ';' et ',' comme séparateurs, et un ';' final supprime le saut de ligne.
fn print_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    let sep = just(';').to(true).or(just(',').to(false));

    text::keyword("PRINT")
        .ignore_then(hspace())
        .ignore_then(
            expr()
                .then_ignore(hspace())
                .then(
                    sep.then_ignore(hspace())
                        .then(expr())
                        .then_ignore(hspace())
                        .repeated()
                )
                .then(sep.or_not())
                .map(|((first, rest), trailing)| {
                    let mut values = vec![first];
                    let mut separators = Vec::new();
                    for (s, v) in rest {
                        separators.push(s);
                        values.push(v);
                    }
                    let no_newline = trailing.is_some();
                    Statement::Print { values, separators, no_newline }
                })
                .or_not()
        )
        .map(|opt| match opt {
            Some(s) => s,
            None    => Statement::Print { values: vec![], separators: vec![], no_newline: false },
        })
}

fn rem_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    // REM keyword
    let rem_kw = text::keyword("REM")
        .ignore_then(filter(|c: &char| *c != '\n' && *c != '\r').repeated());
    // Apostrophe inline comment : ' ...
    let apostrophe = just('\'')
        .ignore_then(filter(|c: &char| *c != '\n' && *c != '\r').repeated());
    rem_kw.or(apostrophe).to(Statement::Rem)
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

/// Paramètres partagés entre SUB et DECLARE SUB.
/// Accepte optionnellement "AS typename" après chaque paramètre (suffixe de type QBasic).
fn param_list() -> impl Parser<char, Vec<String>, Error = Simple<char>> {
    hspace()
        .ignore_then(just('('))
        .ignore_then(hspace())
        .ignore_then(
            var_name()
                .then_ignore(hspace())
                .then_ignore(
                    text::keyword("AS")
                        .ignore_then(hspace())
                        .ignore_then(text::ident())
                        .or_not()
                )
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
    // Reconnaît "END SUB" (avec espace) et "ENDSUB" (sans espace)
    let two_words = text::keyword("END")
        .ignore_then(hspace())
        .ignore_then(text::keyword("SUB"));
    let one_word = text::keyword("ENDSUB");
    two_words.or(one_word).to(Statement::EndSub)
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

/// NEXT supporte plusieurs variables : NEXT K, J, I
/// → NextMulti { vars: ["K", "J", "I"] } si >1 var, Next { var } sinon.
fn next_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("NEXT")
        .ignore_then(
            hspace()
                .ignore_then(
                    var_name()
                        .then_ignore(hspace())
                        .separated_by(just(',').then_ignore(hspace()))
                        .at_least(1)
                )
                .or_not()
        )
        .map(|vars_opt| match vars_opt {
            None => Statement::Next { var: None },
            Some(vars) if vars.len() == 1 => {
                Statement::Next { var: Some(vars.into_iter().next().unwrap()) }
            }
            Some(vars) => Statement::NextMulti { vars },
        })
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
// Console : SCREEN, WIDTH, COLOR, LOCATE, CLS, BEEP, KEY
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

/// KEY ON/OFF/... — no-op ; consomme le reste de l'instruction jusqu'au prochain ':'  ou fin de ligne.
fn key_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("KEY")
        .ignore_then(
            filter(|c: &char| *c != '\n' && *c != '\r' && *c != ':')
                .repeated()
        )
        .to(Statement::Key)
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
    // Reconnaît "END IF" (avec espace) et "ENDIF" (sans espace)
    let two_words = text::keyword("END")
        .ignore_then(hspace())
        .ignore_then(text::keyword("IF"));
    let one_word = text::keyword("ENDIF");
    two_words.or(one_word).to(Statement::EndIf)
}

fn end_stmt() -> impl Parser<char, Statement, Error = Simple<char>> {
    text::keyword("END").to(Statement::End)
}

// ---------------------------------------------------------------------------
// statement()
// ---------------------------------------------------------------------------

fn statement() -> impl Parser<char, Statement, Error = Simple<char>> {
    recursive(|stmt_rec| {
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

        let if_multiline = text::keyword("IF")
            .ignore_then(hspace())
            .ignore_then(expr())
            .then_ignore(hspace())
            .then_ignore(text::keyword("THEN"))
            .map(|cond| Statement::IfThen { cond });

        rem_stmt()
            // END … : ordre obligatoire END IF > END SUB > END seul
            .or(end_if_stmt())
            .or(end_sub_stmt())
            .or(end_stmt())
            // DECLARE avant SUB
            .or(declare_sub_stmt())
            .or(sub_stmt())
            .or(call_stmt())
            .or(dim_stmt())
            .or(print_stmt())
            .or(for_stmt())
            .or(next_stmt())
            .or(while_stmt())
            .or(wend_stmt())
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
            .or(key_stmt())
            // ELSEIF avant ELSE
            .or(elseif_stmt())
            .or(else_stmt())
            .or(if_singleline)
            .or(if_multiline)
            .or(array_set_stmt())
            .or(label_stmt())
            .or(assign_stmt())
    })
}

// ---------------------------------------------------------------------------
// Parseur de ligne : gère l'expansion de NextMulti et DimMulti
// ---------------------------------------------------------------------------

fn line() -> impl Parser<char, Vec<Line>, Error = Simple<char>> {
    let line_number = text::int(10)
        .map(|s: String| s.parse::<u64>().unwrap())
        .then_ignore(hspace())
        .or_not();

    hspace()
        .ignore_then(line_number)
        .then(
            statement()
                .separated_by(
                    hspace().ignore_then(just(':')).ignore_then(hspace())
                )
                .at_least(1)
        )
        .map(|(number, stmts)| {
            let mut lines: Vec<Line> = Vec::new();
            let mut num: Option<u64> = number;

            for stmt in stmts {
                // Expansion des instructions multi-variants
                let expanded: Vec<Statement> = match stmt {
                    Statement::NextMulti { vars } => {
                        vars.into_iter()
                            .map(|v| Statement::Next { var: Some(v) })
                            .collect()
                    }
                    Statement::DimMulti { items } => {
                        items.into_iter()
                            .map(|(var, dims)| Statement::Dim { var, dims })
                            .collect()
                    }
                    other => vec![other],
                };

                for s in expanded {
                    lines.push(Line {
                        number: num.take(), // premier → numéro, reste → None
                        statement: s,
                    });
                }
            }
            lines
        })
}

// ---------------------------------------------------------------------------
// Point d'entrée public
// ---------------------------------------------------------------------------

pub fn parse(source: &str) -> Result<Program, Vec<Simple<char>>> {
    let preprocessed = preprocess(source);
    let normalized   = normalize_case(&preprocessed);

    line()
        .separated_by(
            filter(|c: &char| *c == '\r' || *c == '\n').repeated().at_least(1)
        )
        .allow_leading()
        .allow_trailing()
        .map(|lines_vec| Program {
            lines: lines_vec.into_iter().flatten().collect(),
        })
        .parse(normalized.as_str())
}
