REM --- Comparaison de chaînes, multi-instructions, END ---

PRINT "=== Comparaison de chaînes ==="
A$ = "Bonjour"
B$ = "Hello"

IF A$ = "Bonjour" THEN PRINT "A$ est Bonjour"
IF A$ <> B$ THEN PRINT "A$ et B$ sont différents"
IF "abc" < "abd" THEN PRINT "abc < abd : ok"
IF "z" > "a"    THEN PRINT "z > a : ok"

REM Tri de deux chaînes
X$ = "Mango" : Y$ = "Apple"
IF X$ > Y$ THEN PRINT Y$ + " vient avant " + X$

REM Comparaison dans une boucle
PRINT "Recherche de 'trois' :"
FOR I = 1 TO 5
    N$ = STR$(I)
    IF N$ = "3" THEN PRINT "trouvé :", I
NEXT I

PRINT ""
PRINT "=== Multi-instructions sur une ligne ==="

A = 0 : B = 0 : C = 0
A = 10 : B = 20 : C = A + B
PRINT "A =", A, "  B =", B, "  C =", C

FOR I = 1 TO 5 : PRINT I : NEXT I

PRINT "Compte :" : X = 1 : PRINT X : X = X + 1 : PRINT X : X = X + 1 : PRINT X

PRINT ""
PRINT "=== END ==="
PRINT "avant END"
END
PRINT "cette ligne ne s'affiche jamais"
