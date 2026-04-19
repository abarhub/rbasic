REM --- Expressions arithmetiques ---
PRINT "=== Arithmetique ==="
PRINT "3 + 4 =", 3 + 4
PRINT "10 - 3 =", 10 - 3
PRINT "6 * 7 =", 6 * 7
PRINT "17 / 5 =", 17 / 5
PRINT "17 MOD 5 =", 17 MOD 5
PRINT "2 + 3 * 4 =", 2 + 3 * 4
PRINT "(2 + 3) * 4 =", (2 + 3) * 4

REM --- Unaire ---
PRINT "=== Unaire ==="
X = 10
PRINT "-X =", -X
PRINT "--X =", --X
PRINT "+X =", +X
Y = -5
PRINT "Y =", Y
PRINT "-Y =", -Y

REM --- Comparaisons ---
PRINT "=== Comparaisons (vrai=-1, faux=0) ==="
PRINT "5 = 5 :", 5 = 5
PRINT "5 = 6 :", 5 = 6
PRINT "3 < 7 :", 3 < 7
PRINT "7 > 3 :", 7 > 3
PRINT "5 <= 5 :", 5 <= 5
PRINT "5 >= 6 :", 5 >= 6
PRINT "4 <> 5 :", 4 <> 5

REM --- NOT ---
PRINT "=== NOT ==="
PRINT "NOT 0 =", NOT 0
PRINT "NOT -1 =", NOT -1
PRINT "NOT 3 < 7 =", NOT 3 < 7
PRINT "NOT 7 < 3 =", NOT 7 < 3

REM --- AND / OR / XOR ---
PRINT "=== AND / OR / XOR ==="
PRINT "6 AND 3 =", 6 AND 3
PRINT "6 OR 3 =", 6 OR 3
PRINT "6 XOR 3 =", 6 XOR 3
PRINT "(3>1) AND (5>2) =", (3 > 1) AND (5 > 2)
PRINT "(3>1) AND (5>9) =", (3 > 1) AND (5 > 9)
PRINT "(3>1) OR  (5>9) =", (3 > 1) OR (5 > 9)
PRINT "(3>9) OR  (5>9) =", (3 > 9) OR (5 > 9)

REM --- Concatenation de chaines ---
PRINT "=== Chaines ==="
DIM PRENOM$(10)
DIM NOM$(15)
PRENOM$ = "Alice"
NOM$ = "Dupont"
PRINT "Bonjour " + PRENOM$ + " " + NOM$
PRINT "Longueur tronquee : |" + PRENOM$ + "|"
DIM COURT$(3)
COURT$ = "Bonjour"
PRINT "Tronque a 3 : |" + COURT$ + "|"
