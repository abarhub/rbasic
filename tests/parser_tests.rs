use rbasic::ast::{Expr, Op, Statement};
use rbasic::parser::parse;

fn stmt(source: &str) -> Statement {
    parse(source).expect("parse error").lines.remove(0).statement
}

fn line0(source: &str) -> rbasic::ast::Line {
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

// --- Expressions arithmétiques entières ---

fn binop(s: &str) -> (Op, Box<Expr>, Box<Expr>) {
    match stmt(s) {
        Statement::Print { mut values } => {
            match values.remove(0) {
                Expr::BinOp { op, left, right } => (op, left, right),
                _ => panic!("expected BinOp"),
            }
        }
        _ => panic!("expected Print"),
    }
}

#[test]
fn test_expr_addition() {
    let (op, l, r) = binop("PRINT 2 + 3");
    assert!(matches!(op, Op::Add));
    assert!(matches!(*l, Expr::Integer(2)));
    assert!(matches!(*r, Expr::Integer(3)));
}

#[test]
fn test_expr_soustraction() {
    let (op, l, r) = binop("PRINT 10 - 4");
    assert!(matches!(op, Op::Sub));
    assert!(matches!(*l, Expr::Integer(10)));
    assert!(matches!(*r, Expr::Integer(4)));
}

#[test]
fn test_expr_multiplication() {
    let (op, l, r) = binop("PRINT 3 * 5");
    assert!(matches!(op, Op::Mul));
    assert!(matches!(*l, Expr::Integer(3)));
    assert!(matches!(*r, Expr::Integer(5)));
}

#[test]
fn test_expr_division() {
    let (op, _, _) = binop("PRINT 10 / 2");
    assert!(matches!(op, Op::Div));
}

#[test]
fn test_expr_modulo() {
    let (op, _, _) = binop("PRINT 10 % 3");
    assert!(matches!(op, Op::Mod));
}

#[test]
fn test_expr_priorite_mul_sur_add() {
    // 2 + 3 * 4 doit être parsé comme 2 + (3 * 4)
    let (op, l, r) = binop("PRINT 2 + 3 * 4");
    assert!(matches!(op, Op::Add));
    assert!(matches!(*l, Expr::Integer(2)));
    assert!(matches!(*r, Expr::BinOp { op: Op::Mul, .. }));
}

#[test]
fn test_expr_parentheses() {
    // (2 + 3) * 4 doit être parsé comme (2 + 3) * 4
    let (op, l, r) = binop("PRINT (2 + 3) * 4");
    assert!(matches!(op, Op::Mul));
    assert!(matches!(*l, Expr::BinOp { op: Op::Add, .. }));
    assert!(matches!(*r, Expr::Integer(4)));
}

// --- Opérateurs de comparaison ---

#[test]
fn test_expr_egal() {
    let (op, _, _) = binop("PRINT X = 5");
    assert!(matches!(op, Op::Eq));
}

#[test]
fn test_expr_different() {
    let (op, _, _) = binop("PRINT X <> 5");
    assert!(matches!(op, Op::Ne));
}

#[test]
fn test_expr_inferieur() {
    let (op, _, _) = binop("PRINT X < 5");
    assert!(matches!(op, Op::Lt));
}

#[test]
fn test_expr_superieur() {
    let (op, _, _) = binop("PRINT X > 5");
    assert!(matches!(op, Op::Gt));
}

#[test]
fn test_expr_inferieur_egal() {
    let (op, _, _) = binop("PRINT X <= 5");
    assert!(matches!(op, Op::Le));
}

#[test]
fn test_expr_superieur_egal() {
    let (op, _, _) = binop("PRINT X >= 5");
    assert!(matches!(op, Op::Ge));
}

// --- Concaténation de chaînes ---

#[test]
fn test_expr_concat_litteraux() {
    let (op, l, r) = binop(r#"PRINT "bon" + "jour""#);
    assert!(matches!(op, Op::Add));
    assert!(matches!(*l, Expr::StringLit(ref s) if s == "bon"));
    assert!(matches!(*r, Expr::StringLit(ref s) if s == "jour"));
}

#[test]
fn test_expr_concat_variables() {
    let (op, l, r) = binop("PRINT A$ + B$");
    assert!(matches!(op, Op::Add));
    assert!(matches!(*l, Expr::Variable(ref v) if v == "A$"));
    assert!(matches!(*r, Expr::Variable(ref v) if v == "B$"));
}

#[test]
fn test_affectation_avec_expression() {
    let s = stmt("X = 2 + 3");
    assert!(matches!(s, Statement::Let { var, value: Expr::BinOp { op: Op::Add, .. } } if var == "X"));
}
