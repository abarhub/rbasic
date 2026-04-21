use rbasic::ast::{Expr, Op, Statement, UnaryOp};
#[allow(unused_imports)]
use rbasic::ast::JumpTarget;
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
    assert!(matches!(s, Statement::Dim { ref var, ref dims } if var == "NOM$" && dims == &[20]));
}

#[test]
fn test_dim_with_spaces() {
    let s = stmt("DIM NOM$( 30 )");
    assert!(matches!(s, Statement::Dim { ref var, ref dims } if var == "NOM$" && dims == &[30]));
}

#[test]
fn test_dim_with_line_number() {
    let l = line0("10 DIM TITRE$(50)");
    assert_eq!(l.number, Some(10));
    assert!(matches!(l.statement, Statement::Dim { ref var, ref dims } if var == "TITRE$" && dims == &[50]));
}

#[test]
fn test_dim_int_array_1d() {
    let s = stmt("DIM A(10)");
    assert!(matches!(s, Statement::Dim { ref var, ref dims } if var == "A" && dims == &[10]));
}

#[test]
fn test_dim_int_array_2d() {
    let s = stmt("DIM B(3, 4)");
    assert!(matches!(s, Statement::Dim { ref var, ref dims } if var == "B" && dims == &[3, 4]));
}

#[test]
fn test_dim_str_array_2d() {
    let s = stmt("DIM C$(2, 5)");
    assert!(matches!(s, Statement::Dim { ref var, ref dims } if var == "C$" && dims == &[2, 5]));
}

#[test]
fn test_array_set_1d() {
    let s = stmt("A(2) = 42");
    assert!(matches!(s, Statement::ArraySet { ref name, .. } if name == "A"));
}

#[test]
fn test_array_set_2d() {
    let s = stmt("B(1, 3) = 99");
    if let Statement::ArraySet { name, indices, .. } = s {
        assert_eq!(name, "B");
        assert_eq!(indices.len(), 2);
    } else {
        panic!("Expected ArraySet");
    }
}

#[test]
fn test_array_access_in_expr() {
    let s = stmt("X = A(0)");
    if let Statement::Let { value: Expr::ArrayAccess { ref name, ref indices }, .. } = s {
        assert_eq!(name, "A");
        assert_eq!(indices.len(), 1);
    } else {
        panic!("Expected Let with ArrayAccess");
    }
}

// --- PRINT ---

#[test]
fn test_print_string() {
    let s = stmt(r#"PRINT "bonjour""#);
    if let Statement::Print { values, .. } = s {
        assert_eq!(values.len(), 1);
        assert!(matches!(&values[0], Expr::StringLit(v) if v == "bonjour"));
    }
}

#[test]
fn test_print_integer() {
    let s = stmt("PRINT 99");
    if let Statement::Print { values, .. } = s {
        assert!(matches!(values[0], Expr::Integer(99)));
    }
}

#[test]
fn test_print_variable() {
    let s = stmt("PRINT X");
    if let Statement::Print { values, .. } = s {
        assert!(matches!(&values[0], Expr::Variable(v) if v == "X"));
    }
}

#[test]
fn test_print_string_variable() {
    let s = stmt("PRINT A$");
    if let Statement::Print { values, .. } = s {
        assert!(matches!(&values[0], Expr::Variable(v) if v == "A$"));
    }
}

#[test]
fn test_print_multiple_params() {
    let s = stmt(r#"PRINT "val", 1, X"#);
    if let Statement::Print { values, .. } = s {
        assert_eq!(values.len(), 3);
        assert!(matches!(&values[0], Expr::StringLit(v) if v == "val"));
        assert!(matches!(values[1], Expr::Integer(1)));
        assert!(matches!(&values[2], Expr::Variable(v) if v == "X"));
    }
}

#[test]
fn test_print_empty() {
    let s = stmt("PRINT");
    assert!(matches!(s, Statement::Print { values, .. } if values.is_empty()));
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
        Statement::Print { mut values, .. } => {
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
    let (op, _, _) = binop("PRINT 10 MOD 3");
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

// --- Opérateurs unaires ---

fn unary_expr(source: &str) -> (UnaryOp, Box<Expr>) {
    match stmt(source) {
        Statement::Print { mut values, .. } => match values.remove(0) {
            Expr::UnaryOp { op, operand } => (op, operand),
            _ => panic!("expected UnaryOp"),
        },
        _ => panic!("expected Print"),
    }
}

#[test]
fn test_unaire_negatif_litteral() {
    let (op, operand) = unary_expr("PRINT -5");
    assert!(matches!(op, UnaryOp::Neg));
    assert!(matches!(*operand, Expr::Integer(5)));
}

#[test]
fn test_unaire_negatif_variable() {
    let (op, operand) = unary_expr("PRINT -X");
    assert!(matches!(op, UnaryOp::Neg));
    assert!(matches!(*operand, Expr::Variable(ref v) if v == "X"));
}

#[test]
fn test_unaire_positif() {
    let (op, operand) = unary_expr("PRINT +5");
    assert!(matches!(op, UnaryOp::Pos));
    assert!(matches!(*operand, Expr::Integer(5)));
}

#[test]
fn test_unaire_double_negatif() {
    // --5 = -(-5)
    let (op, operand) = unary_expr("PRINT --5");
    assert!(matches!(op, UnaryOp::Neg));
    assert!(matches!(*operand, Expr::UnaryOp { op: UnaryOp::Neg, .. }));
}

#[test]
fn test_not_litteral() {
    let (op, operand) = unary_expr("PRINT NOT 0");
    assert!(matches!(op, UnaryOp::Not));
    assert!(matches!(*operand, Expr::Integer(0)));
}

#[test]
fn test_not_expression() {
    let (op, _) = unary_expr("PRINT NOT X < 5");
    assert!(matches!(op, UnaryOp::Not));
}

#[test]
fn test_affectation_negatif() {
    let s = stmt("X = -10");
    assert!(matches!(s, Statement::Let { var, value: Expr::UnaryOp { op: UnaryOp::Neg, .. } } if var == "X"));
}

// --- Opérateurs logiques AND / OR / XOR ---

#[test]
fn test_and() {
    let (op, _, _) = binop("PRINT X AND Y");
    assert!(matches!(op, Op::And));
}

#[test]
fn test_or() {
    let (op, _, _) = binop("PRINT X OR Y");
    assert!(matches!(op, Op::Or));
}

#[test]
fn test_xor() {
    let (op, _, _) = binop("PRINT X XOR Y");
    assert!(matches!(op, Op::Xor));
}

#[test]
fn test_precedence_not_avant_and() {
    // NOT X AND Y  doit être (NOT X) AND Y
    let (op, l, _) = binop("PRINT NOT X AND Y");
    assert!(matches!(op, Op::And));
    assert!(matches!(*l, Expr::UnaryOp { op: UnaryOp::Not, .. }));
}

#[test]
fn test_precedence_and_avant_or() {
    // X OR Y AND Z  doit être X OR (Y AND Z)
    let (op, _, r) = binop("PRINT X OR Y AND Z");
    assert!(matches!(op, Op::Or));
    assert!(matches!(*r, Expr::BinOp { op: Op::And, .. }));
}

#[test]
fn test_precedence_or_avant_xor() {
    // X XOR Y OR Z  doit être X XOR (Y OR Z)
    let (op, _, r) = binop("PRINT X XOR Y OR Z");
    assert!(matches!(op, Op::Xor));
    assert!(matches!(*r, Expr::BinOp { op: Op::Or, .. }));
}

// --- Labels ---

#[test]
fn test_label_stmt() {
    let s = stmt("MonLabel:");
    assert!(matches!(s, Statement::Label(ref n) if n == "MONLABEL"));
}

// --- GOTO ---

#[test]
fn test_goto_line_number() {
    let s = stmt("GOTO 10");
    assert!(matches!(s, Statement::Goto(rbasic::ast::JumpTarget::LineNumber(10))));
}

#[test]
fn test_goto_label() {
    let s = stmt("GOTO MonLabel");
    assert!(matches!(s, Statement::Goto(rbasic::ast::JumpTarget::Label(ref n)) if n == "MONLABEL"));
}

// --- IF ---

#[test]
fn test_if_then_no_else() {
    let s = stmt("IF 1 THEN X = 2");
    assert!(matches!(s, Statement::If { else_stmt: None, .. }));
}

#[test]
fn test_if_then_else() {
    let s = stmt("IF 1 THEN X = 2 ELSE X = 3");
    assert!(matches!(s, Statement::If { else_stmt: Some(_), .. }));
}

#[test]
fn test_if_condition_variable() {
    let s = stmt("IF X THEN Y = 1");
    if let Statement::If { cond, .. } = s {
        assert!(matches!(cond, Expr::Variable(ref n) if n == "X"));
    } else {
        panic!("Expected IF");
    }
}

// --- FOR / NEXT ---

#[test]
fn test_for_basic() {
    let s = stmt("FOR I = 1 TO 10");
    assert!(matches!(s, Statement::For { ref var, step: None, .. } if var == "I"));
}

#[test]
fn test_for_with_step() {
    let s = stmt("FOR I = 0 TO 20 STEP 2");
    assert!(matches!(s, Statement::For { ref var, step: Some(_), .. } if var == "I"));
}

#[test]
fn test_next_with_var() {
    let s = stmt("NEXT I");
    assert!(matches!(s, Statement::Next { var: Some(ref v) } if v == "I"));
}

#[test]
fn test_next_without_var() {
    let s = stmt("NEXT");
    assert!(matches!(s, Statement::Next { var: None }));
}

// --- WHILE / WEND ---

#[test]
fn test_while_stmt() {
    let s = stmt("WHILE X > 0");
    assert!(matches!(s, Statement::While { .. }));
}

#[test]
fn test_wend_stmt() {
    let s = stmt("WEND");
    assert!(matches!(s, Statement::Wend));
}

// --- GOSUB / RETURN ---

#[test]
fn test_gosub_line_number() {
    let s = stmt("GOSUB 100");
    assert!(matches!(s, Statement::Gosub(rbasic::ast::JumpTarget::LineNumber(100))));
}

#[test]
fn test_gosub_label() {
    let s = stmt("GOSUB maRoutine");
    assert!(matches!(s, Statement::Gosub(rbasic::ast::JumpTarget::Label(ref n)) if n == "MAROUTINE"));
}

#[test]
fn test_return_stmt() {
    let s = stmt("RETURN");
    assert!(matches!(s, Statement::Return));
}

// --- SUB / END SUB / CALL ---

#[test]
fn test_sub_sans_params() {
    let s = stmt("SUB MaSub");
    assert!(matches!(s, Statement::SubDef { ref name, ref params } if name == "MASUB" && params.is_empty()));
}

#[test]
fn test_sub_avec_params() {
    let s = stmt("SUB MaSub(A, B$)");
    if let Statement::SubDef { name, params } = s {
        assert_eq!(name, "MASUB");
        assert_eq!(params, vec!["A", "B$"]);
    } else { panic!("Expected SubDef"); }
}

#[test]
fn test_end_sub() {
    let s = stmt("END SUB");
    assert!(matches!(s, Statement::EndSub));
}

#[test]
fn test_call_sans_args() {
    let s = stmt("CALL MaSub");
    assert!(matches!(s, Statement::Call { ref name, ref args } if name == "MASUB" && args.is_empty()));
}

#[test]
fn test_call_avec_args() {
    let s = stmt("CALL MaSub(1, 2)");
    if let Statement::Call { name, args } = s {
        assert_eq!(name, "MASUB");
        assert_eq!(args.len(), 2);
    } else { panic!("Expected Call"); }
}

// --- Nombres flottants ---

#[test]
fn test_float_literal() {
    let s = stmt("X = 3.14");
    if let Statement::Let { var, value: Expr::Float(f) } = s {
        assert_eq!(var, "X");
        assert!((f - 3.14).abs() < 1e-10);
    } else {
        panic!("Expected Let with Float");
    }
}

#[test]
fn test_float_zero() {
    let s = stmt("X = 0.0");
    if let Statement::Let { value: Expr::Float(f), .. } = s {
        assert_eq!(f, 0.0);
    } else {
        panic!("Expected Float");
    }
}

#[test]
fn test_float_without_decimal_is_integer() {
    // "42" sans point décimal doit rester un entier
    let s = stmt("X = 42");
    assert!(matches!(s, Statement::Let { value: Expr::Integer(42), .. }));
}

#[test]
fn test_float_in_expression() {
    // 1.5 + 2.5 : les deux opérandes sont des flottants
    let (op, l, r) = binop("PRINT 1.5 + 2.5");
    assert!(matches!(op, Op::Add));
    if let Expr::Float(f) = *l { assert!((f - 1.5).abs() < 1e-10); } else { panic!("Expected Float left"); }
    if let Expr::Float(f) = *r { assert!((f - 2.5).abs() < 1e-10); } else { panic!("Expected Float right"); }
}

#[test]
fn test_float_mixed_expression() {
    // 3 + 1.5 : entier + flottant
    let (op, l, r) = binop("PRINT 3 + 1.5");
    assert!(matches!(op, Op::Add));
    assert!(matches!(*l, Expr::Integer(3)));
    if let Expr::Float(f) = *r { assert!((f - 1.5).abs() < 1e-10); } else { panic!("Expected Float right"); }
}

#[test]
fn test_float_in_for() {
    let s = stmt("FOR X = 1.0 TO 2.0 STEP 0.5");
    if let Statement::For { var, from: Expr::Float(f), to: Expr::Float(t), step: Some(Expr::Float(st)) } = s {
        assert_eq!(var, "X");
        assert!((f - 1.0).abs() < 1e-10);
        assert!((t - 2.0).abs() < 1e-10);
        assert!((st - 0.5).abs() < 1e-10);
    } else {
        panic!("Expected For with floats");
    }
}

#[test]
fn test_float_print() {
    let s = stmt("PRINT 3.14");
    if let Statement::Print { mut values, .. } = s {
        if let Expr::Float(f) = values.remove(0) {
            assert!((f - 3.14).abs() < 1e-10);
        } else {
            panic!("Expected Float in Print");
        }
    } else {
        panic!("Expected Print");
    }
}

// --- SLEEP ---

#[test]
fn test_sleep_entier() {
    let s = stmt("SLEEP 2");
    assert!(matches!(s, Statement::Sleep { duration: Expr::Integer(2) }));
}

#[test]
fn test_sleep_expression() {
    let s = stmt("SLEEP N");
    assert!(matches!(s, Statement::Sleep { duration: Expr::Variable(ref v) } if v == "N"));
}

// --- RANDOMIZE ---

#[test]
fn test_randomize_entier() {
    let s = stmt("RANDOMIZE 42");
    assert!(matches!(s, Statement::Randomize { seed: Expr::Integer(42) }));
}

#[test]
fn test_randomize_timer() {
    // TIMER est lu comme une variable
    let s = stmt("RANDOMIZE TIMER");
    assert!(matches!(s, Statement::Randomize { seed: Expr::Variable(ref v) } if v == "TIMER"));
}

// --- IF multiligne ---

#[test]
fn test_if_then_stmt() {
    let s = stmt("IF X > 0 THEN");
    assert!(matches!(s, Statement::IfThen { .. }));
}

#[test]
fn test_elseif_stmt() {
    let s = stmt("ELSEIF X = 2 THEN");
    assert!(matches!(s, Statement::ElseIf { .. }));
}

#[test]
fn test_else_stmt() {
    let s = stmt("ELSE");
    assert!(matches!(s, Statement::Else));
}

#[test]
fn test_end_if_stmt() {
    let s = stmt("END IF");
    assert!(matches!(s, Statement::EndIf));
}

#[test]
fn test_if_multiline_program_parses() {
    let src = "IF X > 0 THEN\n    PRINT \"pos\"\nELSEIF X = 0 THEN\n    PRINT \"zero\"\nELSE\n    PRINT \"neg\"\nEND IF";
    let prog = rbasic::parser::parse(src).expect("parse error");
    assert_eq!(prog.lines.len(), 7);
    assert!(matches!(prog.lines[0].statement, Statement::IfThen { .. }));
    assert!(matches!(prog.lines[2].statement, Statement::ElseIf { .. }));
    assert!(matches!(prog.lines[4].statement, Statement::Else));
    assert!(matches!(prog.lines[6].statement, Statement::EndIf));
}

// --- DO/LOOP ---

use rbasic::ast::DoCondition;

#[test]
fn test_do_while_stmt() {
    let s = stmt("DO WHILE X < 10");
    assert!(matches!(s, Statement::DoLoop { pre_cond: Some(DoCondition::While(_)) }));
}

#[test]
fn test_do_until_stmt() {
    let s = stmt("DO UNTIL X >= 10");
    assert!(matches!(s, Statement::DoLoop { pre_cond: Some(DoCondition::Until(_)) }));
}

#[test]
fn test_do_bare_stmt() {
    let s = stmt("DO");
    assert!(matches!(s, Statement::DoLoop { pre_cond: None }));
}

#[test]
fn test_loop_bare_stmt() {
    let s = stmt("LOOP");
    assert!(matches!(s, Statement::Loop { post_cond: None }));
}

#[test]
fn test_loop_while_stmt() {
    let s = stmt("LOOP WHILE I < 5");
    assert!(matches!(s, Statement::Loop { post_cond: Some(DoCondition::While(_)) }));
}

#[test]
fn test_loop_until_stmt() {
    let s = stmt("LOOP UNTIL I >= 5");
    assert!(matches!(s, Statement::Loop { post_cond: Some(DoCondition::Until(_)) }));
}

// --- DECLARE SUB ---

#[test]
fn test_declare_sub_no_params() {
    let s = stmt("DECLARE SUB Salut()");
    assert!(matches!(s, Statement::DeclareSub { ref name, ref params } if name == "SALUT" && params.is_empty()));
}

#[test]
fn test_declare_sub_with_params() {
    let s = stmt("DECLARE SUB Double(N, M)");
    if let Statement::DeclareSub { name, params } = s {
        assert_eq!(name, "DOUBLE");
        assert_eq!(params, vec!["N", "M"]);
    } else {
        panic!("Expected DeclareSub");
    }
}

// --- Console ---

#[test]
fn test_screen_stmt() {
    let s = stmt("SCREEN 0");
    assert!(matches!(s, Statement::Screen { .. }));
}

#[test]
fn test_width_stmt() {
    let s = stmt("WIDTH 80");
    assert!(matches!(s, Statement::Width { .. }));
}

#[test]
fn test_color_fg_only() {
    let s = stmt("COLOR 14");
    assert!(matches!(s, Statement::Color { bg: None, .. }));
}

#[test]
fn test_color_fg_bg() {
    let s = stmt("COLOR 14, 1");
    assert!(matches!(s, Statement::Color { bg: Some(_), .. }));
}

#[test]
fn test_locate_stmt() {
    let s = stmt("LOCATE 5, 10");
    assert!(matches!(s, Statement::Locate { .. }));
}

#[test]
fn test_cls_stmt() {
    let s = stmt("CLS");
    assert!(matches!(s, Statement::Cls));
}

#[test]
fn test_beep_stmt() {
    let s = stmt("BEEP");
    assert!(matches!(s, Statement::Beep));
}

// --- END ---

#[test]
fn test_end_stmt() {
    let s = stmt("END");
    assert!(matches!(s, Statement::End));
}

// --- Instructions multiples sur une ligne (:) ---

fn stmts(source: &str) -> Vec<Statement> {
    rbasic::parser::parse(source)
        .expect("parse error")
        .lines
        .into_iter()
        .map(|l| l.statement)
        .collect()
}

#[test]
fn test_multistatement_deux_stmts() {
    let v = stmts("A = 1 : B = 2");
    assert_eq!(v.len(), 2);
    assert!(matches!(&v[0], Statement::Let { var, .. } if var == "A"));
    assert!(matches!(&v[1], Statement::Let { var, .. } if var == "B"));
}

#[test]
fn test_multistatement_trois_stmts() {
    let v = stmts("A = 1 : B = 2 : PRINT A");
    assert_eq!(v.len(), 3);
}

#[test]
fn test_multistatement_numero_de_ligne_premier_seulement() {
    let prog = rbasic::parser::parse("10 A = 1 : B = 2").expect("parse error");
    assert_eq!(prog.lines[0].number, Some(10));
    assert_eq!(prog.lines[1].number, None);
}

#[test]
fn test_multistatement_rem_consomme_tout() {
    // REM consomme jusqu'à la fin de ligne : rien après
    let v = stmts("REM ceci est un commentaire : pas un stmt");
    assert_eq!(v.len(), 1);
    assert!(matches!(v[0], Statement::Rem));
}

// --- Fichier programme.bas complet ---

#[test]
fn test_parse_programme_bas_complet() {
    // Vérifie que programme.bas est parsé jusqu'au bout sans s'arrêter prématurément.
    // Le fichier contient ~80 lignes logiques (SCREEN, WIDTH, FOR, GOSUB, IF, etc.).
    let src = include_str!("../programmes/programme.bas");
    let program = rbasic::parser::parse(src.trim())
        .expect("programme.bas doit parser sans erreur");
    assert!(
        program.lines.len() >= 80,
        "Attendu ≥ 80 lignes parsées, obtenu {} — arrêt prématuré du parseur ?",
        program.lines.len()
    );
    // Vérifie la présence de quelques instructions clés
    let has_randomize = program.lines.iter().any(|l| matches!(l.statement, Statement::Randomize { .. }));
    let has_for       = program.lines.iter().any(|l| matches!(l.statement, Statement::For { .. }));
    let has_gosub     = program.lines.iter().any(|l| matches!(l.statement, Statement::Gosub(_)));
    assert!(has_randomize, "RANDOMIZE manquant dans le programme parsé");
    assert!(has_for,       "FOR manquant dans le programme parsé");
    assert!(has_gosub,     "GOSUB manquant dans le programme parsé");
}
