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

    // --- Affectation ---

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
