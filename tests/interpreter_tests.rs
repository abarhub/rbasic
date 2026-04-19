use rbasic::interpreter::run_with_output;
use rbasic::parser::parse;

fn run_program(source: &str) -> String {
    let program = parse(source).expect("parse error");
    let mut output = Vec::new();
    run_with_output(&program, &mut output);
    String::from_utf8(output).unwrap()
}

// --- Variables entières ---

#[test]
fn test_print_integer() {
    assert_eq!(run_program("PRINT 42"), "42\n");
}

#[test]
fn test_print_int_variable() {
    assert_eq!(run_program("X = 10\nPRINT X"), "10\n");
}

#[test]
fn test_assignment_let() {
    assert_eq!(run_program("LET X = 7\nPRINT X"), "7\n");
}

#[test]
fn test_assignment_without_let() {
    assert_eq!(run_program("X = 7\nPRINT X"), "7\n");
}

#[test]
fn test_assignment_overwrite() {
    assert_eq!(run_program("X = 1\nX = 2\nPRINT X"), "2\n");
}

#[test]
fn test_int_variable_default_zero() {
    assert_eq!(run_program("PRINT X"), "0\n");
}

// --- Variables chaînes ---

#[test]
fn test_print_string_literal() {
    assert_eq!(run_program(r#"PRINT "Bonjour""#), "Bonjour\n");
}

#[test]
fn test_string_variable_assign_and_print() {
    assert_eq!(run_program("A$ = \"hello\"\nPRINT A$"), "hello\n");
}

#[test]
fn test_string_variable_with_let() {
    assert_eq!(run_program("LET NOM$ = \"Alice\"\nPRINT NOM$"), "Alice\n");
}

#[test]
fn test_string_variable_default_empty() {
    assert_eq!(run_program("PRINT A$"), "\n");
}

#[test]
fn test_string_variable_overwrite() {
    assert_eq!(run_program("A$ = \"un\"\nA$ = \"deux\"\nPRINT A$"), "deux\n");
}

#[test]
fn test_string_variable_copy() {
    assert_eq!(run_program("A$ = \"test\"\nB$ = A$\nPRINT B$"), "test\n");
}

// --- DIM ---

#[test]
fn test_dim_initialise_a_vide() {
    assert_eq!(run_program("DIM NOM$(10)\nPRINT NOM$"), "\n");
}

#[test]
fn test_dim_puis_affectation() {
    assert_eq!(run_program("DIM NOM$(10)\nNOM$ = \"Alice\"\nPRINT NOM$"), "Alice\n");
}

#[test]
fn test_dim_tronque_si_trop_long() {
    assert_eq!(run_program("DIM A$(3)\nA$ = \"bonjour\"\nPRINT A$"), "bon\n");
}

#[test]
fn test_dim_ne_tronque_pas_si_assez_court() {
    assert_eq!(run_program("DIM A$(10)\nA$ = \"hi\"\nPRINT A$"), "hi\n");
}

// --- PRINT multi-params et types mixtes ---

#[test]
fn test_print_multiple_params() {
    assert_eq!(run_program(r#"PRINT "val", 99"#), "val 99\n");
}

#[test]
fn test_print_empty() {
    assert_eq!(run_program("PRINT"), "\n");
}

#[test]
fn test_print_mixed_int_string_vars() {
    let src = "X = 42\nNOM$ = \"Bob\"\nPRINT NOM$, X";
    assert_eq!(run_program(src), "Bob 42\n");
}

// --- Programme complet ---

#[test]
fn test_full_program() {
    let src = "10 LET X = 42\n20 Y = 8\n30 PRINT \"X =\", X\n40 PRINT \"Y =\", Y";
    assert_eq!(run_program(src), "X = 42\nY = 8\n");
}

#[test]
fn test_mixed_numbered_unnumbered_lines() {
    let src = "10 X = 5\nPRINT X";
    assert_eq!(run_program(src), "5\n");
}

// --- Expressions arithmétiques ---

#[test]
fn test_addition() {
    assert_eq!(run_program("PRINT 2 + 3"), "5\n");
}

#[test]
fn test_soustraction() {
    assert_eq!(run_program("PRINT 10 - 4"), "6\n");
}

#[test]
fn test_multiplication() {
    assert_eq!(run_program("PRINT 3 * 5"), "15\n");
}

#[test]
fn test_division() {
    assert_eq!(run_program("PRINT 10 / 3"), "3\n");
}

#[test]
fn test_modulo() {
    assert_eq!(run_program("PRINT 10 % 3"), "1\n");
}

#[test]
fn test_priorite_mul_sur_add() {
    assert_eq!(run_program("PRINT 2 + 3 * 4"), "14\n");
}

#[test]
fn test_parentheses() {
    assert_eq!(run_program("PRINT (2 + 3) * 4"), "20\n");
}

#[test]
fn test_expression_avec_variables() {
    assert_eq!(run_program("X = 6\nY = 7\nPRINT X * Y"), "42\n");
}

#[test]
fn test_affectation_expression() {
    assert_eq!(run_program("X = 2 + 3\nPRINT X"), "5\n");
}

// --- Comparaisons entières (résultat : -1 vrai, 0 faux) ---

#[test]
fn test_egal_vrai() {
    assert_eq!(run_program("PRINT 5 = 5"), "-1\n");
}

#[test]
fn test_egal_faux() {
    assert_eq!(run_program("PRINT 5 = 6"), "0\n");
}

#[test]
fn test_different_vrai() {
    assert_eq!(run_program("PRINT 5 <> 6"), "-1\n");
}

#[test]
fn test_inferieur_vrai() {
    assert_eq!(run_program("PRINT 3 < 5"), "-1\n");
}

#[test]
fn test_inferieur_faux() {
    assert_eq!(run_program("PRINT 5 < 3"), "0\n");
}

#[test]
fn test_superieur_vrai() {
    assert_eq!(run_program("PRINT 5 > 3"), "-1\n");
}

#[test]
fn test_inferieur_egal_vrai() {
    assert_eq!(run_program("PRINT 5 <= 5"), "-1\n");
}

#[test]
fn test_superieur_egal_faux() {
    assert_eq!(run_program("PRINT 3 >= 5"), "0\n");
}

#[test]
fn test_comparaison_avec_variable() {
    assert_eq!(run_program("X = 10\nPRINT X > 5"), "-1\n");
}

// --- Concaténation de chaînes ---

#[test]
fn test_concat_litteraux() {
    assert_eq!(run_program(r#"PRINT "bon" + "jour""#), "bonjour\n");
}

#[test]
fn test_concat_variables() {
    assert_eq!(run_program("A$ = \"bon\"\nB$ = \"jour\"\nPRINT A$ + B$"), "bonjour\n");
}

#[test]
fn test_concat_variable_et_litteral() {
    assert_eq!(run_program("NOM$ = \"Alice\"\nPRINT \"Bonjour \" + NOM$"), "Bonjour Alice\n");
}

#[test]
fn test_concat_chaine_triple() {
    assert_eq!(run_program(r#"PRINT "a" + "b" + "c""#), "abc\n");
}

#[test]
fn test_affectation_concat() {
    assert_eq!(run_program("A$ = \"bon\"\nA$ = A$ + \"jour\"\nPRINT A$"), "bonjour\n");
}
