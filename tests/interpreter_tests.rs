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

// --- Opérateurs unaires ---

#[test]
fn test_negatif_litteral() {
    assert_eq!(run_program("PRINT -5"), "-5\n");
}

#[test]
fn test_negatif_variable() {
    assert_eq!(run_program("X = 3\nPRINT -X"), "-3\n");
}

#[test]
fn test_positif_unaire() {
    assert_eq!(run_program("PRINT +7"), "7\n");
}

#[test]
fn test_double_negatif() {
    assert_eq!(run_program("PRINT --5"), "5\n");
}

#[test]
fn test_negatif_dans_expression() {
    assert_eq!(run_program("PRINT 10 + -3"), "7\n");
}

#[test]
fn test_negatif_affectation() {
    assert_eq!(run_program("X = -42\nPRINT X"), "-42\n");
}

// --- NOT ---

#[test]
fn test_not_zero_donne_moins_un() {
    assert_eq!(run_program("PRINT NOT 0"), "-1\n");
}

#[test]
fn test_not_moins_un_donne_zero() {
    assert_eq!(run_program("PRINT NOT -1"), "0\n");
}

#[test]
fn test_not_comparaison_vraie() {
    // 3 < 5 = -1, NOT -1 = 0
    assert_eq!(run_program("PRINT NOT 3 < 5"), "0\n");
}

#[test]
fn test_not_comparaison_fausse() {
    // 5 < 3 = 0, NOT 0 = -1
    assert_eq!(run_program("PRINT NOT 5 < 3"), "-1\n");
}

// --- AND / OR / XOR ---

#[test]
fn test_and_bit_a_bit() {
    assert_eq!(run_program("PRINT 6 AND 3"), "2\n");   // 0110 & 0011 = 0010
}

#[test]
fn test_or_bit_a_bit() {
    assert_eq!(run_program("PRINT 6 OR 3"), "7\n");    // 0110 | 0011 = 0111
}

#[test]
fn test_xor_bit_a_bit() {
    assert_eq!(run_program("PRINT 6 XOR 3"), "5\n");   // 0110 ^ 0011 = 0101
}

#[test]
fn test_and_logique() {
    // (-1) AND (-1) = -1 (vrai AND vrai)
    assert_eq!(run_program("PRINT (3 > 1) AND (5 > 2)"), "-1\n");
}

#[test]
fn test_or_logique() {
    // (-1) OR 0 = -1 (vrai OR faux)
    assert_eq!(run_program("PRINT (3 > 1) OR (2 > 5)"), "-1\n");
}

#[test]
fn test_precedence_cmp_avant_not() {
    // NOT (3 < 5) = NOT (-1) = 0
    assert_eq!(run_program("PRINT NOT 3 < 5"), "0\n");
}

#[test]
fn test_precedence_not_avant_and() {
    // (NOT 0) AND -1 = (-1) AND (-1) = -1
    assert_eq!(run_program("PRINT NOT 0 AND -1"), "-1\n");
}

// --- GOTO et labels ---

#[test]
fn test_goto_line_number() {
    let src = "10 X = 1\n20 GOTO 40\n30 X = 2\n40 PRINT X";
    assert_eq!(run_program(src), "1\n");
}

#[test]
fn test_goto_label() {
    let src = "X = 1\nGOTO fin\nX = 2\nfin:\nPRINT X";
    assert_eq!(run_program(src), "1\n");
}

// --- IF / THEN / ELSE ---

#[test]
fn test_if_then_vrai() {
    assert_eq!(run_program("IF 1 THEN PRINT \"oui\""), "oui\n");
}

#[test]
fn test_if_then_faux() {
    assert_eq!(run_program("IF 0 THEN PRINT \"oui\"\nPRINT \"non\""), "non\n");
}

#[test]
fn test_if_then_else_vrai() {
    assert_eq!(run_program("IF 1 THEN PRINT \"oui\" ELSE PRINT \"non\""), "oui\n");
}

#[test]
fn test_if_then_else_faux() {
    assert_eq!(run_program("IF 0 THEN PRINT \"oui\" ELSE PRINT \"non\""), "non\n");
}

#[test]
fn test_if_avec_comparaison() {
    let src = "X = 5\nIF X > 3 THEN PRINT \"grand\" ELSE PRINT \"petit\"";
    assert_eq!(run_program(src), "grand\n");
}

#[test]
fn test_if_affectation_then() {
    let src = "X = 10\nIF X = 10 THEN Y = 99\nPRINT Y";
    assert_eq!(run_program(src), "99\n");
}

// --- FOR / NEXT ---

#[test]
fn test_for_simple() {
    let src = "FOR I = 1 TO 3\nPRINT I\nNEXT I";
    assert_eq!(run_program(src), "1\n2\n3\n");
}

#[test]
fn test_for_step_2() {
    let src = "FOR I = 0 TO 6 STEP 2\nPRINT I\nNEXT I";
    assert_eq!(run_program(src), "0\n2\n4\n6\n");
}

#[test]
fn test_for_step_negatif() {
    let src = "FOR I = 3 TO 1 STEP -1\nPRINT I\nNEXT I";
    assert_eq!(run_program(src), "3\n2\n1\n");
}

#[test]
fn test_for_zero_iteration() {
    // FROM > TO avec step positif : boucle non exécutée
    let src = "FOR I = 5 TO 1\nPRINT I\nNEXT I\nPRINT \"fin\"";
    assert_eq!(run_program(src), "fin\n");
}

#[test]
fn test_for_next_sans_var() {
    let src = "FOR I = 1 TO 3\nPRINT I\nNEXT";
    assert_eq!(run_program(src), "1\n2\n3\n");
}

#[test]
fn test_for_accumulation() {
    let src = "S = 0\nFOR I = 1 TO 5\nS = S + I\nNEXT I\nPRINT S";
    assert_eq!(run_program(src), "15\n");
}

#[test]
fn test_for_imbriques() {
    let src = "FOR I = 1 TO 2\nFOR J = 1 TO 2\nPRINT I\nNEXT J\nNEXT I";
    assert_eq!(run_program(src), "1\n1\n2\n2\n");
}

// --- WHILE / WEND ---

#[test]
fn test_while_simple() {
    let src = "X = 1\nWHILE X <= 3\nPRINT X\nX = X + 1\nWEND";
    assert_eq!(run_program(src), "1\n2\n3\n");
}

#[test]
fn test_while_zero_iteration() {
    let src = "X = 0\nWHILE X > 0\nPRINT X\nWEND\nPRINT \"fin\"";
    assert_eq!(run_program(src), "fin\n");
}

#[test]
fn test_while_compte_a_rebours() {
    let src = "N = 3\nWHILE N > 0\nPRINT N\nN = N - 1\nWEND";
    assert_eq!(run_program(src), "3\n2\n1\n");
}

// --- GOSUB / RETURN ---

#[test]
fn test_gosub_simple() {
    let src = "GOSUB affiche\nGOTO fin\naffiche:\nPRINT \"bonjour\"\nRETURN\nfin:";
    assert_eq!(run_program(src), "bonjour\n");
}

#[test]
fn test_gosub_retour_correct() {
    // Après RETURN on continue après le GOSUB
    let src = "PRINT \"avant\"\nGOSUB routine\nPRINT \"apres\"\nGOTO fin\nroutine:\nPRINT \"routine\"\nRETURN\nfin:";
    assert_eq!(run_program(src), "avant\nroutine\napres\n");
}

#[test]
fn test_gosub_multiple_appels() {
    let src = "GOSUB inc\nGOSUB inc\nGOSUB inc\nPRINT X\nGOTO fin\ninc:\nX = X + 1\nRETURN\nfin:";
    assert_eq!(run_program(src), "3\n");
}

#[test]
fn test_gosub_line_number() {
    let src = "10 GOSUB 30\n20 GOTO 50\n30 PRINT \"sub\"\n40 RETURN\n50 PRINT \"fin\"";
    assert_eq!(run_program(src), "sub\nfin\n");
}

#[test]
fn test_gosub_imbriques() {
    // Un sous-programme appelle un autre sous-programme
    let src = "GOSUB a\nPRINT \"retour main\"\nGOTO fin\na:\nPRINT \"debut a\"\nGOSUB b\nPRINT \"fin a\"\nRETURN\nb:\nPRINT \"dans b\"\nRETURN\nfin:";
    assert_eq!(run_program(src), "debut a\ndans b\nfin a\nretour main\n");
}

// --- Tableaux 1D entiers ---

#[test]
fn test_tableau_int_1d_affectation_lecture() {
    let src = "DIM A(5)\nA(0) = 10\nA(1) = 20\nPRINT A(0), A(1)";
    assert_eq!(run_program(src), "10 20\n");
}

#[test]
fn test_tableau_int_valeur_defaut_zero() {
    let src = "DIM A(3)\nPRINT A(2)";
    assert_eq!(run_program(src), "0\n");
}

#[test]
fn test_tableau_int_indice_max() {
    // DIM A(n) permet les indices 0..n inclus
    let src = "DIM A(3)\nA(3) = 99\nPRINT A(3)";
    assert_eq!(run_program(src), "99\n");
}

#[test]
fn test_tableau_int_expression_indice() {
    let src = "DIM A(10)\nI = 3\nA(I) = 42\nPRINT A(I)";
    assert_eq!(run_program(src), "42\n");
}

#[test]
fn test_tableau_int_expression_valeur() {
    let src = "DIM A(5)\nA(0) = 3 * 7\nPRINT A(0)";
    assert_eq!(run_program(src), "21\n");
}

// --- Tableaux 1D chaînes ---

#[test]
fn test_tableau_str_1d() {
    let src = "DIM NOMS$(3)\nNOMS$(0) = \"Alice\"\nNOMS$(1) = \"Bob\"\nPRINT NOMS$(0), NOMS$(1)";
    assert_eq!(run_program(src), "Alice Bob\n");
}

#[test]
fn test_tableau_str_valeur_defaut_vide() {
    let src = "DIM S$(5)\nPRINT \"|\" + S$(0) + \"|\"";
    assert_eq!(run_program(src), "||\n");
}

// --- Tableaux 2D ---

#[test]
fn test_tableau_2d_affectation_lecture() {
    let src = "DIM M(2, 3)\nM(1, 2) = 55\nPRINT M(1, 2)";
    assert_eq!(run_program(src), "55\n");
}

#[test]
fn test_tableau_2d_valeur_defaut_zero() {
    let src = "DIM M(3, 3)\nPRINT M(0, 0)";
    assert_eq!(run_program(src), "0\n");
}

#[test]
fn test_tableau_2d_plusieurs_cases() {
    let src = "DIM M(2, 2)\nM(0, 0) = 1\nM(0, 1) = 2\nM(1, 0) = 3\nM(1, 1) = 4\nPRINT M(0, 0), M(0, 1), M(1, 0), M(1, 1)";
    assert_eq!(run_program(src), "1 2 3 4\n");
}

#[test]
fn test_tableau_2d_indices_variables() {
    let src = "DIM T(4, 4)\nI = 2\nJ = 3\nT(I, J) = 77\nPRINT T(2, 3)";
    assert_eq!(run_program(src), "77\n");
}

// --- Tableaux 3D ---

#[test]
fn test_tableau_3d() {
    let src = "DIM C(2, 2, 2)\nC(1, 1, 1) = 123\nPRINT C(1, 1, 1)";
    assert_eq!(run_program(src), "123\n");
}

// --- Tableaux avec boucles ---

#[test]
fn test_tableau_rempli_par_boucle() {
    let src = "DIM A(4)\nFOR I = 0 TO 4\nA(I) = I * I\nNEXT I\nPRINT A(0), A(1), A(2), A(3), A(4)";
    assert_eq!(run_program(src), "0 1 4 9 16\n");
}

#[test]
fn test_tableau_somme_par_boucle() {
    let src = "DIM A(4)\nFOR I = 0 TO 4\nA(I) = I + 1\nNEXT I\nS = 0\nFOR I = 0 TO 4\nS = S + A(I)\nNEXT I\nPRINT S";
    assert_eq!(run_program(src), "15\n");
}

#[test]
fn test_tableau_str_2d() {
    let src = "DIM G$(1, 1)\nG$(0, 0) = \"TL\"\nG$(0, 1) = \"TR\"\nG$(1, 0) = \"BL\"\nG$(1, 1) = \"BR\"\nPRINT G$(0, 0), G$(1, 1)";
    assert_eq!(run_program(src), "TL BR\n");
}
