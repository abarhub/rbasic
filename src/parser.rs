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

// Identifiant avec $ optionnel en suffixe pour les variables chaînes
fn var_name() -> impl Parser<char, String, Error = Simple<char>> {
    text::ident()
        .then(just('$').or_not())
        .map(|(name, dollar): (String, Option<char>)| {
            if dollar.is_some() {
                format!("{}$", name)
            } else {
                name
            }
        })
}

fn expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    string_lit().map(Expr::StringLit)
        .or(integer().map(Expr::Integer))
        .or(var_name().map(Expr::Variable))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn stmt(source: &str) -> Statement {
        parse(source).expect("parse error").lines.remove(0).statement
    }

    fn line0(source: &str) -> Line {
        parse(source).expect("parse error").lines.remove(0)
    }

    // --- Numéros de ligne ---

    #[test]
    fn test_line_with_number() {
        let l = line0("10 X = 5");
        assert!(matches!(l.number, Some(10)));
    }

    #[test]
    fn test_line_without_number() {
        let l = line0("X = 5");
        assert!(l.number.is_none());
    }

    // --- Affectation entière ---

    #[test]
    fn test_let_with_keyword() {
        let s = stmt("LET X = 42");
        assert!(matches!(s, Statement::Let { var, value: Expr::Integer(42) } if var == "X"));
    }

    #[test]
    fn test_let_without_keyword() {
        let s = stmt("X = 42");
        assert!(matches!(s, Statement::Let { var, value: Expr::Integer(42) } if var == "X"));
    }

    #[test]
    fn test_let_with_line_number() {
        let l = line0("20 LET Y = 7");
        assert_eq!(l.number, Some(20));
        assert!(matches!(l.statement, Statement::Let { var, value: Expr::Integer(7) } if var == "Y"));
    }

    // --- Affectation chaîne ---

    #[test]
    fn test_let_string_variable_direct() {
        let s = stmt(r#"A$ = "bonjour""#);
        assert!(matches!(s, Statement::Let { var, value: Expr::StringLit(_) } if var == "A$"));
    }

    #[test]
    fn test_let_string_variable_with_let() {
        let s = stmt(r#"LET NOM$ = "Alice""#);
        assert!(matches!(s, Statement::Let { var, value: Expr::StringLit(v) } if var == "NOM$" && v == "Alice"));
    }

    #[test]
    fn test_let_string_variable_from_string_var() {
        let s = stmt("A$ = B$");
        assert!(matches!(s, Statement::Let { var, value: Expr::Variable(src) } if var == "A$" && src == "B$"));
    }

    // --- DIM ---

    #[test]
    fn test_dim_string_variable() {
        let s = stmt("DIM NOM$(20)");
        assert!(matches!(s, Statement::Dim { var, size: 20 } if var == "NOM$"));
    }

    #[test]
    fn test_dim_with_spaces() {
        let s = stmt("DIM NOM$( 30 )");
        assert!(matches!(s, Statement::Dim { var, size: 30 } if var == "NOM$"));
    }

    #[test]
    fn test_dim_with_line_number() {
        let l = line0("10 DIM TITRE$(50)");
        assert_eq!(l.number, Some(10));
        assert!(matches!(l.statement, Statement::Dim { var, size: 50 } if var == "TITRE$"));
    }

    // --- PRINT ---

    #[test]
    fn test_print_string() {
        let s = stmt(r#"PRINT "bonjour""#);
        if let Statement::Print { values } = s {
            assert_eq!(values.len(), 1);
            assert!(matches!(&values[0], Expr::StringLit(v) if v == "bonjour"));
        }
    }

    #[test]
    fn test_print_integer() {
        let s = stmt("PRINT 99");
        if let Statement::Print { values } = s {
            assert!(matches!(values[0], Expr::Integer(99)));
        }
    }

    #[test]
    fn test_print_variable() {
        let s = stmt("PRINT X");
        if let Statement::Print { values } = s {
            assert!(matches!(&values[0], Expr::Variable(v) if v == "X"));
        }
    }

    #[test]
    fn test_print_string_variable() {
        let s = stmt("PRINT A$");
        if let Statement::Print { values } = s {
            assert!(matches!(&values[0], Expr::Variable(v) if v == "A$"));
        }
    }

    #[test]
    fn test_print_multiple_params() {
        let s = stmt(r#"PRINT "val", 1, X"#);
        if let Statement::Print { values } = s {
            assert_eq!(values.len(), 3);
            assert!(matches!(&values[0], Expr::StringLit(v) if v == "val"));
            assert!(matches!(values[1], Expr::Integer(1)));
            assert!(matches!(&values[2], Expr::Variable(v) if v == "X"));
        }
    }

    #[test]
    fn test_print_empty() {
        let s = stmt("PRINT");
        assert!(matches!(s, Statement::Print { values } if values.is_empty()));
    }

    // --- Programme complet ---

    #[test]
    fn test_program_multiple_lines() {
        let prog = parse("LET X = 1\nPRINT X").expect("parse error");
        assert_eq!(prog.lines.len(), 2);
    }

    #[test]
    fn test_program_blank_lines() {
        let prog = parse("X = 1\n\nPRINT X").expect("parse error");
        assert_eq!(prog.lines.len(), 2);
    }

    #[test]
    fn test_program_mixed_numbered_unnumbered() {
        let prog = parse("10 X = 5\nPRINT X").expect("parse error");
        assert_eq!(prog.lines[0].number, Some(10));
        assert!(prog.lines[1].number.is_none());
    }
}
