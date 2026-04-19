REM --- Fonctions built-in ---

PRINT "=== LEN, ASC, CHR$ ==="
A$ = "Bonjour"
PRINT "LEN(Bonjour) =", LEN(A$)
PRINT "ASC(A) =", ASC("A")
PRINT "CHR$(65) =", CHR$(65)
PRINT "CHR$(ASC(Z)) =", CHR$(ASC("Z"))

PRINT "=== STR$, VAL ==="
N = 42
PRINT "STR$(42) = |" + STR$(N) + "|"
PRINT "VAL(123) =", VAL("123")
PRINT "VAL(-7) =", VAL("-7")
PRINT "VAL(abc) =", VAL("abc")

PRINT "=== LEFT$, RIGHT$, MID$ ==="
S$ = "Bonjour le monde"
PRINT "LEFT$(S$, 7) =", LEFT$(S$, 7)
PRINT "RIGHT$(S$, 5) =", RIGHT$(S$, 5)
PRINT "MID$(S$, 9, 2) =", MID$(S$, 9, 2)
PRINT "MID$(S$, 9) =", MID$(S$, 9)

PRINT "=== UCASE$, LCASE$ ==="
PRINT UCASE$("bonjour")
PRINT LCASE$("MONDE")

PRINT "=== LTRIM$, RTRIM$ ==="
PRINT "|" + LTRIM$("  hello  ") + "|"
PRINT "|" + RTRIM$("  hello  ") + "|"

PRINT "=== SPACE$ ==="
PRINT "|" + SPACE$(5) + "|"

PRINT "=== INSTR ==="
PRINT "INSTR(Bonjour, jour) =", INSTR("Bonjour", "jour")
PRINT "INSTR(Bonjour, xyz) =", INSTR("Bonjour", "xyz")
PRINT "INSTR(2, abcabc, a) =", INSTR(2, "abcabc", "a")

PRINT "=== ABS, SGN, SQR ==="
PRINT "ABS(-42) =", ABS(-42)
PRINT "SGN(-5) =", SGN(-5)
PRINT "SGN(0) =", SGN(0)
PRINT "SGN(3) =", SGN(3)
PRINT "SQR(25) =", SQR(25)

PRINT "=== Combinaisons ==="
X = 12345
PRINT "Longueur de STR$(12345) =", LEN(STR$(X))
S$ = "Valeur : 99"
POS = INSTR(S$, ": ") + 2
PRINT "Nombre extrait =", VAL(MID$(S$, POS))
