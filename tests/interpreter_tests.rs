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
    assert_eq!(run_program("PRINT 10 MOD 3"), "1\n");
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

// --- SUB / END SUB / CALL ---

#[test]
fn test_sub_appel_simple() {
    let src = "CALL Bonjour\nGOTO fin\nSUB Bonjour\n    PRINT \"bonjour\"\nEND SUB\nfin:";
    assert_eq!(run_program(src), "bonjour\n");
}

#[test]
fn test_sub_param_entier() {
    let src = "CALL Double(5)\nGOTO fin\nSUB Double(N)\n    PRINT N * 2\nEND SUB\nfin:";
    assert_eq!(run_program(src), "10\n");
}

#[test]
fn test_sub_param_chaine() {
    let src = "CALL Saluer(\"Alice\")\nGOTO fin\nSUB Saluer(NOM$)\n    PRINT \"Bonjour \" + NOM$\nEND SUB\nfin:";
    assert_eq!(run_program(src), "Bonjour Alice\n");
}

#[test]
fn test_sub_plusieurs_params() {
    let src = "CALL Somme(3, 4)\nGOTO fin\nSUB Somme(A, B)\n    PRINT A + B\nEND SUB\nfin:";
    assert_eq!(run_program(src), "7\n");
}

#[test]
fn test_sub_appels_multiples() {
    let src = "CALL Inc(1)\nCALL Inc(2)\nCALL Inc(3)\nGOTO fin\nSUB Inc(N)\n    PRINT N\nEND SUB\nfin:";
    assert_eq!(run_program(src), "1\n2\n3\n");
}

#[test]
fn test_sub_variables_locales() {
    // X dans le SUB ne modifie pas X dans le programme principal
    let src = "X = 100\nCALL ModX\nPRINT X\nGOTO fin\nSUB ModX\n    X = 999\nEND SUB\nfin:";
    assert_eq!(run_program(src), "100\n");
}

#[test]
fn test_sub_retour_correct() {
    let src = "PRINT \"avant\"\nCALL Milieu\nPRINT \"apres\"\nGOTO fin\nSUB Milieu\n    PRINT \"dans sub\"\nEND SUB\nfin:";
    assert_eq!(run_program(src), "avant\ndans sub\napres\n");
}

#[test]
fn test_sub_avec_boucle() {
    let src = "CALL Compte(3)\nGOTO fin\nSUB Compte(N)\n    FOR I = 1 TO N\n        PRINT I\n    NEXT I\nEND SUB\nfin:";
    assert_eq!(run_program(src), "1\n2\n3\n");
}

#[test]
fn test_sub_imbriques() {
    // Un SUB appelle un autre SUB
    let src = "CALL A\nGOTO fin\nSUB A\n    PRINT \"debut A\"\n    CALL B\n    PRINT \"fin A\"\nEND SUB\nSUB B\n    PRINT \"dans B\"\nEND SUB\nfin:";
    assert_eq!(run_program(src), "debut A\ndans B\nfin A\n");
}

#[test]
fn test_sub_corps_saute_si_non_appele() {
    // Le corps du SUB n'est pas exécuté si on ne l'appelle pas
    let src = "PRINT \"main\"\nSUB NonAppele\n    PRINT \"ne doit pas apparaitre\"\nEND SUB\nPRINT \"fin\"";
    assert_eq!(run_program(src), "main\nfin\n");
}

// --- Fonctions built-in : entiers ---

#[test]
fn test_len() {
    assert_eq!(run_program(r#"PRINT LEN("Bonjour")"#), "7\n");
}

#[test]
fn test_len_vide() {
    assert_eq!(run_program(r#"PRINT LEN("")"#), "0\n");
}

#[test]
fn test_asc() {
    assert_eq!(run_program(r#"PRINT ASC("A")"#), "65\n");
}

#[test]
fn test_val_entier() {
    assert_eq!(run_program(r#"PRINT VAL("42")"#), "42\n");
}

#[test]
fn test_val_negatif() {
    assert_eq!(run_program(r#"PRINT VAL("-7")"#), "-7\n");
}

#[test]
fn test_val_invalide() {
    assert_eq!(run_program(r#"PRINT VAL("abc")"#), "0\n");
}

#[test]
fn test_instr_trouve() {
    assert_eq!(run_program(r#"PRINT INSTR("Bonjour", "jour")"#), "4\n");
}

#[test]
fn test_instr_non_trouve() {
    assert_eq!(run_program(r#"PRINT INSTR("Bonjour", "xyz")"#), "0\n");
}

#[test]
fn test_instr_avec_start() {
    // Cherche "a" a partir de la position 2 dans "abcabc" -> trouve le 2e "a" en position 4
    assert_eq!(run_program(r#"PRINT INSTR(2, "abcabc", "a")"#), "4\n");
}

#[test]
fn test_abs_positif() {
    assert_eq!(run_program("PRINT ABS(5)"), "5\n");
}

#[test]
fn test_abs_negatif() {
    assert_eq!(run_program("PRINT ABS(-5)"), "5\n");
}

#[test]
fn test_sgn_positif() {
    assert_eq!(run_program("PRINT SGN(10)"), "1\n");
}

#[test]
fn test_sgn_negatif() {
    assert_eq!(run_program("PRINT SGN(-3)"), "-1\n");
}

#[test]
fn test_sgn_zero() {
    assert_eq!(run_program("PRINT SGN(0)"), "0\n");
}

#[test]
fn test_sqr() {
    assert_eq!(run_program("PRINT SQR(16)"), "4\n");
}

// --- Fonctions built-in : chaînes ---

#[test]
fn test_str_dollar() {
    assert_eq!(run_program("PRINT STR$(42)"), "42\n");
}

#[test]
fn test_chr_dollar() {
    assert_eq!(run_program("PRINT CHR$(65)"), "A\n");
}

#[test]
fn test_space_dollar() {
    assert_eq!(run_program(r#"PRINT "|" + SPACE$(3) + "|""#), "|   |\n");
}

#[test]
fn test_left_dollar() {
    assert_eq!(run_program(r#"PRINT LEFT$("Bonjour", 3)"#), "Bon\n");
}

#[test]
fn test_right_dollar() {
    assert_eq!(run_program(r#"PRINT RIGHT$("Bonjour", 4)"#), "jour\n");
}

#[test]
fn test_mid_dollar_deux_args() {
    assert_eq!(run_program(r#"PRINT MID$("Bonjour", 4)"#), "jour\n");
}

#[test]
fn test_mid_dollar_trois_args() {
    assert_eq!(run_program(r#"PRINT MID$("Bonjour", 2, 3)"#), "onj\n");
}

#[test]
fn test_ucase_dollar() {
    assert_eq!(run_program(r#"PRINT UCASE$("hello")"#), "HELLO\n");
}

#[test]
fn test_lcase_dollar() {
    assert_eq!(run_program(r#"PRINT LCASE$("WORLD")"#), "world\n");
}

#[test]
fn test_ltrim_dollar() {
    assert_eq!(run_program(r#"PRINT LTRIM$("  bonjour")"#), "bonjour\n");
}

#[test]
fn test_rtrim_dollar() {
    assert_eq!(run_program(r#"PRINT RTRIM$("bonjour  ")"#), "bonjour\n");
}

#[test]
fn test_fonctions_combinees() {
    // STR$ + LEN
    let src = r#"N = 12345
PRINT LEN(STR$(N))"#;
    assert_eq!(run_program(src), "5\n");
}

#[test]
fn test_str_dollar_dans_expression() {
    let src = r#"X = 42
PRINT "Valeur : " + STR$(X)"#;
    assert_eq!(run_program(src), "Valeur : 42\n");
}

// --- Nombres flottants ---

#[test]
fn test_float_literal() {
    assert_eq!(run_program("PRINT 3.14"), "3.14\n");
}

#[test]
fn test_float_zero_frac() {
    // Un flottant sans partie décimale doit s'afficher comme entier
    assert_eq!(run_program("PRINT 4.0"), "4\n");
}

#[test]
fn test_float_addition() {
    assert_eq!(run_program("PRINT 1.5 + 2.5"), "4\n");
}

#[test]
fn test_float_soustraction() {
    assert_eq!(run_program("PRINT 5.0 - 1.25"), "3.75\n");
}

#[test]
fn test_float_multiplication() {
    assert_eq!(run_program("PRINT 2.5 * 4.0"), "10\n");
}

#[test]
fn test_float_division() {
    // flottant / flottant → division réelle
    assert_eq!(run_program("PRINT 10.0 / 4.0"), "2.5\n");
}

#[test]
fn test_int_division_reste_entier() {
    // entier / entier → division entière (rétrocompatibilité)
    assert_eq!(run_program("PRINT 10 / 3"), "3\n");
}

#[test]
fn test_mixed_int_float_addition() {
    // int + float → float
    assert_eq!(run_program("PRINT 1 + 0.5"), "1.5\n");
}

#[test]
fn test_mixed_float_int_division() {
    // float / int → division réelle
    assert_eq!(run_program("PRINT 10.0 / 3"), "3.3333333\n");
}

#[test]
fn test_float_variable() {
    assert_eq!(run_program("X = 3.14\nPRINT X"), "3.14\n");
}

#[test]
fn test_float_variable_arithmetique() {
    assert_eq!(run_program("X = 1.5\nY = 2.5\nPRINT X + Y"), "4\n");
}

#[test]
fn test_float_negatif() {
    assert_eq!(run_program("PRINT -3.14"), "-3.14\n");
}

#[test]
fn test_float_comparaison_vraie() {
    assert_eq!(run_program("PRINT 3.14 < 4.0"), "-1\n");
}

#[test]
fn test_float_comparaison_fausse() {
    assert_eq!(run_program("PRINT 3.14 > 4.0"), "0\n");
}

#[test]
fn test_sqr_float() {
    // SQR retourne toujours un flottant (√2 arrondi à 7 décimales)
    assert_eq!(run_program("PRINT SQR(2.0)"), "1.4142136\n");
}

#[test]
fn test_abs_float() {
    assert_eq!(run_program("PRINT ABS(-3.14)"), "3.14\n");
}

#[test]
fn test_abs_float_positif() {
    assert_eq!(run_program("PRINT ABS(2.71)"), "2.71\n");
}

#[test]
fn test_sgn_float_negatif() {
    assert_eq!(run_program("PRINT SGN(-2.5)"), "-1\n");
}

#[test]
fn test_sgn_float_positif() {
    assert_eq!(run_program("PRINT SGN(0.1)"), "1\n");
}

#[test]
fn test_val_float() {
    assert_eq!(run_program(r#"PRINT VAL("2.5")"#), "2.5\n");
}

#[test]
fn test_str_dollar_float() {
    assert_eq!(run_program("PRINT STR$(3.14)"), "3.14\n");
}

#[test]
fn test_int_builtin() {
    assert_eq!(run_program("PRINT INT(3.9)"), "3\n");
}

#[test]
fn test_int_builtin_negatif() {
    // INT arrondit vers le bas (floor)
    assert_eq!(run_program("PRINT INT(-3.1)"), "-4\n");
}

#[test]
fn test_fix_builtin() {
    // FIX tronque vers zéro
    assert_eq!(run_program("PRINT FIX(-3.9)"), "-3\n");
}

#[test]
fn test_cint_builtin() {
    assert_eq!(run_program("PRINT CINT(3.5)"), "4\n");
}

#[test]
fn test_csng_builtin() {
    // CSNG convertit en flottant
    assert_eq!(run_program("PRINT CSNG(5)"), "5\n");
}

#[test]
fn test_for_float_step() {
    // FOR avec pas flottant
    let src = "FOR X = 1.0 TO 2.0 STEP 0.5\nPRINT X\nNEXT X";
    assert_eq!(run_program(src), "1\n1.5\n2\n");
}

#[test]
fn test_for_float_from_to() {
    let src = "FOR X = 0.0 TO 1.0 STEP 0.25\nPRINT X\nNEXT X";
    assert_eq!(run_program(src), "0\n0.25\n0.5\n0.75\n1\n");
}

// --- STRING$ ---

#[test]
fn test_string_dollar_chaine() {
    assert_eq!(run_program(r#"PRINT STRING$(5, "A")"#), "AAAAA\n");
}

#[test]
fn test_string_dollar_ascii() {
    // 65 = 'A'
    assert_eq!(run_program("PRINT STRING$(3, 65)"), "AAA\n");
}

#[test]
fn test_string_dollar_zero() {
    assert_eq!(run_program(r#"PRINT STRING$(0, "X")"#), "\n");
}

#[test]
fn test_string_dollar_tiret() {
    assert_eq!(run_program(r#"PRINT STRING$(4, "-")"#), "----\n");
}

#[test]
fn test_string_dollar_premier_char() {
    // On utilise le premier caractère de la chaîne
    assert_eq!(run_program(r#"PRINT STRING$(3, "ABC")"#), "AAA\n");
}

// --- RND et RANDOMIZE ---

#[test]
fn test_rnd_positif() {
    // RND est toujours >= 0
    let src = "RANDOMIZE 1\nX = RND\nIF X >= 0 THEN PRINT \"ok\"";
    assert_eq!(run_program(src), "ok\n");
}

#[test]
fn test_rnd_inferieur_a_un() {
    // RND est toujours < 1
    let src = "RANDOMIZE 1\nX = RND\nIF X < 1 THEN PRINT \"ok\"";
    assert_eq!(run_program(src), "ok\n");
}

#[test]
fn test_rnd_deux_appels_differents() {
    // Deux appels successifs produisent des valeurs différentes
    let src = "RANDOMIZE 42\nA = RND\nB = RND\nIF A <> B THEN PRINT \"different\"";
    assert_eq!(run_program(src), "different\n");
}

#[test]
fn test_rnd_deterministe() {
    // Avec la même graine, RND produit la même séquence
    let src1 = "RANDOMIZE 7\nX = RND\nPRINT X";
    let src2 = "RANDOMIZE 7\nX = RND\nPRINT X";
    assert_eq!(run_program(src1), run_program(src2));
}

#[test]
fn test_rnd_graines_differentes() {
    // Deux graines différentes produisent des séquences différentes
    let src1 = "RANDOMIZE 1\nPRINT RND";
    let src2 = "RANDOMIZE 2\nPRINT RND";
    assert_ne!(run_program(src1), run_program(src2));
}

// --- TIMER ---

#[test]
fn test_timer_positif() {
    // TIMER retourne un nombre de secondes positif
    let src = "IF TIMER > 0 THEN PRINT \"ok\"";
    assert_eq!(run_program(src), "ok\n");
}

// --- SLEEP ---

#[test]
fn test_sleep_zero() {
    // SLEEP 0 ne dort pas et continue normalement
    assert_eq!(run_program("SLEEP 0\nPRINT \"ok\""), "ok\n");
}

#[test]
fn test_sleep_puis_print() {
    assert_eq!(run_program("SLEEP 0\nPRINT \"apres\""), "apres\n");
}

// --- IF multiligne ---

#[test]
fn test_if_multiline_true() {
    let src = "X = 5\nIF X > 3 THEN\n    PRINT \"grand\"\nEND IF";
    assert_eq!(run_program(src), "grand\n");
}

#[test]
fn test_if_multiline_false() {
    let src = "X = 1\nIF X > 3 THEN\n    PRINT \"grand\"\nEND IF\nPRINT \"fin\"";
    assert_eq!(run_program(src), "fin\n");
}

#[test]
fn test_if_multiline_else_true() {
    let src = "X = 5\nIF X > 3 THEN\n    PRINT \"A\"\nELSE\n    PRINT \"B\"\nEND IF";
    assert_eq!(run_program(src), "A\n");
}

#[test]
fn test_if_multiline_else_false() {
    let src = "X = 1\nIF X > 3 THEN\n    PRINT \"A\"\nELSE\n    PRINT \"B\"\nEND IF";
    assert_eq!(run_program(src), "B\n");
}

#[test]
fn test_if_multiline_elseif_first_branch() {
    let src = "X = 1\nIF X = 1 THEN\n    PRINT \"un\"\nELSEIF X = 2 THEN\n    PRINT \"deux\"\nELSE\n    PRINT \"autre\"\nEND IF";
    assert_eq!(run_program(src), "un\n");
}

#[test]
fn test_if_multiline_elseif_second_branch() {
    let src = "X = 2\nIF X = 1 THEN\n    PRINT \"un\"\nELSEIF X = 2 THEN\n    PRINT \"deux\"\nELSE\n    PRINT \"autre\"\nEND IF";
    assert_eq!(run_program(src), "deux\n");
}

#[test]
fn test_if_multiline_elseif_else_branch() {
    let src = "X = 9\nIF X = 1 THEN\n    PRINT \"un\"\nELSEIF X = 2 THEN\n    PRINT \"deux\"\nELSE\n    PRINT \"autre\"\nEND IF";
    assert_eq!(run_program(src), "autre\n");
}

#[test]
fn test_if_multiline_nested() {
    let src = "A = 1\nB = 2\nIF A = 1 THEN\n    IF B = 2 THEN\n        PRINT \"ok\"\n    END IF\nEND IF";
    assert_eq!(run_program(src), "ok\n");
}

#[test]
fn test_if_multiline_nested_inner_false() {
    let src = "A = 1\nB = 0\nIF A = 1 THEN\n    IF B = 2 THEN\n        PRINT \"non\"\n    END IF\n    PRINT \"oui\"\nEND IF";
    assert_eq!(run_program(src), "oui\n");
}

#[test]
fn test_if_multiline_multiple_stmts_in_body() {
    let src = "X = 5\nIF X > 0 THEN\n    PRINT \"a\"\n    PRINT \"b\"\nEND IF";
    assert_eq!(run_program(src), "a\nb\n");
}

// --- DO/LOOP ---

#[test]
fn test_do_while_loop() {
    let src = "I = 0\nDO WHILE I < 3\n    PRINT I\n    I = I + 1\nLOOP";
    assert_eq!(run_program(src), "0\n1\n2\n");
}

#[test]
fn test_do_while_zero_iterations() {
    let src = "I = 5\nDO WHILE I < 3\n    PRINT I\n    I = I + 1\nLOOP\nPRINT \"fin\"";
    assert_eq!(run_program(src), "fin\n");
}

#[test]
fn test_do_until_loop() {
    let src = "I = 0\nDO UNTIL I >= 3\n    PRINT I\n    I = I + 1\nLOOP";
    assert_eq!(run_program(src), "0\n1\n2\n");
}

#[test]
fn test_do_until_zero_iterations() {
    // DO UNTIL cond : si cond est déjà vraie, zéro itération
    let src = "I = 1\nDO UNTIL I < 3\n    PRINT I\nLOOP\nPRINT \"fin\"";
    assert_eq!(run_program(src), "fin\n");
}

#[test]
fn test_do_loop_while_post() {
    // Post-condition : au moins une itération même si fausse dès le départ
    let src = "I = 5\nDO\n    PRINT I\n    I = I + 1\nLOOP WHILE I < 3\nPRINT \"fin\"";
    assert_eq!(run_program(src), "5\nfin\n");
}

#[test]
fn test_do_loop_until_post() {
    let src = "I = 0\nDO\n    PRINT I\n    I = I + 1\nLOOP UNTIL I >= 3";
    assert_eq!(run_program(src), "0\n1\n2\n");
}

#[test]
fn test_do_loop_infinite_break_via_goto() {
    // DO...LOOP sans condition — on sort via GOTO
    let src = "I = 0\nDO\n    PRINT I\n    I = I + 1\n    IF I >= 3 THEN GOTO fin\nLOOP\nfin:\nPRINT \"stop\"";
    assert_eq!(run_program(src), "0\n1\n2\nstop\n");
}

#[test]
fn test_do_while_several_iterations() {
    let src = "S = 0\nI = 1\nDO WHILE I <= 5\n    S = S + I\n    I = I + 1\nLOOP\nPRINT S";
    assert_eq!(run_program(src), "15\n");
}

// --- DECLARE SUB ---

#[test]
fn test_declare_sub_noop() {
    // DECLARE SUB est un no-op : le programme doit quand même appeler le SUB
    let src = "DECLARE SUB Salut()\nCALL Salut\nSUB Salut()\n    PRINT \"bonjour\"\nEND SUB";
    assert_eq!(run_program(src), "bonjour\n");
}

#[test]
fn test_declare_sub_with_params() {
    let src = "DECLARE SUB Double(N)\nCALL Double(7)\nSUB Double(N)\n    PRINT N * 2\nEND SUB";
    assert_eq!(run_program(src), "14\n");
}

// --- Console (no-ops hors terminal réel) ---

#[test]
fn test_screen_noop() {
    assert_eq!(run_program("SCREEN 0\nPRINT \"ok\""), "ok\n");
}

#[test]
fn test_width_noop() {
    assert_eq!(run_program("WIDTH 80\nPRINT \"ok\""), "ok\n");
}

#[test]
fn test_color_noop() {
    // COLOR est un no-op hors terminal ; le programme continue normalement
    assert_eq!(run_program("COLOR 14, 0\nPRINT \"jaune\""), "jaune\n");
}

#[test]
fn test_locate_noop() {
    assert_eq!(run_program("LOCATE 5, 10\nPRINT \"ici\""), "ici\n");
}

#[test]
fn test_cls_noop() {
    assert_eq!(run_program("CLS\nPRINT \"propre\""), "propre\n");
}

#[test]
fn test_beep_does_not_crash() {
    // BEEP émet BEL ; on vérifie que le programme continue sans planter
    let out = run_program("BEEP\nPRINT \"apres\"");
    assert!(out.contains("apres"));
}

#[test]
fn test_inkey_empty_without_console() {
    // Hors mode console, INKEY$ retourne toujours "" → LEN = 0
    let src = "K$ = INKEY$\nPRINT LEN(K$)";
    assert_eq!(run_program(src), "0\n");
}

#[test]
fn test_inkey_print_empty() {
    // INKEY$ directement dans PRINT : retourne une chaîne vide
    assert_eq!(run_program("PRINT INKEY$"), "\n");
}

#[test]
fn test_csrlin_without_console() {
    // Hors terminal, CSRLIN retourne 1
    assert_eq!(run_program("PRINT CSRLIN"), "1\n");
}

#[test]
fn test_pos_without_console() {
    // Hors terminal, POS(0) retourne 1
    assert_eq!(run_program("PRINT POS(0)"), "1\n");
}

#[test]
fn test_color_in_loop() {
    // COLOR dans une boucle : pas de crash, le contenu s'affiche bien
    let src = "FOR I = 1 TO 3\n    COLOR I\n    PRINT I\nNEXT I";
    assert_eq!(run_program(src), "1\n2\n3\n");
}

#[test]
fn test_locate_color_print() {
    // Séquence typique : positionnement + couleur + affichage
    let src = "LOCATE 1, 1\nCOLOR 15\nPRINT \"test\"";
    assert_eq!(run_program(src), "test\n");
}

// --- Comparaison de chaînes ---

#[test]
fn test_str_cmp_egal_vrai() {
    let src = "A$ = \"hello\"\nIF A$ = \"hello\" THEN PRINT \"ok\"";
    assert_eq!(run_program(src), "ok\n");
}

#[test]
fn test_str_cmp_egal_faux() {
    let src = "A$ = \"hello\"\nIF A$ = \"world\" THEN PRINT \"non\" ELSE PRINT \"ok\"";
    assert_eq!(run_program(src), "ok\n");
}

#[test]
fn test_str_cmp_different() {
    let src = "A$ = \"abc\"\nIF A$ <> \"def\" THEN PRINT \"ok\"";
    assert_eq!(run_program(src), "ok\n");
}

#[test]
fn test_str_cmp_inferieur() {
    let src = "IF \"abc\" < \"abd\" THEN PRINT \"ok\"";
    assert_eq!(run_program(src), "ok\n");
}

#[test]
fn test_str_cmp_superieur() {
    let src = "IF \"z\" > \"a\" THEN PRINT \"ok\"";
    assert_eq!(run_program(src), "ok\n");
}

#[test]
fn test_str_cmp_inferieur_egal() {
    let src = "IF \"abc\" <= \"abc\" THEN PRINT \"ok\"";
    assert_eq!(run_program(src), "ok\n");
}

#[test]
fn test_str_cmp_superieur_egal() {
    let src = "IF \"b\" >= \"a\" THEN PRINT \"ok\"";
    assert_eq!(run_program(src), "ok\n");
}

#[test]
fn test_str_cmp_vide() {
    let src = "A$ = \"\"\nIF A$ = \"\" THEN PRINT \"vide\"";
    assert_eq!(run_program(src), "vide\n");
}

#[test]
fn test_str_cmp_dans_boucle() {
    let src = "FOR I = 1 TO 3\n    S$ = STR$(I)\n    IF S$ = \"2\" THEN PRINT \"deux\"\nNEXT I";
    assert_eq!(run_program(src), "deux\n");
}

// --- Instructions multiples sur une ligne (:) ---

#[test]
fn test_multistatement_deux_affectations() {
    assert_eq!(run_program("A = 1 : B = 2 : PRINT A + B"), "3\n");
}

#[test]
fn test_multistatement_deux_prints() {
    assert_eq!(run_program("PRINT \"a\" : PRINT \"b\""), "a\nb\n");
}

#[test]
fn test_multistatement_avec_numero_de_ligne() {
    assert_eq!(run_program("10 A = 5 : PRINT A"), "5\n");
}

#[test]
fn test_multistatement_if_then_suivi() {
    // L'instruction après : est hors du IF (toujours exécutée)
    let src = "X = 0\nIF X > 0 THEN PRINT \"si\" : PRINT \"toujours\"";
    assert_eq!(run_program(src), "toujours\n");
}

#[test]
fn test_multistatement_for_sur_une_ligne() {
    // FOR, PRINT et NEXT sur la même ligne source
    let src = "FOR I = 1 TO 3 : PRINT I : NEXT I";
    assert_eq!(run_program(src), "1\n2\n3\n");
}

#[test]
fn test_multistatement_trois_instructions() {
    let src = "A = 10 : B = 20 : C = A + B : PRINT C";
    assert_eq!(run_program(src), "30\n");
}

// --- END ---

#[test]
fn test_end_arrete_execution() {
    let src = "PRINT \"avant\"\nEND\nPRINT \"apres\"";
    assert_eq!(run_program(src), "avant\n");
}

#[test]
fn test_end_dans_if() {
    let src = "X = 5\nIF X > 3 THEN END\nPRINT \"non\"";
    assert_eq!(run_program(src), "");
}

#[test]
fn test_end_apres_boucle() {
    let src = "FOR I = 1 TO 3\n    PRINT I\n    IF I = 2 THEN END\nNEXT I\nPRINT \"fin\"";
    assert_eq!(run_program(src), "1\n2\n");
}
