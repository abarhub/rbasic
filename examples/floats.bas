REM --- Nombres flottants ---

PRINT "=== Litteraux flottants ==="
PRINT 3.14
PRINT 2.5
PRINT 0.001

PRINT "=== Arithmetique flottante ==="
PRINT 1.5 + 2.5
PRINT 5.0 - 1.25
PRINT 2.5 * 4.0
PRINT 10.0 / 4.0

PRINT "=== Division entiere vs flottante ==="
PRINT "10 / 3 =", 10 / 3
PRINT "10.0 / 3 =", 10.0 / 3

PRINT "=== Mixte entier + flottant ==="
PRINT 1 + 0.5
PRINT 3 * 1.5

PRINT "=== Variables flottantes ==="
PI = 3.14159
R = 5.0
PRINT "Pi =", PI
PRINT "R =", R
PRINT "Perimetre =", 2.0 * PI * R

PRINT "=== Fonctions ==="
PRINT "SQR(2) =", SQR(2.0)         REM affiche 1.4142136
PRINT "SQR(9) =", SQR(9.0)
PRINT "ABS(-3.14) =", ABS(-3.14)
PRINT "SGN(-2.5) =", SGN(-2.5)
PRINT "SGN(1.5) =", SGN(1.5)
PRINT "INT(3.9) =", INT(3.9)
PRINT "INT(-3.1) =", INT(-3.1)
PRINT "FIX(-3.9) =", FIX(-3.9)
PRINT "CINT(3.5) =", CINT(3.5)
PRINT "CSNG(7) =", CSNG(7)

PRINT "=== STR$ et VAL ==="
PRINT "STR$(3.14) =", STR$(3.14)
PRINT "VAL(2.71) =", VAL("2.71")

PRINT "=== Comparaisons ==="
X = 3.14
IF X > 3.0 THEN PRINT "X > 3.0 : vrai"
IF X < 4.0 THEN PRINT "X < 4.0 : vrai"
IF X = 3.14 THEN PRINT "X = 3.14 : vrai"

PRINT "=== FOR avec pas flottant ==="
FOR T = 0.0 TO 1.0 STEP 0.25
    PRINT T
NEXT T

PRINT "=== Accumulation flottante ==="
S = 0.0
FOR I = 1 TO 5
    S = S + 0.1
NEXT I
PRINT "Somme 5 * 0.1 =", S
